mod structs;
use structs::{vec3::*, ray::*};

fn color(ray: &Ray) -> Vec3 {
    let unit_direction = unit_vector(&ray.direction());
    let t: f64 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3{x: 1.0, y:1.0, z: 1.0} + t * Vec3{x:0.5, y:0.7, z:1.0}
}

fn main() {
    let nx: u16 = 200;
    let ny: u16 = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left_corner = Vec3{x:-2.0, y:-1.0, z:-1.0};
    let horizontal = Vec3{x:4.0, y:0.0, z:0.0};
    let vertical = Vec3{x:0.0, y:2.0, z:0.0};
    let origin = Vec3{x:0.0, y:0.0, z:0.0};

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::from(&origin, &(&lower_left_corner + u * &horizontal + v * &vertical));
            let col = color(&r);

            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
