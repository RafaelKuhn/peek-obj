use std::fmt;

use crate::{maths::*, clip_space_to_screen_space, ivec2::IVec2};


#[derive(Clone, Copy)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}

	pub fn zero() -> Self {
		Self { x: 0.0, y: 0.0, z: 0.0 }
	}

	pub fn side() -> Self {
		Self { x: 0.0, y: 0.0, z: 1.0 }
	}
	pub fn up() -> Self {
		Self { x: 0.0, y: 0.0, z: 1.0 }
	}
	pub fn forward() -> Self {
		Self { x: 0.0, y: 0.0, z: 1.0 }
	}

	pub fn squared_magnitude(&self) -> f32 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn squared_dist_to(&self, other: &Vec3) -> f32 {
		let vec = Self::new(other.x - self.x, other.y - self.y, other.z - self.z);
		vec.squared_magnitude()
	}

	pub fn magnitude(&self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn normalized(&self) -> Self {
		let magnitude = self.magnitude();
		Vec3::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
	}

	pub fn dist_to(&self, other: &Vec3) -> f32 {
		let vec = Self::new(other.x - self.x, other.y - self.y, other.z - self.z);
		vec.magnitude()
	}

	pub fn with_y_inverted(&self) -> Self {
		Self { x: self.x, y: -self.y, z: self.z }
	}

	pub fn invert_y(mut self) -> Self {
		let self_mut = &mut self;
		self_mut.y = -self_mut.y;
		self
	}

	pub fn clip_space_to_screen_space(&self, screen_width: u16, screen_height: u16) -> IVec2 {
		clip_space_to_screen_space(self, screen_width, screen_height)
	}

	pub fn get_transformed_by_mat3x3(&self, mat: &[f32]) -> Self {
		const SZ: u16 = 3;
		let x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)];
		let y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)];
		let z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)];

		Self { x, y, z }
	}

	pub fn transform_by_mat3x3(&mut self, mat: &[f32]) {
		const SZ: u16 = 3;
		let x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)];
		let y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)];
		let z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)];

		self.x = x;
		self.y = y;
		self.z = z;
	}

	pub fn get_transformed_by_mat4x4_discard_w(&self, mat: &[f32]) -> Self {
		const SZ: u16 = 4;
		let x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)] + 1.0 * mat[xy_to_it(3, 0, SZ)];
		let y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)] + 1.0 * mat[xy_to_it(3, 1, SZ)];
		let z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)] + 1.0 * mat[xy_to_it(3, 2, SZ)];

		Self { x, y, z }
	}

	pub fn get_transformed_by_mat4x4_uniform(&self, mat: &[f32]) -> Self {
		const SZ: u16 = 4;
		let mut x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)] + 1.0 * mat[xy_to_it(3, 0, SZ)];
		let mut y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)] + 1.0 * mat[xy_to_it(3, 1, SZ)];
		let mut z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)] + 1.0 * mat[xy_to_it(3, 2, SZ)];
		let w     = self.x * mat[xy_to_it(0, 3, SZ)] + self.y * mat[xy_to_it(1, 3, SZ)] + self.z * mat[xy_to_it(2, 3, SZ)] + 1.0 * mat[xy_to_it(3, 3, SZ)];

		if w != 0.0 {
			x /= w;
			y /= w;
			z /= w;
		}

		Self { x, y, z }
	}

	pub fn add_vec(&self, rhs: &Vec3) -> Self {
		Vec3 {
			x: rhs.x + self.x,
			y: rhs.y + self.y,
			z: rhs.z + self.z,
		}
	}

	pub fn mul_vec(&self, rhs: f32) -> Self {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}

	pub fn inversed(&self) -> Self {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}

	pub fn cross_product(a: &Vec3, b: &Vec3) -> Vec3 {
		Vec3::new(
			a.y * b.z - a.z * b.y,
			a.z * b.x - a.x * b.z,
			a.x * b.y - a.y * b.x,
		)
	}

	pub fn dot_product(a: &Vec3, b: &Vec3) -> f32 {
		a.x*b.x + a.y*b.y + a.z*b.z
	}
}

impl std::ops::Add<&Vec3> for &Vec3 {
	type Output = Vec3;

	fn add(self, rhs: &Vec3) -> Self::Output {
		self.add_vec(rhs)
	}
}

impl std::ops::Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Self::Output {
		self.add_vec(&rhs)
	}
}

impl std::ops::Mul<f32> for &Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f32) -> Self::Output {
		self.mul_vec(rhs)
	}
}

impl std::ops::Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f32) -> Self::Output {
		self.mul_vec(rhs)
	}
}

impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{:+.2}, {:+.2}, {:+.2}]", self.x, self.y, self.z)
	}
}

impl fmt::Debug for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({:.6}, {:.6}, {:.6})", self.x, self.y, self.z)
	}
}