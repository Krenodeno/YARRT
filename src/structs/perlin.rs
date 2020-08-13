use super::Vec3;
use rand::Rng;

pub struct Perlin {
    point_count: usize,
    random_floats: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Perlin {
        let mut ranfloat = Vec::with_capacity(point_count);
        for _i in 0..point_count {
            ranfloat.push(rand::thread_rng().gen());
        }

        Perlin {
            point_count,
            random_floats: ranfloat,
            perm_x: Perlin::generate_permutations(point_count, point_count),
            perm_y: Perlin::generate_permutations(point_count, point_count),
            perm_z: Perlin::generate_permutations(point_count, point_count),
        }
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        let i = (4 * point.x as usize) & 255;
        let j = (4 * point.y as usize) & 255;
        let k = (4 * point.z as usize) & 255;

        self.random_floats[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn generate_permutations(capacity: usize, point_count: usize) -> Vec<usize> {
        let mut values = Vec::with_capacity(capacity);
        for i in 0..point_count {
            values.push(i);
        }
        Perlin::permute(&mut values[..], point_count);
        values
    }

    /// Executes `count` permutations on `ptr` randomly
    fn permute(ptr: &mut [usize], count: usize) {
        for i in (1..count - 1).rev() {
            let target: usize = rand::thread_rng().gen_range(0, i);
            ptr.swap(i, target);
        }
    }
}
