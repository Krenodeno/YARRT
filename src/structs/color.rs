#[derive(Debug, Copy, Clone)]
pub struct Color {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}

impl Color {
	fn new(r: f64, g: f64, b:f64) -> Color {
		Color {
			r: r,
			g: g,
			b: b,
		}
	}

	fn black() -> Color {
		Color {
			r: 0.0,
			g: 0.0,
			b: 0.0,
		}
	}

	fn white() -> Color {
		Color {
			r: 1.0,
			g: 1.0,
			b: 1.0,
		}
	}
}