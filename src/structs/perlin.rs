use super::{dot, unit_vector, Vec3};
use rand::Rng;

pub struct Perlin {
    random_vectors: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Perlin {
        let mut ranvecs = Vec::with_capacity(point_count);
        for _i in 0..point_count {
            ranvecs.push(unit_vector(Vec3::random_range(-1.0, 1.0)));
        }

        Perlin {
            random_vectors: ranvecs,
            perm_x: Perlin::generate_permutations(point_count, point_count),
            perm_y: Perlin::generate_permutations(point_count, point_count),
            perm_z: Perlin::generate_permutations(point_count, point_count),
        }
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        let i = point.x.floor();
        let j = point.y.floor();
        let k = point.z.floor();

        let u = point.x - i;
        let v = point.y - j;
        let w = point.z - k;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vectors[self.perm_x
                        [((i as isize + di as isize) & 255) as usize]
                        ^ self.perm_y[((j as isize + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k as isize + dk as isize) & 255) as usize]];
                }
            }
        }

        trilinear_interpolation(u, v, w, &c)
    }

    pub fn turbulence(&self, point: &Vec3, octave_count: usize) -> f64 {
        let mut acc = 0.0;
        let mut multiplicator = 1.0;
        let mut weight = 1.0;

        for _i in 0..octave_count {
            acc += weight * self.noise(&(point * multiplicator));
            weight *= 0.5;
            multiplicator *= 2.0;
        }

        acc.abs()
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

fn trilinear_interpolation(u: f64, v: f64, w: f64, c: &[[[Vec3; 2]; 2]; 2]) -> f64 {
    // Smooth
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f = i as f64;
                let j_f = j as f64;
                let k_f = k as f64;
                let weight = Vec3::new(u - i_f, v - j_f, w - k_f);
                acc += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                    * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                    * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                    * dot(c[i][j][k], weight);
            }
        }
    }

    acc
}
