mod structs;
mod hitables;
mod camera;
mod materials;

use structs::*;
use hitables::*;
use camera::Camera;
use materials::*;

use rand::Rng;
use std::sync::Arc;

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
            return Vec3::new(0.0, 0.0, 0.0);
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

fn main() {
    let image_width: u16 = 200;
    let image_height: u16 = 100;
    let sample_per_pixel: u16 = 100;
    let max_depth: u32 = 50;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let cam = Camera::default();
    let mut rng = rand::thread_rng();

    // Create spheres and add them to the list
    let mut spheres = HitableList::new();
    spheres.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Lambertian{albedo: Vec3::new(0.1, 0.2, 0.5)})
    }));
    spheres.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Arc::new(Lambertian{albedo: Vec3::new(0.8, 0.8, 0.0)})
    }));
    spheres.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0))
    }));
    spheres.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::new(Dielectric{ref_idx: 1.5})
    }));
    spheres.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.45,
        material: Arc::new(Dielectric{ref_idx: 1.5})
    }));    // Hollow glass sphere (soap bubble)

    // Image calculation
    for j in (0..image_height).rev() {
        eprint!("Scanlines remaining: {}\r", j);
        for i in 0..image_width {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..sample_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / image_width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / image_height as f64;
                let r = cam.get_ray(u, v);

                let _p = r.point_at(2.0);
                col += color(&r, &spheres, max_depth);
            }

            col /= sample_per_pixel as f64;
            let col = gamma(col);

            println!("{}", col);
        }
    }

    eprintln!("Done!                    ");
}
