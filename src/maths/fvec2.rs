use std::fmt;

use crate::{Float, IVec2, Int, UVec2};

pub struct FVec2 {
	pub x: Float,
	pub y: Float,
}

impl FVec2 {
	pub fn new(x: Float, y: Float) -> Self {
		Self { x, y }
	}

	pub fn sum_t(&self, rhs: (Float, Float)) -> Self {
		Self { x: self.x as Float + rhs.0, y: self.y as Float + rhs.1 }
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
}

impl Into<IVec2> for FVec2 {
	fn into(self) -> IVec2 {
		IVec2::new(self.x as Int, self.y as Int)
	}
}

impl Into<IVec2> for &FVec2 {
	fn into(self) -> IVec2 {
		IVec2::new(self.x as Int, self.y as Int)
	}
}

// TODO: delete
impl Into<UVec2> for FVec2 {
	fn into(self) -> UVec2 {
		debug_assert!(self.x >= 0.0, "ivec {:} x is < 0", self);
		debug_assert!(self.y >= 0.0, "ivec {:} y is < 0", self);
		UVec2::new(self.x as u16, self.y as u16)
	}
}

// TODO: delete
impl Into<UVec2> for &FVec2 {
	fn into(self) -> UVec2 {
		debug_assert!(self.x >= 0.0, "ivec {:} x is < 0", self);
		debug_assert!(self.y >= 0.0, "ivec {:} y is < 0", self);
		UVec2::new(self.x as u16, self.y as u16)
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
