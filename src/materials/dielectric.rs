use super::Material;
use super::random_in_unit_sphere;
use crate::structs::{dot, Ray, unit_vector, Vec3};
use crate::hitables::HitRecord;

#[derive(Debug, Copy, Clone)]
pub struct Dielectric{
	pub ref_idx: f64,
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
	let uv = unit_vector(*v);
	let dt = dot(uv, *n);
	let discriminant = 1.0 - ni_over_nt*ni_over_nt * (1.0 - dt*dt);
	if (discriminant > 0.0) {
		return Some(ni_over_nt * (uv - n*dt) - n * discriminant.sqrt());
	}
	else {
		return None;
	}
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
	v - 2.0 * dot(v, n) * n
}

impl Material for Dielectric {
	fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		*attenuation = Vec3{x:1.0, y: 1.0, z:1.0};
		let mut outward_normal: Vec3 = Vec3{x:1.0, y:0.0, z:0.0};
		let mut ni_over_nt = 0.0;
		let reflected = reflect(ray.direction(), rec.normal);
		if dot(ray.direction(), rec.normal) > 0.0 {
			let refracted = refract(&ray.direction(), &(-rec.normal), self.ref_idx);
			match refracted {
				Some(r) => {
					*scattered = Ray::from(rec.p, r);
				},
				None => {
					*scattered = Ray::from(rec.p, reflected);
					return false;
				}
			}
		}
		else {
			let refracted = refract(&ray.direction(), &rec.normal, 1.0 / self.ref_idx);
			match refracted {
				Some(r) => {
					*scattered = Ray::from(rec.p, r);
				},
				None => {
					*scattered = Ray::from(rec.p, reflected);
					return false;
				}
			}
		}
		return true;
	}
}