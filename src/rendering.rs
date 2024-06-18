pub mod mesh;
pub mod camera;

pub mod renderer;
pub mod yade_renderer;
pub mod obj_renderer;
pub mod primitives;
pub mod utils;
pub mod bounding_box;
pub mod culling;
pub mod render_settings;
pub mod ball_painter;
pub mod help_screen;

pub use primitives::*;
pub use utils::*;
pub use culling::*;
pub use bounding_box::*;
pub use render_settings::*;
pub use ball_painter::*;



use help_screen::HELP_SCR;

use crate::{app::App, camera::Camera, fps_measure::FpsMeasure, maths::*, terminal::TerminalBuffer, timer::Timer, utils::*};

use self::cull_mode::CullMode;


// ascii luminance:
// . , - ~ : ; = ! & # @
pub static BACKGROUND_FILL_CHAR: char = ' ';
// pub static BACKGROUND_FILL_CHAR: char = '⠥';

// static LUMIN: &str = " .,-~:;=!&#@";
// static DIRS: &str =
//   "↖ ↑ ↗" +
//   "← · →" +
//   "↙ ↓ ↘";

const BALL_FILL_CHAR: char = '@';

// this is just here to remember me that to render braille each char needs to have 3 bytes
pub static ASCII_BYTES_PER_CHAR: usize = 1;


pub fn render_clear(buffer: &mut TerminalBuffer) {
	
	debug_assert!(BACKGROUND_FILL_CHAR.len_utf8() == 1, "Background fill should be ASCII");

	buffer.raw_ascii_screen.fill(BACKGROUND_FILL_CHAR as u8);


	// only needs to care about this for braille rendering
	// for y in 0..buffer.hei {

	// 	let y_offset = y as usize * buffer.wid as usize * ASCII_BYTES_PER_CHAR;
	// 	for x in 0..buffer.wid {
	// 		let it = y_offset + x as usize * ASCII_BYTES_PER_CHAR;

	// 		// note: needs to fill [0, 0, 0, 0] because encode_utf8 only fills the required utf-8 leaving trash in there
	// 		buffer.vec[it .. it+ASCII_BYTES_PER_CHAR].fill(0);

	// 		// BACKGROUND_FILL_CHAR.encode_utf8(&mut buffer.vec[it + def_pad .. it+4]);
	// 		BACKGROUND_FILL_CHAR.encode_utf8(&mut buffer.vec[it .. it+ASCII_BYTES_PER_CHAR]);
	// 	}
	// }
}


pub fn render_verbose(fps_measure: &FpsMeasure, camera: &Camera, app: &mut App) {

	const PAUSED_STR: &str = " ENGINE RUNNING! ";
	render_string_snap_right(PAUSED_STR, &UVec2::new(0, app.buf.hei - 1), &mut app.buf);

	if !app.is_verbose { return }

	let is_free_mov = app.is_free_mov();
	let buf = &mut app.buf;

	let mut highest_pos = UVec2::new(0, 0);

	#[cfg(debug_assertions)]
	render_string(&format!("cam pos: {:?} ", camera.position), &highest_pos, buf);
	#[cfg(not(debug_assertions))]
	render_string(&format!("cam pos: {:} ", camera.position), &highest_pos, buf);

	highest_pos.y += 1;
	#[cfg(debug_assertions)]
	render_string(&format!("cam rot: {:?} ", camera.rotation), &highest_pos, buf);
	#[cfg(not(debug_assertions))]
	render_string(&format!("cam rot: {:} ", camera.rotation.rad_to_deg_pretty()), &highest_pos, buf);

	#[cfg(debug_assertions)] {
		highest_pos.y += 2;
		render_string(&format!("cam sid: {:} m {:.4} ", camera.side.inversed(), camera.side.magnitude()), &highest_pos, buf);
		highest_pos.y += 1;
		render_string(&format!("cam  up: {:} m {:.4} ", camera.up, camera.up.magnitude()), &highest_pos, buf);
		highest_pos.y += 1;
		render_string(&format!("cam fwd: {:} m {:.4} ", camera.forward.inversed(), camera.forward.magnitude()), &highest_pos, buf);
	}


	let mut lowest_pos_bl = UVec2::new(0, buf.hei - 1);

	let wxh = buf.wid as u32 * buf.hei as u32;
	let aspect = buf.wid as f32 / buf.hei as f32;
	render_string(&format!("w: {}, h: {}, w*h: {}, a: {:.2} ", buf.wid, buf.hei, wxh, aspect), &lowest_pos_bl, buf);
	lowest_pos_bl.y -= 1;
	render_string(&format!("frame n: {} ", fps_measure.total_frame_count), &lowest_pos_bl, buf);
	lowest_pos_bl.y -= 1;
	render_string(&format!("scaled time: {:.2} ", fps_measure.time_aggr.as_millis() as f32 * 0.001), &lowest_pos_bl, buf);
	lowest_pos_bl.y -= 1;
	render_string(&format!("time scale: {:.1} ", fps_measure.time_scale), &lowest_pos_bl, buf);
	lowest_pos_bl.y -= 1;
	render_string(&format!("dt: {:.4}ms ", fps_measure.delta_time_millis), &lowest_pos_bl, buf);
	lowest_pos_bl.y -= 1;
	render_string(&format!("fps: {:.2} ", fps_measure.fps), &lowest_pos_bl, buf);


	let mut lowest_pos_br = UVec2::new(0, buf.hei - 2);

	render_string_snap_right(&format!(" z sort mode: {:} ", buf.get_sorting_mode()), &lowest_pos_br, buf);
	lowest_pos_br.y -= 1;
	render_string_snap_right(&format!(" cull mode: {:} ", buf.get_cull_mode()), &lowest_pos_br, buf);
	lowest_pos_br.y -= 1;
	render_string_snap_right(&format!(" move mode: {:} ", if is_free_mov { "free movement" } else { "orbital" }), &lowest_pos_br, buf);
	lowest_pos_br.y -= 1;
	render_string_snap_right(&format!(" light mode: {:} ", buf.get_ball_fill_mode()), &lowest_pos_br, buf);

	let gizmos_mode = buf.get_gizmos_mode();
	if let GizmosType::WorldAxes = gizmos_mode {
		lowest_pos_br.y -= 1;
		render_string_snap_right(&format!(" gizmos: {:} ", gizmos_mode), &lowest_pos_br, buf);
	}

	let help_txt = " PRESS H FOR HELP ";
	let center = UVec2::new(buf.wid / 2 - help_txt.len() as u16 / 2, 0);
	render_string(&help_txt, &center, buf);

	// indices
	// TODO: DEBUG OPTION draw indices
	// for num in 0..buf.hei {
	// 	render_string(&format!("{}", num), &UVec2::new(0, num), buf);
	// 	render_string_snap_right(&format!("{}", num), &UVec2::new(0, num), buf);
	// }
}

pub fn render_help_screen(buf: &mut TerminalBuffer) {

	let mut max_line = 0;
	let mut cur_max_line = 0;
	for &ch in HELP_SCR {
		if ch == b'\n' {
			max_line = max_line.max(cur_max_line);
			cur_max_line = 0;
			continue;
		}

		cur_max_line += 1;
	}

	let mut x = buf.wid/2 - max_line/2;
	let mut y = 0;
	for &ch in HELP_SCR {

		if ch == b'\t' {
			x += 4;
			continue;
		}

		if ch == b'\n' {
			y += 1;
			x = buf.wid/2 - max_line/2;
			continue;
		}

		if x < buf.wid && y < buf.hei {
			buf.raw_ascii_screen[xy_to_it(x, y, buf.wid)] = ch;
		}

		x += 1;
	}

	let help_txt = "PRESS H TO QUIT HELP";
	let center = UVec2::new(buf.wid / 2 - help_txt.len() as u16 / 2, 0);
	render_string(&help_txt, &center, buf);

	let help_txt = "KEYBINDINGS";
	let center = UVec2::new(buf.wid / 2 - help_txt.len() as u16 / 2, 2);
	render_string(&help_txt, &center, buf);

	let help_txt = "HELP SCREEN!";
	let center = UVec2::new(buf.wid - help_txt.len() as u16 - 1, buf.hei - 1);
	render_string(&help_txt, &center, buf);
}


pub fn render_string_snap_right(string: &str, pos: &UVec2, buf: &mut TerminalBuffer) {
	let new_pos = UVec2::new(buf.wid - string.len() as u16 - pos.x, pos.y);
	render_string(string, &new_pos, buf);
}

pub fn render_mat_dbg(mat: &[f32], pos: &UVec2, buf: &mut TerminalBuffer) {
	let r0 = fmt_mat4_line(mat[ 0], mat[ 1], mat[ 2], mat[ 3]);
	render_string(&r0, pos, buf);

	let r1 = fmt_mat4_line(mat[ 4], mat[ 5], mat[ 6], mat[ 7]);
	render_string(&r1, &UVec2::new(pos.x, pos.y+1), buf);

	let r2 = fmt_mat4_line(mat[ 8], mat[ 9], mat[10], mat[11]);
	render_string(&r2, &UVec2::new(pos.x, pos.y+2), buf);

	let r3 = fmt_mat4_line(mat[12], mat[13], mat[14], mat[15]);
	render_string(&r3, &UVec2::new(pos.x, pos.y+3), buf);
}

// TODO: FIX THIS SHIT
pub fn render_axes(axis_size: f32, render_marks: bool, camera: &Camera, buf: &mut TerminalBuffer) {

	if let GizmosType::None = buf.get_gizmos_mode() { return }

	buf.write_debug("rendering axes\n");

	buf.copy_projection_to_render_matrix();
	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);

	// let origin  = screen_project(&Vec3::new(0.0, 0.0, 0.0), &buf.render_mat, buf.wid, buf.hei);
	let origin = &Vec3::new(0.0, 0.0, 0.0);

	// TODO: debug why this can get crappily signed values if AXIS_SZ_WORLD is > 500
	// TODO: this is already bugged at FUCKING 30
	// const AXIS_SZ_WORLD: f32 = 20.0;

	// TODO: cull and render line
	let up    = cull_line_into_screen_space(&origin, &Vec3::new(0.0, axis_size, 0.0), camera, buf);
	let right = cull_line_into_screen_space(&origin, &Vec3::new(axis_size, 0.0, 0.0), camera, buf);
	let front = cull_line_into_screen_space(&origin, &Vec3::new(0.0, 0.0, axis_size), camera, buf);

	if let Some(up_line) = up {
		render_bresenham_line(&up_line.p0, &up_line.p1, buf, '|');
	}
	if let Some(right_line) = right {
		render_bresenham_line(&right_line.p0, &right_line.p1, buf, '-');
	}
	if let Some(front_line) = front {
		render_bresenham_line(&front_line.p0, &front_line.p1, buf, '/');
	}

	// let up    = screen_project(&Vec3::new(0.0, AXIS_SZ_WORLD, 0.0), &buf.render_mat, buf.wid, buf.hei);
	// let right = screen_project(&Vec3::new(AXIS_SZ_WORLD, 0.0, 0.0), &buf.render_mat, buf.wid, buf.hei);
	// let front = screen_project(&Vec3::new(0.0, 0.0, AXIS_SZ_WORLD), &buf.render_mat, buf.wid, buf.hei);

	// render_bresenham_line(&origin, &up, buf, '|');
	// render_bresenham_line(&origin, &right, buf, '-');
	// render_bresenham_line(&origin, &front, buf, '/');
	
	if !render_marks { return }

	let offset = 0.1;
	for i in 0..3 {
		let i = (i + 1) as f32;
		let marker_x = screen_project(&Vec3::new(i, 0.0, 0.0), &buf.render_mat, buf.wid, buf.hei);
		let marker_x0 = screen_project(&Vec3::new(i, offset, -offset), &buf.render_mat, buf.wid, buf.hei);
		let marker_x1 = screen_project(&Vec3::new(i, -offset, offset), &buf.render_mat, buf.wid, buf.hei);

		render_bresenham_line(&marker_x0, &marker_x, buf, '/');
		render_bresenham_line(&marker_x, &marker_x1, buf, '/');

		safe_render_string_signed(&format!("{:.1}", i), marker_x.x - 1, marker_x.y, buf);

		let marker_z = screen_project(&Vec3::new(0.0, 0.0, i), &buf.render_mat, buf.wid, buf.hei);
		let marker_z0 = screen_project(&Vec3::new(-offset, offset, i), &buf.render_mat, buf.wid, buf.hei);
		let marker_z1 = screen_project(&Vec3::new(offset, -offset,i), &buf.render_mat, buf.wid, buf.hei);

		render_bresenham_line(&marker_z0, &marker_z, buf, '\\');
		render_bresenham_line(&marker_z, &marker_z1, buf, '\\');

		safe_render_string_signed(&format!("{:.1}", i), marker_z.x - 1, marker_z.y, buf);

		let marker_y = screen_project(&Vec3::new(0.0, i, 0.0), &buf.render_mat, buf.wid, buf.hei);
		let marker_y0 = screen_project(&Vec3::new(-offset, i, offset), &buf.render_mat, buf.wid, buf.hei);
		let marker_y1 = screen_project(&Vec3::new(offset, i, -offset), &buf.render_mat, buf.wid, buf.hei);

		render_bresenham_line(&marker_y0, &marker_y, buf, '\\');
		render_bresenham_line(&marker_y, &marker_y1, buf, '\\');

		safe_render_string_signed(&format!("{:.1}", i), marker_y.x - 1, marker_y.y, buf);	
	}	
}

pub fn render_orientation(buf: &mut TerminalBuffer, camera: &Camera) {

	buf.copy_projection_to_render_matrix();

	const GIZMO_SIZE_WORLD: f32 = 0.15;

	// in world space, the gizmos is 8 units back (view matrix is irrelevant for these calculations)
	let base_world_space = Vec3::new(0.0, 0.0, -8.0);
	let origin = screen_project_f(&base_world_space, &buf.render_mat, buf.wid, buf.hei);
	let gizmos_side_reference_point = base_world_space.added_vec(&Vec3::new(GIZMO_SIZE_WORLD, 0.0, 0.0));
	let gizmos_side_reference_point_projected = screen_project_f(&gizmos_side_reference_point, &buf.render_mat, buf.wid, buf.hei);

	let side_offset = (gizmos_side_reference_point_projected.x - origin.x) as Float;

	let screen_offset = FVec2::new(
			buf.wid as Float / 2.0 -   side_offset       - 1.0,
		- ( buf.hei as Float / 2.0 - ( side_offset / 2.0 ) - 1.0 )
	);

	let origin_2d = origin.sum(&screen_offset);

	let dbg_forward = camera.forward.with_y_inverted();
	let dbg_side = camera.side.with_y_inverted();
	let dbg_up = camera.up.with_y_inverted();

	let mut draw_between = |dir: &Vec3, ch: char| {
		let ptr = screen_project_f(&(base_world_space + (dir * GIZMO_SIZE_WORLD)), &buf.render_mat, buf.wid, buf.hei).sum(&screen_offset).round_into_ivec2();
		render_bresenham_line(&origin_2d.round_into_ivec2(), &ptr, buf, ch);
		render_char('O', &ptr.into(), buf);
	};


	let dot_x = Vec3::dot_product(&Vec3::side(), &dbg_side);
	if dot_x > 0.0 {
		draw_between(&dbg_side, 'x');
	}

	let dot_y = Vec3::dot_product(&Vec3::up(), &dbg_up);
	if dot_y > 0.0 {
		draw_between(&dbg_up, 'y');
	}

	let dot_z = Vec3::dot_product(&Vec3::forward(), &dbg_forward);
	if dot_z > 0.0 {
		draw_between(&dbg_forward, 'z');
	}

	if dot_x <= 0.0 {
		draw_between(&dbg_side, 'X');
	}

	if dot_y <= 0.0 {
		draw_between(&dbg_up, 'Y');
	}

	if dot_z <= 0.0 {
		draw_between(&dbg_forward, 'Z');
	}

	render_char('O', &origin_2d.into(), buf);
}

pub fn render_bounding_box(bbox: &BoundingBox, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);

	let start_ms = 0;
	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001 * 0.5;
	// teapot weirded out thing
	// let start_ms = 95_374;
	// let t = (start_ms) as f32 * 0.001;
	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);
	let (angle_x, angle_y, angle_z) = (0.0, t, 0.0);
	// let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);

	let speed = 0.3;
	let sharpness = 2.5;

	let tri_wave = triangle_wave(t * speed);
	let t_smooth_wave = smoothed_0_to_1_s(tri_wave, sharpness);
	let tmod = lerp_f32(0.5, 1.0, t_smooth_wave);
	let (scale_x, scale_y, scale_z) = (tmod, tmod, tmod);
	let (scale_x, scale_y, scale_z) = (1.0, 1.0, 1.0);

	buf.copy_projection_to_render_matrix();

	apply_identity_to_mat_4x4(&mut buf.transf_mat);
	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);

	// buf.clear_debug();
	// buf.write_debug("BOUNDING BOX:\n");
	// buf.write_debug(&format!("{} top_right_front\n", bbox.top_right_front));
	// buf.write_debug(&format!("{} top_right_back\n", bbox.top_right_back));
	// buf.write_debug(&format!("{} top_left_back\n", bbox.top_left_back));
	// buf.write_debug(&format!("{} top_left_front\n", bbox.top_left_front));
	// buf.write_debug(&format!("{} bottom_right_front\n", bbox.bottom_right_front));
	// buf.write_debug(&format!("{} bottom_right_back\n", bbox.bottom_right_back));
	// buf.write_debug(&format!("{} bottom_left_back\n", bbox.bottom_left_back));
	// buf.write_debug(&format!("{} bottom_left_front\n", bbox.bottom_left_front));

	let top_right_front_2d = screen_project(&bbox.top_right_front, &buf.render_mat, buf.wid, buf.hei);
	let top_right_back_2d  = screen_project(&bbox.top_right_back, &buf.render_mat, buf.wid, buf.hei);
	let top_left_back_2d   = screen_project(&bbox.top_left_back, &buf.render_mat, buf.wid, buf.hei);
	let top_left_front_2d  = screen_project(&bbox.top_left_front, &buf.render_mat, buf.wid, buf.hei);
	let bottom_right_front_2d = screen_project(&bbox.bottom_right_front, &buf.render_mat, buf.wid, buf.hei);
	let bottom_right_back_2d  = screen_project(&bbox.bottom_right_back, &buf.render_mat, buf.wid, buf.hei);
	let bottom_left_back_2d   = screen_project(&bbox.bottom_left_back, &buf.render_mat, buf.wid, buf.hei);
	let bottom_left_front_2d  = screen_project(&bbox.bottom_left_front, &buf.render_mat, buf.wid, buf.hei);

	render_bresenham_line(&top_right_front_2d, &top_right_back_2d, buf, '/');
	render_bresenham_line(&top_right_back_2d, &top_left_back_2d, buf, '-');
	render_bresenham_line(&top_left_back_2d, &top_left_front_2d, buf, '/');
	render_bresenham_line(&top_left_front_2d, &top_right_front_2d, buf, '-');

	render_bresenham_line(&bottom_right_front_2d, &bottom_right_back_2d, buf, '/');
	render_bresenham_line(&bottom_right_back_2d, &bottom_left_back_2d, buf, '-');
	render_bresenham_line(&bottom_left_back_2d, &bottom_left_front_2d, buf, '/');
	render_bresenham_line(&bottom_left_front_2d, &bottom_right_front_2d, buf, '-');

	render_bresenham_line(&bottom_right_front_2d, &top_right_front_2d, buf, '|');
	render_bresenham_line(&bottom_right_back_2d, &top_right_back_2d, buf, '|');
	render_bresenham_line(&bottom_left_back_2d, &top_left_back_2d, buf, '|');
	render_bresenham_line(&bottom_left_front_2d, &top_left_front_2d, buf, '|');
}

pub fn render_test(camera: &mut Camera, app: &mut App) {
	if app.is_free_mov() { return }
	// else { return }

	todo!("make some sort of struct and render it to debug 'rotate around arbitrary axis'");

	let buf = &mut app.buf;

	let ang_x = app.user_dir.y;
	let ang_y = app.user_dir.x;

	camera.cache_rot_x += ang_x;
	camera.cache_rot_y += ang_y;
	camera.cache_dist += app.user_dir.z;

	let mut debug_pos = UVec2::new(0, 3);
	render_string(&format!("ang x {:.2}, y {:.2}, deg x {:.2} y {:.2}", camera.cache_rot_x, camera.cache_rot_y, rad_to_deg(camera.cache_rot_x), rad_to_deg(camera.cache_rot_y)), &debug_pos, buf);

	// TODO: camera initial position
	// let base_pos = Vec3::new(0., 0., 16. + camera.cache_dist);
	// let base_pos = Vec3::new(0., 0., 2. + camera.cache_dist);
	let base_pos = Vec3::new(0.0, 0.0, 0.5 + camera.cache_dist);

	apply_identity_to_mat_4x4(&mut buf.transf_mat);
	apply_rotation_to_mat_4x4(&mut buf.transf_mat, camera.cache_rot_x, camera.cache_rot_y, 0.0);

	buf.copy_projection_to_render_matrix();

	let view_pos = base_pos
	.rotated_x(camera.cache_rot_x)
	.rotated_y(camera.cache_rot_y)
	.get_transformed_by_mat4x4_discard_w(&camera.view_matrix)
	;

	let pos = screen_project(&view_pos, &buf.render_mat, buf.wid, buf.hei);

	let target = Vec3::zero();
	buf.copy_projection_to_render_matrix();
	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	let target_scr = screen_project(&target, &buf.render_mat, buf.wid, buf.hei);

	render_bresenham_line(&pos, &target_scr, buf, '*');

	safe_render_char_i('@', &pos, buf);
	safe_render_char_i('@', &target_scr, buf);
}