use super::{Color, Resource, Vec3};

use std::hash::Hash;
use std::path::Path;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

impl<T: Texture> Resource for T {}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum TextureKind<'a> {
    Constant(Color),
    Checker(&'a TextureConfig<'a>, &'a TextureConfig<'a>),
    FromFile(&'a Path),
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct TextureConfig<'a> {
    pub kind: TextureKind<'a>,
}

// Solid Color

pub struct SolidColor {
    pub color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color
    }
}

// Checker Texture

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.value(u, v, &p)
        } else {
            self.even.value(u, v, &p)
        }
    }
}
