// #![allow(clippy::let_and_return)]

use std::cmp::Ordering;

use crate::{maths::*, render_string, terminal::TerminalBuffer, Primitive};

use self::vec3::Vec4;


#[must_use]
pub fn clip_space_to_screen_space(p: &Vec3, screen_width: u16, screen_height: u16) -> IVec2 {
	let screen_x = (p.x + 1.0) * 0.5 * screen_width  as f32;
	let screen_y = (p.y + 1.0) * 0.5 * screen_height as f32;

	IVec2::new(screen_x as Int, screen_y as Int)
}

#[must_use]
pub fn clip_space_to_screen_space_f(p: &Vec3, screen_width: u16, screen_height: u16) -> FVec2 {
	let screen_x = (p.x + 1.0) * 0.5 * screen_width  as f32;
	let screen_y = (p.y + 1.0) * 0.5 * screen_height as f32;

	FVec2::new(screen_x, screen_y)
}


pub struct ScreenTri {
	pub p0: IVec2,
	pub p1: IVec2,
	pub p2: IVec2,
}

impl ScreenTri {
	pub fn from_clip_space(p0: Vec4, p1: Vec4, p2: Vec4, wid: u16, hei: u16) -> Self {
		let p0 = clip_space_to_screen_space(&p0.homogeneous(), wid, hei);
		let p1 = clip_space_to_screen_space(&p1.homogeneous(), wid, hei);
		let p2 = clip_space_to_screen_space(&p2.homogeneous(), wid, hei);
		ScreenTri { p0, p1, p2 }
	}

	pub fn from_screen_points(p0: FVec2, p1: FVec2, p2: FVec2) -> Self {
		ScreenTri { p0: p0.into(), p1: p1.into(), p2: p2.into() }
	}
}

// TODO: could do three dot products, if a triangle is
pub fn cull_tri_into_screen_space(p0: Vec4, p1: Vec4, p2: Vec4, buf: &mut TerminalBuffer) -> Option<ScreenTri> {
	
	// the triangle is inside the screen, don't cull
	// buf.write_debug(&format!("p0 {:?} in w ran {} \n", p0, p0.in_w_range()));
	if p0.in_w_range() { return Some(ScreenTri::from_clip_space(p0, p1, p2, buf.wid, buf.hei)) }
	// buf.write_debug(&format!("p1 {:?} in w ran {} \n", p1, p1.in_w_range()));
	if p1.in_w_range() { return Some(ScreenTri::from_clip_space(p0, p1, p2, buf.wid, buf.hei)) }
	// buf.write_debug(&format!("p2 {:?} in w ran {} \n\n", p2, p2.in_w_range()));
	if p2.in_w_range() { return Some(ScreenTri::from_clip_space(p0, p1, p2, buf.wid, buf.hei)) }

	let screen_p0 = clip_space_to_screen_space_f(&p0.homogeneous(), buf.wid, buf.hei);
	let screen_p1 = clip_space_to_screen_space_f(&p1.homogeneous(), buf.wid, buf.hei);
	let screen_p2 = clip_space_to_screen_space_f(&p2.homogeneous(), buf.wid, buf.hei);

	let (width, height) = (buf.wid as f32, buf.hei as f32);

	// buf.write_debug(&format!("p0..p1 inters scr {}\n", line_intersect_screen(&screen_p0, &screen_p1, width, height)));
	if line_intersect_screen(&screen_p0, &screen_p1, width, height) { return Some(ScreenTri::from_screen_points(screen_p0, screen_p1, screen_p2)) }
	// buf.write_debug(&format!("p1..p2 inters scr {}\n", line_intersect_screen(&screen_p1, &screen_p2, width, height)));
	if line_intersect_screen(&screen_p1, &screen_p2, width, height) { return Some(ScreenTri::from_screen_points(screen_p0, screen_p1, screen_p2)) }
	// buf.write_debug(&format!("p2..p0 inters scr {}\n", line_intersect_screen(&screen_p2, &screen_p0, width, height)));
	if line_intersect_screen(&screen_p2, &screen_p0, width, height) { return Some(ScreenTri::from_screen_points(screen_p0, screen_p1, screen_p2)) }

	None	
}


pub fn cull_ball_into_radius(pos: Vec4, rad: f32, buf: &mut TerminalBuffer) -> Option<f32> {
	buf.write_debug(&format!("pos {:?} wr {} \n", pos, pos.in_w_range()));
	if pos.in_w_range() { }
	Some(7.0)
}


pub fn cull_circle(pos: &FVec2, x_rad: f32, buf: &mut TerminalBuffer) -> bool {
	let (wid, hei) = (buf.wid as f32, buf.hei as f32);
	
	let (last_x, last_y) = (wid - 1.0, hei - 1.0);

	let closest_point = if pos.x >= wid {
		// buf.write_debug(&format!("to the right"));
		FVec2::new(last_x, pos.y.clamp(0.0, last_y))
	} else if pos.x < 0.0 {
		// buf.write_debug(&format!("to the left"));
		FVec2::new(0.0, pos.y.clamp(0.0, last_y))
	} else if pos.y >= hei {
		// buf.write_debug(&format!("downwards"));
		FVec2::new(pos.x, last_y)
	} else if pos.y < 0.0 {
		// buf.write_debug(&format!("upwards"));
		FVec2::new(pos.x, 0.0)
	} else {
		return false;
	};


	// X scale is double Y scale
	let mut vec_pos_to_closest = pos - &closest_point;
	vec_pos_to_closest.scale_y(2.0);

	let sq_magnitude = vec_pos_to_closest.squared_magnitude();
	let sq_x_rad = x_rad * x_rad;

	// render_char('&', &(&closest_point).into(), buf);
	// buf.write_debug(&format!("\npos to closest {:?}\nsq mag {:.4}, sq rad {:.2}\n(rad {:.4})\nCULL? {} \n\n", vec_pos_to_closest, sq_magnitude, sq_x_rad, x_rad, sq_magnitude >= sq_x_rad));

	sq_magnitude >= sq_x_rad
}


#[must_use]
pub fn screen_project(vec: &Vec3, render_mat: &[f32], wid: u16, hei: u16) -> IVec2 {
	let projected_3d = vec.get_transformed_by_mat4x4_homogeneous(render_mat);
	let projected_2d = clip_space_to_screen_space(&projected_3d, wid, hei);
	projected_2d
}

#[must_use]
pub fn screen_project_f(vec: &Vec3, render_mat: &[f32], wid: u16, hei: u16) -> FVec2 {
	let projected_3d = vec.get_transformed_by_mat4x4_homogeneous(render_mat);
	let projected_2d = clip_space_to_screen_space_f(&projected_3d, wid, hei);
	projected_2d
}

fn render_uvec2_dbg(vec: &UVec2, pos: &UVec2, buf: &mut TerminalBuffer) {
	render_string(&format!("[{},{}]", vec.x, vec.y), pos, buf);
}

fn render_vec3_dbg(vec: &Vec3, pos: &UVec2, buf: &mut TerminalBuffer) {
	render_string(&format!("[{:+.2},{:+.2},{:+.2}]", vec.x, vec.y, vec.z), pos, buf);
}