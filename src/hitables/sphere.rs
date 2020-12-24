use super::hitable::*;
use super::Aabb;
use crate::materials::Material;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    /// Get the u,v pair of coordinates of the point p on the sphere.
    /// p: a given pont on the sphere of radius one, centered at the origin.
    /// u: returned value [0,1] of angle around the Y axis from X = -1.
    /// v: returned value [0,1] of angle from Y = -1 to Y = +1.
    pub fn get_sphere_uv(p: &Vec3) -> (f64, f64) {
        let phi = (-p.z).atan2(p.x) + PI;
        let theta = p.y.acos();

        let u = phi / (2.0_f64 * PI);
        let v = theta / PI;
        (u, v)
    }
}

fn compute_hit(ray: &Ray, temp: f64, sphere: &Sphere) -> Option<HitRecord> {
    let p = ray.point_at(temp);
    let outward_normal = (p - sphere.center) / sphere.radius;
    let (u, v) = Sphere::get_sphere_uv(&outward_normal);
    let rec = HitRecord::new(temp, u, v, p, ray, outward_normal, sphere.material.clone());
    Some(rec)
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
                return compute_hit(ray, temp, self);
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                return compute_hit(ray, temp, self);
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(Aabb {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}
