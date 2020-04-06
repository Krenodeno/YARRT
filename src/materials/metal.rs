use super::Material;
use super::{random_in_unit_sphere, reflect};
use crate::structs::{dot, Ray, unit_vector, Vec3};
use crate::hitables::HitRecord;

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzziness: fuzz.max(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected: Vec3 = reflect(&unit_vector(ray.direction()), &rec.normal);
        let scattered = Ray::from(rec.p, reflected + self.fuzziness * random_in_unit_sphere());
        let attenuation = self.albedo;
        if dot(scattered.direction(), rec.normal) > 0.0 {
            return Some((attenuation, scattered));
        }
        else {
            return None;
        }
    }
}