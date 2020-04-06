use super::Material;
use super::{random_unit_vector};
use crate::structs::{Ray, Vec3};
use crate::hitables::HitRecord;

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        // Different diffuse distributions possibles, see materials/mod.rs
        let scatter_direction = rec.normal + random_unit_vector();
        let scattered = Ray::from(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}