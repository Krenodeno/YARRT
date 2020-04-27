mod structs;
mod hitables;
mod cameras;
mod materials;

use structs::*;
use hitables::*;
use cameras::*;
use materials::*;

use rand::Rng;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::{Instant};
use std::thread;

/// Generate a random scene with 484 little random spheres,
/// 3 bigger spheres in center, and a spheric ground.
fn random_scene() -> HitableList {
    let mut world = HitableList::new();

    world.push(Box::new(Sphere{
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian{albedo: Vec3::new(0.5, 0.5, 0.5)}),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::thread_rng().gen::<f64>();
            let center = Vec3::new(a as f64 + 0.9 * rand::thread_rng().gen::<f64>(), 0.2, b as f64 + 0.9 * rand::thread_rng().gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    world.push(Box::new(MovingSphere{
                        center0: center,
                        center1: center + Vec3::new(0.0, 0.5 * rand::thread_rng().gen::<f64>(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian{albedo}),
                    }));
                }
                else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);
                    world.push(Box::new(Sphere{
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal::new(albedo, fuzz)),
                    }));
                }
                else {
                    // Glass
                    world.push(Box::new(Sphere{
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric{ref_idx: 1.5}),
                    }));
                }
            }
        }
    }

    world.push(Box::new(Sphere{
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric{ref_idx: 1.5}),
    }));

    world.push(Box::new(Sphere{
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian{albedo: Vec3::new(0.4, 0.2, 0.1)}),
    }));

    world.push(Box::new(Sphere{
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    }));

    return world;
}

/// Compute the color of the current ray
/// in the world of hitables.
/// This function run recursively until maximum number of recursions
/// (depth parameter) is reached or no hitable is hit.
fn color(ray: &Ray, world: &dyn Hitable, depth: u32) -> Vec3 {
    let record = world.hit(&ray, 0.001, std::f64::MAX);
    match record {
        Some(rec) => {
            let res = rec.material.scatter(&ray, &rec);
            if let Some((attenuation, scattered)) = res {
                if depth != 0 {
                    return attenuation * color(&scattered, world, depth-1);
                }
            }
            return Vec3::default();
        },
        None => {
            // sky color
            let unit_direction = unit_vector(ray.direction());
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn gamma(color: Vec3) -> Vec3 {
    Vec3{
        x: color.x.sqrt(),
        y: color.y.sqrt(),
        z: color.z.sqrt(),
    }
}

/// Dispatch ray-tracing algorithm on several threads to create an image of the current scene
fn render(image_width: u32, image_height: u32, sample_per_pixel: u32, world: Arc<dyn Hitable>, camera: Arc<dyn Camera>) -> Image {
    let max_depth: u32 = 50;
    let thread_count = 4;

    let mut handles = vec![];
    let lines: Arc<Vec<u32>> = Arc::new((0..image_height).rev().collect());

    let lines_per_thread = lines.len() / thread_count;
    let tougher_threads = lines.len() % thread_count;
    let mut offset = 0;

    let (tx, rx) = mpsc::channel();

    for id in 0..thread_count {

        let chunksize =
            if id < tougher_threads {
                lines_per_thread + 1
            } else {
                lines_per_thread
            };

        let world = world.clone();
        let camera = camera.clone();
        let lines = lines.clone();
        let tx = tx.clone();

        handles.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut pixels = vec![];
            let end = offset + chunksize;
            for j in &lines[offset..end] {
                for i in 0..image_width {
                    let mut col = Vec3::default();
                    for _s in 0..sample_per_pixel {
                        let u = (i as f64 + rng.gen::<f64>()) / image_width as f64;
                        let v = (*j as f64 + rng.gen::<f64>()) / image_height as f64;
                        let r = camera.get_ray(u, v);

                        let _p = r.point_at(2.0);
                        col += color(&r, world.as_ref(), max_depth);
                    }

                    col /= sample_per_pixel as f64;
                    let col = gamma(col);

                    pixels.push(col.x);
                    pixels.push(col.y);
                    pixels.push(col.z);
                }
                tx.send(1).unwrap();
            }
            return pixels;
        }));
        offset += chunksize;
    }

    // print status
    // lines is a reversed list of number, so it work well to count
    for i in lines.iter() {
        rx.recv().unwrap();
        eprint!("Scanlines remaining: {}    \r", i);
    }

    let mut buffer = Vec::new();

    for handle in handles {
        let mut pixels = handle.join().unwrap();
        buffer.append(&mut pixels);
    }

    Image::from(image_width, image_height, PixelFormat::RGBU8, &buffer)
}

fn main() {
    let image_width: u32 = 400;
    let image_height: u32 = 225;
    let sample_per_pixel: u32 = 100;

    let aspect_ratio = image_width as f64 / image_height as f64;
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::default();
    let up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = ThinLensCamera::new_look_at(lookfrom, lookat, up, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

    // Create a scene full of random spheres
    let world = random_scene();

    let before = Instant::now();

    let image = render(image_width, image_height, sample_per_pixel, Arc::from(world), Arc::from(cam));

    eprintln!("Done in {}secs!           ", before.elapsed().as_secs());

    println!("{}", image.encode(ImageFormat::PPM));
}
