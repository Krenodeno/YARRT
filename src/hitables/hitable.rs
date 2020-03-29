pub use crate::structs::*;
use crate::materials::Material;
use std::sync::Arc;

pub struct HitRecord {
	pub t: f64,
	pub p: Vec3,
	pub normal: Vec3,
	pub material: Arc<dyn Material>,
}

pub trait Hitable {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}