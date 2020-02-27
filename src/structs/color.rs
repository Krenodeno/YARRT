#[derive(Debug)]
pub struct Color {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}

impl Color {
	fn BLACK() -> Color {
		Color {
			r: 0.0,
			g: 0.0,
			b: 0.0,
		}
	}

	fn WHITE() -> Color {
		Color {
			r: 1.0,
			g: 1.0,
			b: 1.0,
		}
	}
}