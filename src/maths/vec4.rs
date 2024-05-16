use std::fmt;

use crate::maths::*;


pub struct Vec4 {
	pub xyz: Vec3,
	pub w: f32,
}

impl Vec4 {
	pub fn homogeneous(mut self) -> Vec3 {
		self.xyz.x /= self.w;
		self.xyz.y /= self.w;
		self.xyz.z /= self.w;

		self.xyz
	}

	pub fn in_w_range(&self) -> bool {
		in_range(self.xyz.x, self.w, -self.w) && in_range(self.xyz.y, self.w, -self.w)
	}

	pub fn x_in_w_range(&self) -> bool {
		in_range(self.xyz.x, self.w, -self.w)
	}
	pub fn y_in_w_range(&self) -> bool {
		in_range(self.xyz.y, self.w, -self.w)
	}

}

impl fmt::Debug for Vec4 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:.6}, {:.6}, {:.6}, {:.6}", self.xyz.x, self.xyz.y, self.xyz.z, self.w)
	}
}