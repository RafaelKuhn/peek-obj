use std::fmt;

use crate::{uvec2::UVec2, Int};


pub struct IVec2 {
	pub x: Int,
	pub y: Int,
}

impl IVec2 {
	pub fn new(x: Int, y: Int) -> Self {
		Self { x, y }
	}

	pub fn sum(&self, rhs: &IVec2) -> Self {
		Self { x: self.x + rhs.x, y: self.y + rhs.y }
	}

	pub fn sum_t(&self, rhs: (Int, Int)) -> Self {
		Self { x: self.x + rhs.0, y: self.y + rhs.1 }
	}

	pub fn magnitude(&self) -> f32 {
		let xf = self.x as f32;
		let yf = self.y as f32;
		(xf * xf + yf * yf).sqrt()
	}

	pub fn dist_to(&self, other: &Self) -> f32 {
		let vec = Self::new(other.x - self.x, other.y - self.y);
		vec.magnitude()
	}
}

impl Into<UVec2> for IVec2 {
	fn into(self) -> UVec2 {
		debug_assert!(self.x >= 0, "ivec {:} x is < 0", self);
		debug_assert!(self.y >= 0, "ivec {:} y is < 0", self);
		UVec2::new(self.x as u16, self.y as u16)
	}
}

impl Into<UVec2> for &IVec2 {
	fn into(self) -> UVec2 {
		debug_assert!(self.x >= 0, "ivec {:} x is < 0", self);
		debug_assert!(self.y >= 0, "ivec {:} y is < 0", self);
		UVec2::new(self.x as u16, self.y as u16)
	}
}

impl fmt::Display for IVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}

impl fmt::Debug for IVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({:}, {:})", self.x, self.y)
	}
}
