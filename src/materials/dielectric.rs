use super::Material;
use super::reflect;
use crate::structs::{dot, Ray, unit_vector, Vec3};
use crate::hitables::HitRecord;

use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Dielectric{
    pub ref_idx: f64,
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, *n);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -(1.0 - r_out_parallel.squared_length()).sqrt() * n;
    r_out_parallel + r_out_perp
}

/// Simple polynomial approximation by Christophe Schlick
fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = unit_vector(ray.direction());

        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            // Must reflect
            let reflected = reflect(&ray.direction(), &rec.normal);
            let scattered = Ray::new(rec.p, reflected, ray.time());
            return Some((attenuation, scattered));
        }
        // Can refract
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if rand::thread_rng().gen::<f64>() < reflect_prob {
            let reflected = reflect(&unit_direction, &rec.normal);
            let scattered = Ray::new(rec.p, reflected, ray.time());
            return Some((attenuation, scattered));
        }

        let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
        let scattered = Ray::new(rec.p, refracted, ray.time());
        Some((attenuation, scattered))
    }
}