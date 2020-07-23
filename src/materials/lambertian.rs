use super::random_unit_vector;
use super::Material;
use crate::hitables::HitRecord;
use crate::structs::{Ray, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        // Different diffuse distributions possibles, see materials/mod.rs
        let scatter_direction = rec.normal + random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction, ray.time());
        Some((self.albedo, scattered))
    }
}
