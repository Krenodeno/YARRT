use super::hitable::*;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin() - &self.center;
        let a = dot(&ray.direction(), &ray.direction());
        let b = dot(&oc, &ray.direction());
        let c = dot(&oc, &oc) - &self.radius * &self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let rec = HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                };
                return Some(rec);
            }
            let temp = (-b + (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at(temp);
                let rec = HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                };
                return Some(rec);
            }
        }
        None
    }
}