use crate::{maths::*, render_string, terminal_wrapper::TerminalBuffer};



pub fn clip_space_to_screen_space(p: &Vec3, screen_width: u16, screen_height: u16) -> IVec2 {
	let screen_x = (p.x + 1.0) * 0.5 * screen_width  as f32;
	let screen_y = (p.y + 1.0) * 0.5 * screen_height as f32;

	IVec2::new(screen_x as Int, screen_y as Int)
}

// TODO: delete
pub fn clip_space_to_screen_space_f32(p: &Vec3, screen_width: u16, screen_height: u16) -> (f32, f32) {
	let screen_x = (p.x + 1.0) * 0.5 * screen_width  as f32;
	let screen_y = (p.y + 1.0) * 0.5 * screen_height as f32;

	(screen_x, screen_y)
}

pub fn clip_space_to_screen_space_f(p: &Vec3, screen_width: u16, screen_height: u16) -> FVec2 {
	let screen_x = (p.x + 1.0) * 0.5 * screen_width  as f32;
	let screen_y = (p.y + 1.0) * 0.5 * screen_height as f32;

	FVec2::new(screen_x, screen_y)
}

pub fn normalize_clip_space(p: &Vec3) -> Vec3 {
	let screen_x = (p.x + 1.0) * 0.5;
	let screen_y = (p.y + 1.0) * 0.5;

	Vec3::new(screen_x, screen_y, 0.0)
}

pub fn screen_project(vec: &Vec3, render_mat: &[f32], wid: u16, hei: u16) -> IVec2 {
	let projected_3d = vec.get_transformed_by_mat4x4_uniform(render_mat);
	let projected_2d = clip_space_to_screen_space(&projected_3d, wid, hei);
	projected_2d
}

fn render_uvec2_dbg(vec: &UVec2, pos: &UVec2, buf: &mut TerminalBuffer) {
	render_string(&format!("[{},{}]", vec.x, vec.y), pos, buf);
}

fn render_vec3_dbg(vec: &Vec3, pos: &UVec2, buf: &mut TerminalBuffer) {
	render_string(&format!("[{:+.2},{:+.2},{:+.2}]", vec.x, vec.y, vec.z), pos, buf);
}
