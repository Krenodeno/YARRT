use super::hitable::*;
use super::Aabb;
use crate::materials::Material;
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        let phi = p.z.atan2(p.x);
        let theta = p.y.asin();
        let u = 1.0_f64 - (phi + std::f64::consts::PI) / 2.0_f64 * std::f64::consts::PI;
        let v = (theta + std::f64::consts::PI / 2.0_f64) / std::f64::consts::PI;
        (u, v)
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().squared_length();
        let half_b = dot(oc, ray.direction());
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let outward_normal = (p - self.center) / self.radius;
                let (u, v) = Sphere::get_sphere_uv(&((p - self.center) / self.radius));
                let rec = HitRecord::new(temp, u, v, p, ray, outward_normal, self.material.clone());
                return Some(rec);
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let outward_normal = (p - self.center) / self.radius;
                let (u, v) = Sphere::get_sphere_uv(&((p - self.center) / self.radius));
                let rec = HitRecord::new(temp, u, v, p, ray, outward_normal, self.material.clone());
                return Some(rec);
            }
        }
        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(Aabb {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}
