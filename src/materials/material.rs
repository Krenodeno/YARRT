use crate::hitables::HitRecord;
use crate::structs::{Ray, Vec3};

pub trait Material: Send + Sync {
    /// Returns a tuple composed of attenuation on RGB channels and the scattered ray, if applicable
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;

    /// Return the emissive color
    /// Default implementation return no emissive color (black)
    fn emit(&self, _u: f64, _b: f64, _p: &Vec3) -> Vec3 {
        Vec3::default()
    }
}
