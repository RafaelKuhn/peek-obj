use core::{fmt,};
use std::fmt::Display;


pub static TAU: f32 = 6.28318530; 


#[derive(Debug)]
pub struct Vec2 {
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
	type Output = Vec2;
	fn sub(self, rhs: &UVec2) -> Self::Output {
		return Vec2 {
			x: rhs.x as i16 - self.x as i16,
			y: rhs.y as i16 - self.y as i16,
		}
	}
}

impl std::ops::Sub<UVec2> for UVec2 {
	type Output = Vec2;
	fn sub(self, rhs: UVec2) -> Self::Output {
		return Vec2 {
			x: rhs.x as i16 - self.x as i16,
			y: rhs.y as i16 - self.y as i16,
		}
	}
}

pub fn lerp(a: u32, b: u32, t: f32) -> u32 {
	(a as f32 * t + (b - a) as f32 * t) as u32
}


// pub fn lerp<T: PrimInt>(a: T, b: T, t: f32) -> T {
// 	(a.to_f32().unwrap() * t + (b - a).to_f32().unwrap() * t) // needs to cast to T, can't do it with generics without cost
// }

// pub fn lerpf<T: Float>(a: T, b: T, t: f32) -> T {
// 	(a * t + (b - a) as f32 * t) as T;
// }

