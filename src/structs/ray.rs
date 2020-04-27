use super::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            time,
        }
    }

    pub fn origin(&self) -> Vec3 {
        Vec3 {
            x: self.orig.x,
            y: self.orig.y,
            z: self.orig.z,
        }
    }

    pub fn direction(&self) -> Vec3 {
        Vec3 {
            x: self.dir.x,
            y: self.dir.y,
            z: self.dir.z,
        }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        &self.orig + &(&self.dir * t)
    }
}