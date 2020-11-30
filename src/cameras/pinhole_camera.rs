use super::Camera;
use crate::structs::{cross, unit_vector, Ray, Vec3};

use rand::Rng;

pub struct PinholeCamera {
    pub origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    // shutter open/close times
    time0: f64,
    time1: f64,
}

impl PinholeCamera {
    pub fn new_look_at(
        lookfrom: Vec3,
        lookat: Vec3,
        up: Vec3,
        vfov: f64,
        aspect: f64,
        t0: f64,
        t1: f64,
    ) -> PinholeCamera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(up, w));
        let v = cross(w, u);

        PinholeCamera {
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            time0: t0,
            time1: t1,
        }
    }

    pub fn new(vfov: f64, aspect: f64, t0: f64, t1: f64) -> PinholeCamera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        PinholeCamera {
            origin: Vec3::default(),
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0 * half_height, 0.0),
            time0: t0,
            time1: t1,
        }
    }

    pub fn default() -> PinholeCamera {
        PinholeCamera {
            origin: Vec3::default(),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            time0: 0.0,
            time1: 0.0,
        }
    }
}

impl Camera for PinholeCamera {
    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
            rand::thread_rng().gen_range(self.time0, self.time1), // TODO pre-gen for better performances
        )
    }
}
