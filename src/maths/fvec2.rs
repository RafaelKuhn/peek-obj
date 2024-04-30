use std::fmt;

use crate::{Float, IVec2, UVec2};

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

impl From<IVec2> for UVec2 {
	fn from(vec: IVec2) -> UVec2 {
		debug_assert!(vec.x >= 0, "ivec {:} x is < 0", vec);
		debug_assert!(vec.y >= 0, "ivec {:} y is < 0", vec);
		UVec2::new(vec.x as u16, vec.y as u16)
	}
}

impl From<&IVec2> for UVec2 {
	fn from(vec: &IVec2) -> UVec2 {
		debug_assert!(vec.x >= 0, "ivec {:} x is < 0", vec);
		debug_assert!(vec.y >= 0, "ivec {:} y is < 0", vec);
		UVec2::new(vec.x as u16, vec.y as u16)
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
