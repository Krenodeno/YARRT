use super::hitable::*;
use super::aabb::*;
use crate::materials::Material;
use std::sync::Arc;

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0))*(self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().squared_length();
        let half_b = dot(oc, ray.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let outward_normal = (p - self.center(ray.time())) / self.radius;
                let rec = HitRecord::new(temp, p, ray, outward_normal, self.material.clone());
                return Some(rec);
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let outward_normal = (p - self.center(ray.time())) / self.radius;
                let rec = HitRecord::new(temp, p, ray, outward_normal, self.material.clone());
                return Some(rec);
            }
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let box0 = Aabb {
            min: self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        };

        let box1 = Aabb {
            min: self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        };

        Some(surrounding_box(&box0, &box1))
    }
}