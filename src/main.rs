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

fn color(ray: &Ray, world: &dyn Hitable, depth: u32) -> Vec3 {
    let record = world.hit(&ray, 0.001, std::f64::MAX);
    match record {
        Some(rec) => {
            let mut scattered: Ray = Ray::from(Vec3{x:0.0,y:0.0,z:0.0}, Vec3{x:1.0,y:0.0,z:0.0});
            let mut attenuation: Vec3 = Vec3{x:1.0,y:1.0,z:1.0};
            if depth < 50 && rec.material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * color(&scattered, world, depth+1);
            }
            else {
                return Vec3{x:0.0, y:0.0, z:0.0};
            }
        },
        None => {
            // sky color
            let unit_direction = unit_vector(ray.direction());
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - t) * Vec3{x: 1.0, y:1.0, z: 1.0} + t * Vec3{x:0.5, y:0.7, z:1.0};
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
    let nx: u16 = 200;
    let ny: u16 = 100;
    let ns: u16 = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let cam = Camera::default();
    let mut rng = rand::thread_rng();

    // Create spheres and add them to the list
    let mut spheres = HitableList::new();
    spheres.push(Box::new(Sphere {
        center: Vec3 {x:0.0, y:0.0, z:-1.0},
        radius: 0.5,
        material: Arc::new(Lambertian{albedo: Vec3{x:0.1, y:0.2, z:0.5}})
    }));
    spheres.push(Box::new(Sphere {
        center: Vec3 {x:0.0, y:-100.5, z:-1.0},
        radius: 100.0,
        material: Arc::new(Lambertian{albedo: Vec3{x:0.8, y:0.8, z:0.0}})
    }));
    spheres.push(Box::new(Sphere {
        center: Vec3 {x:1.0, y:0.0, z:-1.0},
        radius: 0.5,
        material: Arc::new(Metal::new(Vec3{x:0.8, y:0.6, z:0.2}, 0.0))
    }));
    spheres.push(Box::new(Sphere {
        center: Vec3 {x:-1.0, y:0.0, z:-1.0},
        radius: 0.5,
        material: Arc::new(Dielectric{ref_idx: 1.5})
    }));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3{x:0.0, y:0.0, z:0.0};
            for s in 0..ns {
                let u = (i as f64 + rng.gen::<f64>()) / nx as f64;
                let v = (j as f64 + rng.gen::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);

                let _p = r.point_at(2.0);
                col += color(&r, &spheres, 0);
            }

            col /= ns as f64;
            let col = gamma(col);

            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
