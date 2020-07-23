#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }

    fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}
