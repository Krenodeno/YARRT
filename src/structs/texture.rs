use super::{Color, Resource, Vec3};

use std::hash::Hash;
use std::path::Path;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum TextureKind<'a> {
    Constant(Color),
    FromFile(&'a Path),
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct TextureConfig<'a> {
    pub kind: TextureKind<'a>,
}

pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Color {
        self.color
    }
}

impl<T: Texture> Resource for T {}
