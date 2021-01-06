use super::Material;

use crate::hitables::HitRecord;
use crate::structs::{Ray, Texture, Vec3};

use std::sync::Arc;

pub struct Emissive {
    pub emit: Arc<dyn Texture>,
}

impl Material for Emissive {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emit(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
