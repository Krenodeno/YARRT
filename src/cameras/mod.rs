mod pinhole_camera;
pub use pinhole_camera::*;
mod thin_lens_camera;
pub use thin_lens_camera::*;

use crate::structs::Ray;

pub trait Camera {
    fn get_ray(&self, u: f64, v: f64) -> Ray;
}