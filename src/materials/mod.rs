mod material;
pub use material::Material;
mod lambertian;
pub use lambertian::Lambertian;
mod metal;
pub use metal::Metal;
mod dielectric;
pub use dielectric::Dielectric;

use rand::Rng;

use crate::structs::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p: Vec3 = 2.0 * Vec3{x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>()} - Vec3 {x:1.0, y:1.0, z:1.0};
        if p.squared_lentgth() >= 1.0 {
            return p;
        }
    }
}