use super::Vec3;
use super::ManagedResource;

use std::hash::Hash;
use std::path::Path;
use std::sync::Arc;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

impl<T: Texture> ManagedResource<TextureConfig<&Path>> for T {
    fn load_from_config(config: &TextureConfig<&Path>) -> Option<Arc<T>> {
        match config.kind {
            TextureKind::Constant(v) => Some(Arc::new(SolidColor::new(v))),
            _ => None,
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum TextureKind<P: AsRef<Path>> {
    Constant(Vec3),
    FromFile(P),
}

#[derive(Eq, Hash, PartialEq)]
pub struct TextureConfig<P: AsRef<Path>> {
    pub kind: TextureKind<P>,
}

pub struct SolidColor {
    pub color: Vec3,
}

impl SolidColor {
    pub fn new(color: Vec3) -> SolidColor {
        SolidColor {
            color,
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color
    }
}