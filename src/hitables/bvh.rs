use super::{surrounding_box, Aabb, HitRecord, Hitable, HitableList, Ray};

use std::cmp::Ordering;
use std::sync::Arc;

use rand::prelude::*;

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    bb: Aabb,
}

impl BVHNode {
    /// Construct a BVH from a list of hitables objects
    pub fn new(list: &HitableList, time0: f64, time1: f64) -> BVHNode {
        BVHNode::from(&list.list[..], time0, time1)
    }

    /// Construct a BVH based on a slice of hitables
    /// Use a randomly choosen axis to cut
    /// For other construction methods, see SAH ...
    fn from(objects: &[Arc<dyn Hitable>], time0: f64, time1: f64) -> BVHNode {
        let axis = thread_rng().gen_range(0, 2); // TODO pre-gen for better performances
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!(),
        };

        let object_span = objects.len();

        let (left, right) = match object_span {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => {
                if let Ordering::Less = comparator(&objects[0], &objects[1]) {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            _ => {
                let mut sorted = vec![];
                sorted.reserve(object_span);
                for i in objects {
                    sorted.push(i.clone())
                }
                sorted.sort_by(comparator);

                let mid = object_span / 2;
                (
                    Arc::new(BVHNode::from(&sorted[..mid], time0, time1)) as Arc<dyn Hitable>,
                    Arc::new(BVHNode::from(&sorted[mid..], time0, time1)) as Arc<dyn Hitable>,
                )
            }
        };

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        if box_left.is_none() && box_right.is_none() {
            eprintln!("Error: No bounding box in BVHNode construction.");
        }

        BVHNode {
            left,
            right,
            bb: surrounding_box(&box_left.unwrap(), &box_right.unwrap()),
        }
    }
}

fn box_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() && box_b.is_none() {
        eprintln!("Error: No Bounding box in BVHNode constrction.");
    }

    box_a.unwrap().min[axis]
        .partial_cmp(&box_b.unwrap().min[axis])
        .unwrap()
}

fn box_x_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
    box_compare(a, b, 2)
}

impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bb.hit(ray, t_min, t_max) {
            return None;
        }

        // This combinator return right node hit when left node isn't hit
        // or left node hit if left node is hit but right node isn't
        self.left
            .hit(ray, t_min, t_max)
            .and_then(|h| self.right.hit(ray, t_min, h.t).or_else(|| Some(h)))
            .or_else(|| self.right.hit(ray, t_min, t_max))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(self.bb.clone())
    }
}
