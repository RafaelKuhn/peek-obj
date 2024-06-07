use std::fmt;

use crate::{FVec2, Int};


#[derive(Clone)]
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

impl std::ops::Sub for IVec2 {
	type Output = IVec2;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::Output {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl std::ops::Sub for &IVec2 {
	type Output = IVec2;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::Output {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}


impl std::ops::Add for IVec2 {
	type Output = IVec2;

	fn add(self, rhs: Self) -> Self::Output {
		Self::Output {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl std::ops::Add for &IVec2 {
	type Output = IVec2;

	fn add(self, rhs: Self) -> Self::Output {
		Self::Output {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}


impl std::ops::Mul<f32> for &IVec2 {
	type Output = IVec2;

	fn mul(self, rhs: f32) -> Self::Output {
		Self::Output {
			x: (self.x as f32 * rhs) as Int,
			y: (self.y as f32 * rhs) as Int,
		}
	}
}


impl From<FVec2> for IVec2 {
	fn from(vec: FVec2) -> IVec2 {
		IVec2::new(vec.x as Int, vec.y as Int)
	}
}

impl From<&FVec2> for IVec2 {
	fn from(vec: &FVec2) -> IVec2 {
		IVec2::new(vec.x as Int, vec.y as Int)
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
