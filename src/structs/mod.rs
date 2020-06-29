pub mod color;
mod ray;
pub use ray::*;
mod vec3;
pub use vec3::*;
mod image;
pub use image::*;
mod texture;
pub use texture::*;
mod resources_manager;
pub use resources_manager::*;

use rand::Rng;

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
        if p.squared_length() >= 1.0 {
            continue;
        }
        return p;
    }
}