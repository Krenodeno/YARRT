use super::Camera;

use crate::structs::{cross, random_in_unit_disk, unit_vector, Ray, Vec3};

use rand::Rng;

pub struct ThinLensCamera {
    pub origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    // shutter open/close times
    time0: f64,
    time1: f64,
}

impl ThinLensCamera {
    pub fn new_look_at(
        lookfrom: Vec3,
        lookat: Vec3,
        up: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> ThinLensCamera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(up, w));
        let v = cross(w, u);

        ThinLensCamera {
            origin: lookfrom,
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time0: t0,
            time1: t1,
        }
    }
}

impl Camera for ThinLensCamera {
    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rand::thread_rng().gen_range(self.time0, self.time1),
        )
    }
}
