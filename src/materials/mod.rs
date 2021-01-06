mod material;
pub use material::Material;
mod lambertian;
pub use lambertian::Lambertian;
mod metal;
pub use metal::Metal;
mod dielectric;
pub use dielectric::Dielectric;
mod emissive;
pub use emissive::Emissive;

use rand::Rng;
use std::f64::consts::PI;

use crate::structs::{dot, Vec3};

// Common material functions

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(*v, *n) * n
}

// Distributions

/// Distribution for Lambertian Approximation
pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a: f64 = rng.gen_range(0.0, 2.0 * PI);
    let z: f64 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

/// More intuitive distribution
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p: Vec3 = Vec3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );
        if p.squared_length() >= 1.0 {
            continue;
        }
        return p;
    }
}
