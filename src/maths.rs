use core::{fmt,};
use std::fmt::Display;






// TODO: macro the shit out of this
// or use type system, require a type to be provided by each implementation of Vec3

pub struct UVec3 {
	pub x: u16,
	pub y: u16,
	pub z: u16,
}

impl UVec3 {
	pub fn new(x: u16, y: u16, z: u16,) -> Self {
		UVec3 { x, y, z }
	}
}


pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32,) -> Self {
		Vec3 { x, y, z }
	}

	pub fn transform_by_mat3x3(&mut self, mat: &Vec<f32>) {
		let x = self.x * mat[0*3 + 0] + self.y * mat[0*3 + 1] + self.z * mat[0*3 + 2];
		let y = self.x * mat[1*3 + 0] + self.y * mat[1*3 + 1] + self.z * mat[1*3 + 2];
		let z = self.x * mat[2*3 + 0] + self.y * mat[2*3 + 1] + self.z * mat[2*3 + 2];

		self.x = x;
		self.y = y;
		self.z = z;
	}

	pub fn get_translated_z(&self, v: f32) -> Self {
		Self { x: self.x, y: self.y, z: self.z + v }
	}

	pub fn get_transformed_by_mat3x3(&self, mat: &Vec<f32>) -> Self {
		let x = self.x * mat[0*3 + 0] + self.y * mat[0*3 + 1] + self.z * mat[0*3 + 2];
		let y = self.x * mat[1*3 + 0] + self.y * mat[1*3 + 1] + self.z * mat[1*3 + 2];
		let z = self.x * mat[2*3 + 0] + self.y * mat[2*3 + 1] + self.z * mat[2*3 + 2];

		Self { x, y, z }
	}

	pub fn get_transformed_by_mat4x4(&self, mat: &Vec<f32>) -> Self {
		let mut x = self.x * mat[0*4 + 0] + self.y * mat[0*4 + 1] + self.z * mat[0*4 + 2] + 1.0 * mat[0*4 + 3];
		let mut y = self.x * mat[1*4 + 0] + self.y * mat[1*4 + 1] + self.z * mat[1*4 + 2] + 1.0 * mat[1*4 + 3];
		let mut z = self.x * mat[2*4 + 0] + self.y * mat[2*4 + 1] + self.z * mat[2*4 + 2] + 1.0 * mat[2*4 + 3];
		let w     = self.x * mat[3*4 + 0] + self.y * mat[3*4 + 1] + self.z * mat[3*4 + 2] + 1.0 * mat[3*4 + 3];

		if w != 0.0 {
			x /= w;
			y /= w;
			z /= w;
		}

		Self { x, y, z }
	}

	pub fn transform_by_mat4x4(&mut self, mat: &Vec<f32>) {
		let mut x = self.x * mat[0*4 + 0] + self.y * mat[0*4 + 1] + self.z * mat[0*4 + 2] + 1.0 * mat[0*4 + 3];
		let mut y = self.x * mat[1*4 + 0] + self.y * mat[1*4 + 1] + self.z * mat[1*4 + 2] + 1.0 * mat[1*4 + 3];
		let mut z = self.x * mat[2*4 + 0] + self.y * mat[2*4 + 1] + self.z * mat[2*4 + 2] + 1.0 * mat[2*4 + 3];
		let w     = self.x * mat[3*4 + 0] + self.y * mat[3*4 + 1] + self.z * mat[3*4 + 2] + 1.0 * mat[3*4 + 3];

		if w != 0.0 {
			x /= w;
			y /= w;
			z /= w;
		}

		self.x = x;
		self.y = y;
		self.z = z;
	}
}


#[derive(Debug)]
pub struct IVec2 {
	pub x: i16,
	pub y: i16,
}


pub struct UVec2 {
	pub x: u16,
	pub y: u16,
}

impl Display for UVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
    }
}


impl UVec2 {
	pub fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}
}

// TODO: remove, check if I need to implement display or debug
impl fmt::Debug for UVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl std::ops::Sub<&UVec2> for &UVec2 {
	type Output = IVec2;
	fn sub(self, rhs: &UVec2) -> Self::Output {
		return IVec2 {
			x: rhs.x as i16 - self.x as i16,
			y: rhs.y as i16 - self.y as i16,
		}
	}
}

impl std::ops::Sub<UVec2> for UVec2 {
	type Output = IVec2;
	fn sub(self, rhs: UVec2) -> Self::Output {
		return IVec2 {
			x: rhs.x as i16 - self.x as i16,
			y: rhs.y as i16 - self.y as i16,
		}
	}
}



pub fn lerp(a: u32, b: u32, t: f32) -> u32 {
	(a as f32 * t + (b - a) as f32 * t) as u32
}

