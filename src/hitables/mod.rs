mod hitable;
pub use hitable::*;
mod sphere;
pub use sphere::*;
mod moving_sphere;
pub use moving_sphere::*;
mod xy_rect;
pub use xy_rect::*;
mod aabb;
pub use aabb::*;
mod bvh;
pub use bvh::*;

use crate::materials::Material;

use std::sync::Arc;

pub struct HitableList {
    list: Vec<Arc<dyn Hitable>>,
    pub background: Arc<dyn Material>,
}

impl HitableList {
    pub fn new(background: Arc<dyn Material>) -> HitableList {
        HitableList {
            list: Vec::new(),
            background,
        }
    }

    pub fn push(&mut self, elem: Arc<dyn Hitable>) {
        self.list.push(elem);
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;
        for i in 0..self.list.len() {
            let temp_rec = self.list[i].hit(&ray, t_min, closest_so_far);
            if let Some(n) = temp_rec {
                closest_so_far = n.t;
                rec = Some(n);
            };
        }
        rec
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        if self.list.is_empty() {
            return None;
        }

        let first = self.list[0].bounding_box(t0, t1);

        if let Some(b) = first {
            let mut output_box = Aabb {
                min: b.min,
                max: b.max,
            };
            for hitable in &self.list {
                match hitable.bounding_box(t0, t1) {
                    Some(b) => output_box = surrounding_box(&output_box, &b),
                    None => return None,
                }
            }
            return Some(output_box);
        }

        None
    }
}
