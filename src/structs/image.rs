pub enum PixelFormat {
    RGBU8,
}

pub struct Image {
    width: u32,
    height: u32,
    format: PixelFormat,
    pixels: Vec<f64>,
}

pub enum ImageFormat {
    PPM,
}

impl Image {
    pub fn new(width: u32, height: u32, format: PixelFormat) -> Image {
        let p: Vec<f64> = Vec::with_capacity((width * height) as usize);

        Image {
            width,
            height,
            format,
            pixels: p,
        }
    }

    pub fn from(width: u32, height: u32, format: PixelFormat, pixels: &[f64]) -> Image {
        let chan_count = match format {
            PixelFormat::RGBU8 => 3,
        };
        assert_eq!(pixels.len(), (width * height * chan_count) as usize);

        let p: Vec<f64> = Vec::from(pixels);
        Image {
            width,
            height,
            format,
            pixels: p,
        }
    }

    pub fn get_pixels(&self) -> &Vec<f64> {
        &self.pixels
    }

    pub fn set_pixels(&mut self, first: usize, last: usize, pixels: &[f64]) {
        let mut iter = pixels.iter();
        for subpixel in &mut self.pixels[first..last] {
            *subpixel = *iter.next().unwrap();
        }
    }
}

pub trait Serializable {
    fn encode(&self, format: ImageFormat) -> String;
}

impl Serializable for Image {
    fn encode(&self, format: ImageFormat) -> String {
        match format {
            ImageFormat::PPM => encode_ppm(&self),
        }
    }
}

fn encode_ppm(image: &Image) -> String {
    let mut img = String::new();
    img.push_str("P3\n");
    img.push_str(&image.width.to_string());
    img.push(' ');
    img.push_str(&image.height.to_string());
    img.push_str("\n255\n");

    let mut encode_one_pixel = match image.format {
        PixelFormat::RGBU8 => |r: &f64, g: &f64, b: &f64| {
            img.push_str(&((r * 255.99) as u32).to_string());
            img.push(' ');
            img.push_str(&((g * 255.99) as u32).to_string());
            img.push(' ');
            img.push_str(&((b * 255.99) as u32).to_string());
            img.push('\n');
        },
    };

    let mut subpixel = image.get_pixels().iter();
    for _i in 0..image.width * image.height {
        match image.format {
            PixelFormat::RGBU8 => {
                let r = subpixel.next().unwrap();
                let g = subpixel.next().unwrap();
                let b = subpixel.next().unwrap();
                encode_one_pixel(r, g, b)
            }
        }
    }

    img
}
