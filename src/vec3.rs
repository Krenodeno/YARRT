use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_lentgth(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit_vector(&mut self) {
        let k: f64 = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: -(v1.x * v2.z - v1.z * v2.x),
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub fn unit_vector(vec: &Vec3) -> Vec3 {
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