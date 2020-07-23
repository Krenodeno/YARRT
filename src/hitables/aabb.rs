use crate::structs::{Ray, Vec3};

#[derive(Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let origin = ray.origin();
        let direction = ray.direction();
        for i in 0..3 {
            let inv_dir = 1.0 / direction[i];
            let mut t0 = (self.min[i] - origin[i]) * inv_dir;
            let mut t1 = (self.max[i] - origin[i]) * inv_dir;
            if inv_dir < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = Vec3::new(
        box0.min.x.min(box1.min.x),
        box0.min.y.min(box1.min.y),
        box0.min.z.min(box1.min.z),
    );

    let big = Vec3::new(
        box0.max.x.max(box1.max.x),
        box0.max.y.max(box1.max.y),
        box0.max.z.max(box1.max.z),
    );

    Aabb {
        min: small,
        max: big,
    }
}
