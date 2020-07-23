use crate::hitables::HitRecord;
use crate::structs::{Ray, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}
