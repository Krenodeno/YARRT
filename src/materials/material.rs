use crate::structs::{Ray, Vec3};
use crate::hitables::HitRecord;

pub trait Material {
	fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}