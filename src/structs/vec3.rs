use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

use std::fmt;
use rand::Rng;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", (255.99 * self.x) as u8, (255.99 * self.y) as u8, (255.99 * self.z) as u8)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: rand::thread_rng().gen(),
            y: rand::thread_rng().gen(),
            z: rand::thread_rng().gen(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: rand::thread_rng().gen_range(min, max),
            y: rand::thread_rng().gen_range(min, max),
            z: rand::thread_rng().gen_range(min, max),
        }
    }

    /// Return the length of the vector
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Return the squared length of the vector.
    /// Has the adventage to not compute any square root.
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit_vector(&mut self) {
        let k: f64 = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: -(v1.x * v2.z - v1.z * v2.x),
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub fn unit_vector(vec: Vec3) -> Vec3 {
    vec / vec.length()
}

// v1 + v2
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, mut other: Vec3) -> Self::Output {
        other.x = other.x + self.x;
        other.y = other.y + self.y;
        other.z = other.z + self.z;
        other
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, mut other: Vec3) -> Self::Output {
        other.x += self.x;
        other.y += self.y;
        other.z += self.z;
        other
    }
}

// v1 / v2
impl Div for &Vec3 {
    type Output = Vec3;

    fn div(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// v1 / f
impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

// v1 * v2
impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// v1 * f
impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Self::Output {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Self::Output {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

// f * v1
impl Mul<&Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Self::Output {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, mut other: Vec3) -> Self::Output {
        other.x *= self;
        other.y *= self;
        other.z *= self;
        other
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, mut other: Vec3) -> Self::Output {
        other.x *= self;
        other.y *= self;
        other.z *= self;
        other
    }
}

// -v1
impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// v1 - v2
impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// v2 += v1
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

// v2 /= v1
impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

// v2 /= f
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        let k: f64 = 1.0 / other;
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }
}

// v2 *= v1
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

// v2 *= f
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

// v2 -= v1
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
            self.x -= other.x;
            self.y -= other.y;
            self.z -= other.z;
    }
}