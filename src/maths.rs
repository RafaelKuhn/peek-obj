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

#[inline] #[must_use]
pub fn in_range(f: f32, min: f32, max: f32) -> bool {
	f > min && f < max
}


pub fn line_inside_screen(s0: &FVec2, s1: &FVec2, width: f32, height: f32) -> bool {
	in_range(s0.x, 0.0, width)  &&
	in_range(s0.y, 0.0, height) &&
	in_range(s1.x, 0.0, width)  &&
	in_range(s1.y, 0.0, height)
}

pub fn line_intersect_screen(s0: &FVec2, s1: &FVec2, width: f32, height: f32) -> bool {

	let origin   = &FVec2::new(0.0, 0.0);
	let up       = &FVec2::new(0.0, height);
	let right    = &FVec2::new(width, 0.0);
	let up_right = &FVec2::new(width, height);

	line_intersect(s0, s1, origin, up) ||
	line_intersect(s0, s1, origin, right) ||
	line_intersect(s0, s1, up_right, right) ||
	line_intersect(s0, s1, up_right, up)

	// line_intersects_origin_x(s0, s1, height) ||
	// line_intersects_origin_y(s0, s1, width) ||
	
}

#[inline]
pub fn line_intersect(s0: &FVec2, s1: &FVec2, e0: &FVec2, e1: &FVec2) -> bool {
	let (ldir0, ldir1) = line_intersects(s0, s1, e0, e1);
	in_range(ldir0, 0.0, 1.0) && in_range(ldir1, 0.0, 1.0)
}

#[inline]
pub fn line_intersects(s0: &FVec2, s1: &FVec2, e0: &FVec2, e1: &FVec2) -> (f32, f32) {
	let ldir0 = ((e1.x-e0.x) * (s0.y-e0.y) - (e1.y-e0.y) * (s0.x-e0.x)) / ((e1.y-e0.y) * (s1.x-s0.x) - (e1.x-e0.x) * (s1.y-s0.y));
	let ldir1 = ((s1.x-s0.x) * (s0.y-e0.y) - (s1.y-s0.y) * (s0.x-e0.x)) / ((e1.y-e0.y) * (s1.x-s0.x) - (e1.x-e0.x) * (s1.y-s0.y));

	(ldir0, ldir1)
}

#[inline]
pub fn line_intersects_f(x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> (f32, f32) {
	let ldir0 = ((x3-x2) * (y0-y2) - (y3-y2)*(x0-x2)) / ((y3-y2)*(x1-x0) - (x3-x2) * (y1-y0));
	let ldir1 = ((x1-x0) * (y0-y2) - (y1-y0)*(x0-x2)) / ((y3-y2)*(x1-x0) - (x3-x2) * (y1-y0));

	(ldir0, ldir1)
}

#[inline]
pub fn line_intersects_origin_x(s0: &FVec2, s1: &FVec2, height: f32) -> (f32, f32) {
	let ldir0 = (0.0 - (height) * (s0.x)) / ((height) * (s1.x-s0.x) - 0.0);
	let ldir1 = ((s1.x-s0.x) * (s0.y) - (s1.y-s0.y) * (s0.x)) / ((height) * (s1.x-s0.x) - 0.0);

	(ldir0, ldir1)
}

#[inline]
pub fn line_intersects_origin_y(s0: FVec2, s1: FVec2, width: f32) -> (f32, f32) {
	let ldir0 = ((width) * (s0.y) - 0.0) / (0.0 - (width) * (s1.y-s0.y));
	let ldir1 = ((s1.x-s0.x) * (s0.y) - (s1.y-s0.y) * (s0.x)) / (0.0 - (width) * (s1.y-s0.y));

	(ldir0, ldir1)
}