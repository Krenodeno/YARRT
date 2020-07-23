use super::Material;
use super::{random_in_unit_sphere, reflect};
use crate::hitables::HitRecord;
use crate::structs::{dot, unit_vector, Ray, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzziness: fuzz.max(0.0).min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected: Vec3 = reflect(&unit_vector(ray.direction()), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzziness * random_in_unit_sphere(),
            ray.time(),
        );
        let attenuation = self.albedo;
        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
