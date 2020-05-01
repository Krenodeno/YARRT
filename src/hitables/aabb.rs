use crate::structs::{Ray, Vec3};

pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit (&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let bmin = [self.min.x, self.min.y, self.min.z];
        let bmax = [self.max.x, self.max.y, self.max.z];
        let origin = ray.origin();
        let orig = [origin.x, origin.y, origin.z];
        let direction = ray.direction();
        let dir = [direction.x, direction.y, direction.z];

        for i in 0..3 {
            let tmp0 = (bmin[i] - orig[i]) / dir[i];
            let tmp1 = (bmax[i] - orig[i]) / dir[i];
            let t0 = tmp0.min(tmp1);
            let t1 = tmp0.max(tmp1);

            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}