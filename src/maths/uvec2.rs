
// TODO: generic? Vec<i16> and Vec<f32>

use std::{fmt, ops::Sub};

use crate::{FVec2, IVec2};

pub struct UVec2 {
	pub x: u16,
	pub y: u16,
}

impl UVec2 {
	pub fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}

	pub fn sum(&self, x: u16, y: u16) -> Self {
		Self { x: self.x + x, y: self.y + y }
	}

	pub fn sum_t(&self, rhs: (i16, i16)) -> Self {
		Self { x: (self.x as i16 + rhs.0) as u16, y: (self.y as i16 + rhs.1) as u16 }
	}

	pub fn sum_v(&self, rhs: UVec2) -> Self {
		Self { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}


impl From<&FVec2> for UVec2 {
	fn from(vec: &FVec2) -> Self {
		debug_assert!(vec.x >= 0.0, "ivec {:} x is < 0", vec);
		debug_assert!(vec.y >= 0.0, "ivec {:} y is < 0", vec);
		return UVec2 { x: vec.x as u16, y: vec.y as u16 }
	}
}

impl From<FVec2> for UVec2 {
	fn from(vec: FVec2) -> UVec2 {
		debug_assert!(vec.x >= 0.0, "ivec {:} x is < 0", vec);
		debug_assert!(vec.y >= 0.0, "ivec {:} y is < 0", vec);
		UVec2::new(vec.x as u16, vec.y as u16)
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


impl fmt::Display for UVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}

impl fmt::Debug for UVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}