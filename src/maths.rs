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

