use super::{Aabb, HitRecord, Hitable};
use crate::materials::Material;
use crate::structs::{Ray, Vec3};
use std::sync::Arc;

pub struct XYRect {
    pub material: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}

impl Hitable for XYRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().z) / ray.direction().z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x + t * ray.direction().x;
        let y = ray.origin().y + t * ray.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        let rec = HitRecord::new(
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            ray.point_at(t),
            ray,
            outward_normal,
            self.material.clone(),
        );
        Some(rec)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        // The bounding-box must have non-zero width in each dimension,
        // so pad the Z dimension a small amount
        let padding = 0.001;
        Some(Aabb {
            min: Vec3::new(self.x0, self.y0, self.k - padding),
            max: Vec3::new(self.x1, self.y1, self.k + padding),
        })
    }
}
