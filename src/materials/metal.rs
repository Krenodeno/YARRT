use super::Material;
use super::random_in_unit_sphere;
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
			fuzziness: if fuzz > 1.0 {
				1.0
			} else {
				fuzz
			}
		}
	}
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
	v - 2.0 * dot(v, n) * n
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		let reflected: Vec3 = reflect(unit_vector(ray.direction()), rec.normal);
		*scattered = Ray::from(rec.p, reflected + self.fuzziness * random_in_unit_sphere());
		*attenuation = self.albedo;
		dot(scattered.direction(), rec.normal) > 0.0
	}
}