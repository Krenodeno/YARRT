use super::Material;
use super::random_in_unit_sphere;
use crate::structs::{Ray, Vec3};
use crate::hitables::HitRecord;

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
	pub albedo: Vec3,
}

impl Material for Lambertian {
	fn scatter(&self, _ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		let target: Vec3 = rec.p + rec.normal + random_in_unit_sphere();
		*scattered = Ray::from(rec.p, target - rec.p);
		*attenuation = self.albedo;
		true
	}
}