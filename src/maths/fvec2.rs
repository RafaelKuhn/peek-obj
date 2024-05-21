use std::fmt;

use crate::{Float, IVec2, Int, UVec2};

#[derive(Clone)]
pub struct FVec2 {
	pub x: Float,
	pub y: Float,
}

impl FVec2 {
	pub fn new(x: Float, y: Float) -> Self {
		Self { x, y }
	}

	pub fn sum(&self, rhs: &FVec2) -> Self {
		Self { x: self.x + rhs.x, y: self.y + rhs.y }
	}

	pub fn sum_t(&self, rhs: (Float, Float)) -> Self {
		Self { x: self.x as Float + rhs.0, y: self.y as Float + rhs.1 }
	}

	pub fn round_into_ivec2(&self) -> IVec2 {
		IVec2::new(self.x.round() as Int, self.y.round() as Int)
	}

	pub fn magnitude(&self) -> f32 {
		let xf = self.x;
		let yf = self.y;
		(xf * xf + yf * yf).sqrt()
	}

	pub fn dist_to(&self, other: &Self) -> f32 {
		let vec = Self::new(other.x - self.x, other.y - self.y);
		vec.magnitude()
	}

	pub fn scale_y(&mut self, scale: f32) {
		self.y *= scale;
	}

	pub fn squared_magnitude(&self) -> f32 {
		self.x * self.x + self.y * self.y
	}
}


impl std::ops::Sub for FVec2 {
	type Output = FVec2;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::Output {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl std::ops::Sub for &FVec2 {
	type Output = FVec2;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::Output {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}


impl fmt::Display for FVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{:+.2}, {:+.2}]", self.x, self.y)
	}
}

impl fmt::Debug for FVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({:.6}, {:.6})", self.x, self.y)
	}
}
