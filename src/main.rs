mod cameras;
mod hitables;
mod materials;
mod structs;

use cameras::*;
use hitables::*;
use materials::*;

use rand::Rng;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

/// Generate a random scene with 484 little random spheres,
/// 3 bigger spheres in center, and a spheric ground.
fn random_scene() -> HitableList {
    let mut world = HitableList::new();

    let mut texture_manager = ResourceManager::new();

    let odd_color = TextureConfig {
        kind: TextureKind::Constant(Color::new(51, 77, 26)),
    };
    let even_color = TextureConfig {
        kind: TextureKind::Constant(Color::new(230, 230, 230)),
    };
    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Checker(&odd_color, &even_color),
            }),
        }),
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::thread_rng().gen::<f64>();
            let center = Vec3::new(
                f64::from(a) + 0.9 * rand::thread_rng().gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rand::thread_rng().gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let texture = texture_manager.get_resource(&TextureConfig {
                        kind: TextureKind::Constant(Color::new(
                            (albedo.x * 255.99) as u8,
                            (albedo.y * 255.99) as u8,
                            (albedo.z * 255.99) as u8,
                        )),
                    });
                    world.push(Arc::new(MovingSphere {
                        center0: center,
                        center1: center
                            + Vec3::new(0.0, 0.5 * rand::thread_rng().gen::<f64>(), 0.0),
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Arc::new(Lambertian { albedo: texture }),
                    }));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal::new(albedo, fuzz)),
                    }));
                } else {
                    // Glass
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric { ref_idx: 1.5 }),
                    }));
                }
            }
        }
    }

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Dielectric { ref_idx: 1.5 }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Arc::new(SolidColor::new(Vec3::new(0.4, 0.2, 0.1))),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    }));

    world
}

fn two_spheres() -> HitableList {
    let mut world = HitableList::new();

    let mut texture_manager = ResourceManager::new();

    let odd_color = TextureConfig {
        kind: TextureKind::Constant(Color::new(51, 77, 26)),
    };
    let even_color = TextureConfig {
        kind: TextureKind::Constant(Color::new(230, 230, 230)),
    };

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Checker(&odd_color, &even_color),
            }),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Checker(&odd_color, &even_color),
            }),
        }),
    }));

    world
}

fn two_perlin_spheres() -> HitableList {
    let mut world = HitableList::new();

    let mut texture_manager = ResourceManager::new();

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Perlin(256),
            }),
        }),
    }));

    world.push(Arc::new(Sphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Arc::new(Lambertian {
            albedo: texture_manager.get_resource(&TextureConfig {
                kind: TextureKind::Perlin(256),
            }),
        }),
    }));

    world
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
                    return attenuation * color(&scattered, world, depth - 1);
                }
            }
            Vec3::default()
        }
        None => {
            // sky color
            let unit_direction = unit_vector(ray.direction());
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn gamma(color: Vec3) -> Vec3 {
    Vec3 {
        x: color.x.sqrt(),
        y: color.y.sqrt(),
        z: color.z.sqrt(),
    }
}

/// Dispatch ray-tracing algorithm on several threads to create an image of the current scene
fn render(
    image_width: u32,
    image_height: u32,
    sample_per_pixel: u32,
    world: HitableList,
    camera: Arc<dyn Camera>,
) -> Image {
    let max_depth: u32 = 10;
    let thread_count = 4;

    let mut handles = vec![];
    let lines: Arc<Vec<u32>> = Arc::new((0..image_height).rev().collect());

    let lines_per_thread = lines.len() / thread_count;
    let tougher_threads = lines.len() % thread_count;
    let mut offset = 0;

    let (tx, rx) = mpsc::channel();

    // Compute a BVH of the scene
    let bvh = BVHNode::new(&world, 0.0, 0.1);

    for id in 0..thread_count {
        let chunksize = if id < tougher_threads {
            lines_per_thread + 1
        } else {
            lines_per_thread
        };

        let camera = camera.clone();
        let lines = lines.clone();
        let tx = tx.clone();
        let bvh = bvh.clone();

        handles.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let mut pixels = vec![];
            let end = offset + chunksize;
            for j in &lines[offset..end] {
                for i in 0..image_width {
                    let mut col = Vec3::default();
                    for _s in 0..sample_per_pixel {
                        let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(image_width);
                        let v = (f64::from(*j) + rng.gen::<f64>()) / f64::from(image_height);
                        let r = camera.get_ray(u, v);

                        let _p = r.point_at(2.0);
                        col += color(&r, &bvh, max_depth);
                    }

                    col /= f64::from(sample_per_pixel);
                    let col = gamma(col);

                    pixels.push(col.x);
                    pixels.push(col.y);
                    pixels.push(col.z);
                }
                tx.send(1).unwrap();
            }
            pixels
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

    // Create a scene
    let scene = 2;
    let (world, lookfrom, lookat, vfov, aperture) = match scene {
        0 => (
            random_scene(),
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::default(),
            20.0,
            0.1,
        ),
        1 => (
            two_spheres(),
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::default(),
            20.0,
            0.0,
        ),
        2 | _ => (
            two_perlin_spheres(),
            Vec3::new(13.0, 2.0, 3.0),
            Vec3::default(),
            20.0,
            0.0,
        ),
    };

    let aspect_ratio = f64::from(image_width) / f64::from(image_height);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let cam = ThinLensCamera::new_look_at(
        lookfrom,
        lookat,
        up,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let before = Instant::now();

    let image = render(
        image_width,
        image_height,
        sample_per_pixel,
        world,
        Arc::from(cam),
    );

    eprintln!("Done in {}secs!           ", before.elapsed().as_secs());

    println!("{}", image.encode(ImageFormat::PPM));
}
