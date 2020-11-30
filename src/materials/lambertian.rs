use super::random_unit_vector;
use super::Material;
use crate::hitables::HitRecord;
use crate::structs::{Ray, Texture, Vec3};

use std::sync::Arc;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        // Different diffuse distributions possibles, see materials/mod.rs
        let scatter_direction = rec.normal + random_unit_vector(); // TODO pre-gen for better performances
        let scattered = Ray::new(rec.p, scatter_direction, ray.time());
        Some((self.albedo.value(rec.u, rec.v, &rec.p), scattered))
    }
}
