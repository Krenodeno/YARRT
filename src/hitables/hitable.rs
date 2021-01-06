use super::aabb::Aabb;
use crate::materials::Material;
pub use crate::structs::*;
use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        t: f64,
        u: f64,
        v: f64,
        p: Vec3,
        ray: &Ray,
        outward_normal: Vec3,
        material: Arc<dyn Material>, // TODO Maybe use a weak pointer here
    ) -> HitRecord {
        let (front_face, normal) = compute_face_normal(ray, &outward_normal);
        HitRecord {
            t,
            u,
            v,
            p,
            front_face,
            normal,
            material,
        }
    }
}

/// compute and return front_face and normal
fn compute_face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
    let front_face = dot(ray.direction(), *outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal.clone()
    } else {
        -outward_normal
    };
    (front_face, normal)
}

pub trait Hitable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;
}
