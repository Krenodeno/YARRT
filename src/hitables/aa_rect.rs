use super::{Aabb, HitRecord, Hitable};
use crate::materials::Material;
use crate::structs::{Ray, Vec3};
use std::sync::Arc;

/// XY aligned rectangle
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

/// XY aligned rectangle
pub struct XZRect {
    pub material: Arc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl Hitable for XZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().y) / ray.direction().y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin().x + t * ray.direction().x;
        let z = ray.origin().z + t * ray.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let rec = HitRecord::new(
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
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
            min: Vec3::new(self.x0, self.k - padding, self.z0),
            max: Vec3::new(self.x1, self.k + padding, self.z1),
        })
    }
}

/// YZ aligned rectangle
pub struct YZRect {
    pub material: Arc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl Hitable for YZRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin().x) / ray.direction().x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin().y + t * ray.direction().y;
        let z = ray.origin().z + t * ray.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let rec = HitRecord::new(
            t,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
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
            min: Vec3::new(self.k - padding, self.y0, self.z0),
            max: Vec3::new(self.k + padding, self.y1, self.z1),
        })
    }
}
