use super::vec3::*;

#[derive(Debug)]
pub struct Ray {
	orig: Vec3,
	dir: Vec3,
}

impl Ray {
	pub fn from(origin: &Vec3, direction: &Vec3) -> Ray {
		Ray {
			orig: Vec3{x:origin.x, y:origin.y, z:origin.z},
			dir: Vec3{x:direction.x, y:direction.y, z:direction.z},
		}
	}

	pub fn origin(&self) -> Vec3 {
		Vec3 {
			x: self.orig.x,
			y: self.orig.y,
			z: self.orig.z,
		}
	}

	pub fn direction(&self) -> Vec3 {
		Vec3 {
			x: self.dir.x,
			y: self.dir.y,
			z: self.dir.z,
		}
	}

	pub fn point_at(&self, t: f64) -> Vec3 {
		&self.orig + &(&self.dir * t)
	}
}