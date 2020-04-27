pub use crate::structs::*;
use crate::materials::Material;
use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, ray: &Ray, outward_normal: Vec3, material: Arc<dyn Material>) -> HitRecord {
        let face = dot(ray.direction(), outward_normal) < 0.0;
        let normal = if face { outward_normal } else { -outward_normal };
        HitRecord {
            t,
            p,
            front_face: face,
            normal,
            material,
        }
    }
}

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}