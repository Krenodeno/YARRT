use super::{Color, Perlin, Resource, ResourceConfig, ResourceManager, Vec3};

use image::GenericImageView;

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
    Perlin(usize, usize),
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct TextureConfig<'a> {
    pub kind: TextureKind<'a>,
}

impl<'a> ResourceConfig for TextureConfig<'a> {
    type AssociatedResource = dyn Texture;

    fn create_resource(
        &self,
        res_mgr: &mut ResourceManager<Self>,
    ) -> Arc<Self::AssociatedResource> {
        match self.kind {
            TextureKind::Constant(c) => {
                let v: Vec3 = Vec3::new(
                    c.r as f64 / 255.99,
                    c.g as f64 / 255.99,
                    c.b as f64 / 255.99,
                );
                Arc::new(SolidColor::new(v))
            }
            TextureKind::Checker(odd, even) => {
                let odd_texture = res_mgr.get_resource(odd);
                let even_texture = res_mgr.get_resource(even);
                Arc::new(CheckerTexture::new(odd_texture, even_texture))
            }
            TextureKind::FromFile(p) => Arc::new(ImageTexture::new(p)),
            TextureKind::Perlin(n, s) => Arc::new(PerlinTexture {
                noise: Perlin::new(n),
                scale: s as f64,
            }),
            _ => unimplemented!(),
        }
    }
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

// Image Texture

pub struct ImageTexture {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
    channel_count: u32,
}

impl ImageTexture {
    pub fn new(path: &Path) -> ImageTexture {
        let img = image::open(path);
        match img {
            Ok(i) => ImageTexture {
                pixels: i.to_rgba().into_vec(),
                width: i.width(),
                height: i.height(),
                channel_count: 4,
            },
            Err(_e) => ImageTexture {
                pixels: Vec::new(),
                width: 0,
                height: 0,
                channel_count: 0,
            },
        }
    }

    fn get_pixel(&self, row: u32, col: u32) -> Vec3 {
        let color_scale = 1.0 / 255.0;
        let bytes_per_scanline = self.channel_count * self.width;
        let pixel_index = (row * bytes_per_scanline + col * self.channel_count) as usize;
        let pixel = &self.pixels[pixel_index..pixel_index + 4];

        Vec3::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.pixels.is_empty() {
            return Vec3::new(0.0, 1.0, 1.0);
        }

        let width = self.width as f64;
        let height = self.height as f64;

        let row = u * (height - 1.0);
        let col = v * (width - 1.0);

        let row = f64::max(0.0, row);
        let col = f64::max(0.0, col);

        let row = f64::min(height - 1.0, row);
        let col = f64::min(width - 1.0, col);

        self.get_pixel(row as u32, col as u32)
    }
}

// Perlin Texture

pub struct PerlinTexture {
    noise: Perlin,
    scale: f64,
}

impl Texture for PerlinTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(p, 7)).sin())
    }
}
