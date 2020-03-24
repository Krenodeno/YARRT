mod hitable;
pub use hitable::*;
mod sphere;
pub use sphere::*;


pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList { list: Vec::new() }
    }

    pub fn push(&mut self, elem: Box<dyn Hitable>) {
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
            match temp_rec {
                Some(n) => {
                    closest_so_far = n.t;
                    rec = Some(n);
                },
                None => (),
            };
        }
        rec
    }
}
