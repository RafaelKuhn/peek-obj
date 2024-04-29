pub mod vec3;
pub mod ivec2;
pub mod uvec2;
pub mod fvec2;
pub mod matrices;

pub use self::uvec2::UVec2;
pub use self::ivec2::IVec2;
pub use self::fvec2::FVec2;
pub use self::vec3::Vec3;
pub use matrices::*;


pub type Int = i32;
pub type Float = f32;


#[inline(always)]
pub const fn xy_to_it(x: u16, y: u16, width: u16) -> usize {
	let y_offset = y as usize * width as usize;
	y_offset + x as usize
}

pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
	(1.0 - t) * a + b * t
}

pub fn triangle_wave(t: f32) -> f32 {
	1.0 - ((t % 1.0) - 0.5).abs() * 2.0
}

// when smoothness is 1, it's a line, values bigger than 1 smooth it
pub fn smoothed_0_to_1_s(t: f32, smoothness: f32) -> f32 {
	let t_powered = t.powf(smoothness);
	t_powered / ( t_powered + (1.0 - t).powf(smoothness) )
}

pub fn smoothed_0_to_1(t: f32) -> f32 {
	smoothed_0_to_1_s(t, 2.6)
}
