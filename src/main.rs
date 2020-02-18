use rust_tracing::vec3::Vec3;

fn main() {
    let nx: u16 = 200;
    let ny: u16 = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let v = Vec3 {
                x: i as f64 / nx as f64,
                y: j as f64 / ny as f64,
                z: 0.2,
            };

            let ir = (255.99 * v.x) as u8;
            let ig = (255.99 * v.y) as u8;
            let ib = (255.99 * v.z) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
