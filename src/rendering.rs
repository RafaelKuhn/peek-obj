pub mod mesh;
pub mod camera;

pub mod renderer;
pub mod yade_renderer;
pub mod obj_renderer;
pub mod primitives;
pub mod utils;

pub use primitives::*;
pub use utils::*;

use std::fmt;

use seeded_random::{Random, Seed};

use crate::{maths::*, camera::Camera, mesh::Mesh, benchmark::Benchmark, file_readers::yade_dem_reader::{Circ, YadeDemData}, terminal_wrapper::TerminalBuffer, timer::Timer, utils::*};


// ascii luminance:
// . , - ~ : ; = ! & # @
pub static BACKGROUND_FILL_CHAR: char = ' ';
// pub static BACKGROUND_FILL_CHAR: char = '⠥';

// static LUMIN: &str = " .,-~:;=!&#@";
// static DIRS: &str =
//   "↖ ↑ ↗" +
//   "← · →" +
//   "↙ ↓ ↘";

static FILL_CHAR: char = '@';
static YADE_WIRE_FILL_CHAR: char = '*';
const CIRCLE_FILL_CHAR: char = '@';

pub static ASCII_BYTES_PER_CHAR: usize = 1;
// pub static UTF32_BYTES_PER_CHAR: usize = 4;



#[derive(fmt::Debug)]
pub struct ScreenTriangle {
	pub p0: UVec2,
	pub p1: UVec2,
	pub p2: UVec2,
}

pub fn render_clear(buffer: &mut TerminalBuffer) {

	// TODO: figure out data structure to write braille (UTF32)
	// let def_pad = UTF32_BYTES_PER_CHAR - char::len_utf8(BACKGROUND_FILL_CHAR);

	for y in 0..buffer.hei {

		let y_offset = y as usize * buffer.wid as usize * ASCII_BYTES_PER_CHAR;
		for x in 0..buffer.wid {
			let it = y_offset + x as usize * ASCII_BYTES_PER_CHAR;

			// note: needs to fill [0, 0, 0, 0] because encode_utf8 only fills the required utf-8 leaving trash in there
			buffer.vec[it .. it+ASCII_BYTES_PER_CHAR].fill(0);

			// BACKGROUND_FILL_CHAR.encode_utf8(&mut buffer.vec[it + def_pad .. it+4]);
			BACKGROUND_FILL_CHAR.encode_utf8(&mut buffer.vec[it .. it+ASCII_BYTES_PER_CHAR]);
		}
	}
}


pub fn render_benchmark(benchmark: &Benchmark, camera: &Camera, buffer: &mut TerminalBuffer) {
	let mut highest_pos = UVec2::new(0, 0);
	render_string(&format!("cam pos: {:?}", camera.position), &highest_pos, buffer);
	highest_pos.y += 1;
	render_string(&format!("cam rot: {:?}", camera.rotation), &highest_pos, buffer);
	highest_pos.y += 2;
	render_string(&format!("cam sid: {:} m {:.4}", camera.side.inversed(), camera.side.magnitude()), &highest_pos, buffer);
	highest_pos.y += 1;
	render_string(&format!("cam  up: {:} m {:.4}", camera.up, camera.up.magnitude()), &highest_pos, buffer);
	highest_pos.y += 1;
	render_string(&format!("cam fwd: {:} m {:.4}", camera.forward.inversed(), camera.forward.magnitude()), &highest_pos, buffer);

	let mut lowest_pos = UVec2::new(0, buffer.hei - 1);

	let wxh = buffer.wid as u32 * buffer.hei as u32;
	let aspect = buffer.wid as f32 / buffer.hei as f32;

	render_string(&format!("w: {}, h: {}, w*h: {}, a: {:.2}", buffer.wid, buffer.hei, wxh, aspect), &lowest_pos, buffer);
	lowest_pos.y -= 1;
	render_string(&format!("frame n: {}", benchmark.total_frame_count), &lowest_pos, buffer);
	lowest_pos.y -= 1;
	render_string(&format!("scaled time: {:.2}", benchmark.time_aggr.as_millis() as f32 * 0.001), &lowest_pos, buffer);
	lowest_pos.y -= 1;
	render_string(&format!("time scale: {:.1}", benchmark.time_scale), &lowest_pos, buffer);
	lowest_pos.y -= 1;
	render_string(&format!("dt: {:.4}ms", benchmark.delta_time_millis), &lowest_pos, buffer);
	lowest_pos.y -= 1;
	render_string(&format!("fps: {:.2}", benchmark.fps), &lowest_pos, buffer);

}

pub const YADE_SCALE_TEMP: f32 = 15.0;

pub fn render_yade(yade_data: &YadeDemData, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	// TODO: figure out crappy camera
	let (pos_x, pos_y, pos_z) = (0.0, 1.0, 0.0);

	let speed = 0.5;
	let mut t = timer.time_aggr.as_millis() as f32 * 0.001 * speed;
	if buf.test { t = 0.0; }
	// horizontal
	// let t = (89_540) as f32 * 0.001;
	// exactly middle
	// let t = (0) as f32 * 0.001;

	// let t = 0.0;
	let (angle_x, angle_y, angle_z) = (0.0, t, 0.0);

	let (scale_x, scale_y, scale_z) = (YADE_SCALE_TEMP, YADE_SCALE_TEMP, YADE_SCALE_TEMP);

	buf.copy_projection_to_render_matrix();

	// IF VERBOSE
	// let mut y = 8;

	apply_identity_to_mat_4x4(&mut buf.transf_mat);

	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	// IF VERBOSE
	// render_string("+SCALE", &UVec2::new(2, y-1), buf);
	// render_mat_dbg(&buf.transf_mat.clone(), &UVec2::new(2, y), buf); y += 6;

	apply_rotation_to_mat_4x4_simple(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	// IF VERBOSE
	// render_string("+ROTATION", &UVec2::new(2, y-1), buf);
	// render_mat_dbg(&buf.transf_mat.clone(), &UVec2::new(2, y), buf); y += 6;

	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);
	// IF VERBOSE
	// render_string("+TRANSLATION", &UVec2::new(2, y-1), buf);
	// render_mat_dbg(&buf.transf_mat.clone(), &UVec2::new(2, y), buf); y += 6;

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);

	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);
	// let mut scale = build_identity_4x4();
	// apply_scale_to_mat_4x4(&mut scale, YADE_SCALE_TEMP, YADE_SCALE_TEMP, YADE_SCALE_TEMP);
	// multiply_4x4_matrices(&mut buf.render_mat, &scale);


	for tri in yade_data.tris.iter() {
		// break;

		let p0 = &tri.p0 + &tri.pos;
		let p1 = &tri.p1 + &tri.pos;
		let p2 = &tri.p2 + &tri.pos;

		let trs_p0 = p0.get_transformed_by_mat4x4_uniform(&buf.render_mat);
		let trs_p1 = p1.get_transformed_by_mat4x4_uniform(&buf.render_mat);
		let trs_p2 = p2.get_transformed_by_mat4x4_uniform(&buf.render_mat);

		let screen_p0 = clip_space_to_screen_space(&trs_p0, buf.wid, buf.hei);
		let screen_p1 = clip_space_to_screen_space(&trs_p1, buf.wid, buf.hei);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, buf.wid, buf.hei);

		render_bresenham_line(&screen_p0, &screen_p1, buf, YADE_WIRE_FILL_CHAR);
		render_bresenham_line(&screen_p1, &screen_p2, buf, YADE_WIRE_FILL_CHAR);
		render_bresenham_line(&screen_p2, &screen_p0, buf, YADE_WIRE_FILL_CHAR);
	}

	buf.clear_debug();
	buf.write_debug(&format!("w {}, h {}\n", buf.wid, buf.hei));


	// 253 // bottom
	// 251 // left
	// 61  // before thingy

	let mut indices_by_dist = Vec::<(f32, usize, char, IVec2)>::with_capacity(yade_data.balls.len());

	for (i, ball) in yade_data.balls.iter().enumerate() {
		let ball_pos = ball.pos.get_transformed_by_mat4x4_uniform(&buf.render_mat);
		let dist_circ_to_camera = ball_pos.squared_dist_to(&camera.position);

		let digit = i as u32 % ('Z' as u32 - 'A' as u32) + ('A' as u32);
		let letter = char::from_u32(digit).unwrap();

		let scr = ball_pos.clip_space_to_screen_space(buf.wid, buf.hei);
		// buf.write_debug(&format!("{:}: {:} proj {:}\n", i, ball.pos, ball_pos));
		indices_by_dist.push((dist_circ_to_camera, i, letter, scr));
	}

	indices_by_dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());


	for (i, (_, ball_ind, letter, _)) in indices_by_dist.iter().enumerate() {
		let letter = *letter;
		let ball_ind = *ball_ind;

		let ball = yade_data.balls.get(ball_ind).unwrap();

		// let digit = i as u32 % ('Z' as u32 - 'A' as u32) + ('A' as u32);
		// let letter = char::from_u32(digit).unwrap();

		render_sphere(&(ball.pos + Vec3::new(0.0, 1.0 / YADE_SCALE_TEMP, 0.0)), ball.rad, letter, buf, timer, camera);
	}

	for (i, (_, _, _, scr)) in indices_by_dist.iter().enumerate() {
		safe_render_char_signed(' ', scr.x, scr.y, buf);
	}


	return;
	for (i, ball) in yade_data.balls.iter().enumerate().skip(253).take(1) {

	// for (i, (_dist, ind)) in indices_by_dist.iter().take(1).chain(indices_by_dist.iter().rev().take(1)).enumerate() {
	// for (i, (_dist, ind)) in indices_by_dist.iter().rev().enumerate() {
	// for (i, (_dist, ind, letter)) in indices_by_dist.iter().enumerate().take(4) {
		// if i >= 1 { break; }

		let digit = i as u32 % ('Z' as u32 - 'A' as u32) + ('A' as u32);
		let letter = char::from_u32(digit).unwrap();

		// let ball = yade_data.balls.get(*ind).unwrap();

		// TODO: grab this from vec
		let ball_pos = ball.pos.get_transformed_by_mat4x4_uniform(&buf.render_mat);
		let ball_up = ball.pos.add_vec(&(&camera.up * ball.rad)).get_transformed_by_mat4x4_uniform(&buf.render_mat);
		// let ball_up = ball_pos.add_vec(&(&camera.up * ball.rad * YADE_SCALE_TEMP));

		buf.write_debug(&format!("{:}, r {:.6}, pos {:?} -> {:?} up {:?}\n", i, ball.rad, ball.pos, ball_pos, ball_up));

		let diff_3d = ball_pos.y - ball_up.y;


		let screen_circ = clip_space_to_screen_space(&ball_pos, buf.wid, buf.hei);
		let screen_up = clip_space_to_screen_space(&ball_up, buf.wid, buf.hei);

		let diff = screen_circ.y - screen_up.y;
		render_fill_bres_circle(&screen_circ, diff as f32, letter, buf);

		safe_render_char_signed('r', screen_circ.x, screen_circ.y, buf);
		// safe_render_char_signed('o', screen_up.x, screen_up.y, buf);

		// safe_render_string_signed(&format!("[{:}]", ind), screen_circ.x, screen_circ.y, buf);	
		// render_char('R', &screen_circ, buf);
	}
}


pub fn render_mesh(mesh: &Mesh, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	// let (pos_x, pos_y, pos_z) = (0.0, 0.0, 25.0);

	let start_ms = 89_340;
	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);
	// let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);

	let speed = 0.3;
	let sharpness = 2.5;

	let tri_wave = triangle_wave(t * speed);
	let t_smooth_wave = smoothed_0_to_1(tri_wave, sharpness);
	let tmod = lerp_f32(0.2, 0.4, t_smooth_wave);
	// let tmod = 1.0;
	let (scale_x, scale_y, scale_z) = (tmod, tmod, tmod);


	// TODO: remove these
	let start_ms = 89_340;
	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
	let (angle_x, angle_y, angle_z) = (0.0, t, 0.0);

	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);
	let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);
	let (scale_x, scale_y, scale_z) = (1.0, -1.0, 1.0);

	buf.copy_projection_to_render_matrix();

	apply_identity_to_mat_4x4(&mut buf.transf_mat);
	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4_simple(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);


	let num_tris = mesh.tris_indices.len() / 3;
	for tri_i in 0..num_tris {

		let p0_i = tri_i * 3 + 0;
		let p1_i = tri_i * 3 + 1;
		let p2_i = tri_i * 3 + 2;

		let p0 = mesh.get_vert_at(p0_i).invert_y();
		let p1 = mesh.get_vert_at(p1_i).invert_y();
		let p2 = mesh.get_vert_at(p2_i).invert_y();

		let trs_p0 = p0.get_transformed_by_mat4x4_uniform(&buf.render_mat);
		let trs_p1 = p1.get_transformed_by_mat4x4_uniform(&buf.render_mat);
		let trs_p2 = p2.get_transformed_by_mat4x4_uniform(&buf.render_mat);

		let screen_p0 = clip_space_to_screen_space(&trs_p0, buf.wid, buf.hei);
		let screen_p1 = clip_space_to_screen_space(&trs_p1, buf.wid, buf.hei);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, buf.wid, buf.hei);

		render_bresenham_line(&screen_p0, &screen_p1, buf, FILL_CHAR);
		render_bresenham_line(&screen_p1, &screen_p2, buf, FILL_CHAR);
		render_bresenham_line(&screen_p2, &screen_p0, buf, FILL_CHAR);
	}
}


fn render_mat_dbg(mat: &[f32], pos: &UVec2, buf: &mut TerminalBuffer) {
	let r0 = fmt_mat4_line(mat[ 0], mat[ 1], mat[ 2], mat[ 3]);
	render_string(&r0, pos, buf);

	let r1 = fmt_mat4_line(mat[ 4], mat[ 5], mat[ 6], mat[ 7]);
	render_string(&r1, &UVec2::new(pos.x, pos.y+1), buf);

	let r2 = fmt_mat4_line(mat[ 8], mat[ 9], mat[10], mat[11]);
	render_string(&r2, &UVec2::new(pos.x, pos.y+2), buf);

	let r3 = fmt_mat4_line(mat[12], mat[13], mat[14], mat[15]);
	render_string(&r3, &UVec2::new(pos.x, pos.y+3), buf);
}

pub fn render_axes(buf: &mut TerminalBuffer, camera: &Camera) {

	buf.copy_projection_to_render_matrix();

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);

	let origin  = screen_project(&Vec3::new(0.0, 0.0, 0.0), &buf.render_mat, buf.wid, buf.hei);

	// TODO: debug why this can get crappily signed values if AXIS_SZ_WORLD is > 500
	// TODO: this is already bugged at FUCKING 30
	// const AXIS_SZ_WORLD: f32 = 20.0;
	const AXIS_SZ_WORLD: f32 = 2.0;
	let up    = screen_project(&Vec3::new(0.0, AXIS_SZ_WORLD, 0.0), &buf.render_mat, buf.wid, buf.hei);
	let right = screen_project(&Vec3::new(AXIS_SZ_WORLD, 0.0, 0.0), &buf.render_mat, buf.wid, buf.hei);
	let front = screen_project(&Vec3::new(0.0, 0.0, AXIS_SZ_WORLD), &buf.render_mat, buf.wid, buf.hei);

	// let projected_3d = &Vec3::new(AXIS_SZ_WORLD, 0.0, 0.0).get_transformed_by_mat4x4(&buf.render_mat);
	// buf.write_debug(&format!("\nv: {:}\n", projected_3d));

	// let projected_orig = &Vec3::new(0.0, 0.0, 0.0).get_transformed_by_mat4x4(&buf.render_mat);
	// buf.write_debug(&format!("\norig: {:}\n", projected_orig));

	render_bresenham_line(&origin, &up, buf, '|');
	render_bresenham_line(&origin, &right, buf, '-');
	render_bresenham_line(&origin, &front, buf, '/');
}

pub fn render_gizmos(buf: &mut TerminalBuffer, camera: &Camera) {

	buf.copy_projection_to_render_matrix();

	const GIZMO_SIZE_WORLD: f32 = 0.15;

	// in world space, the gizmos is 8 units back (view matrix is irrelevant for these calculations)
	let base_world_space = Vec3::new(0.0, 0.0, -8.0);
	let origin = screen_project(&base_world_space, &buf.render_mat, buf.wid, buf.hei);
	let gizmos_side_reference_point = base_world_space.add_vec(&Vec3::new(GIZMO_SIZE_WORLD, 0.0, 0.0));
	let gizmos_side_reference_point_projected = screen_project(&gizmos_side_reference_point, &buf.render_mat, buf.wid, buf.hei);

	let side_offset = (gizmos_side_reference_point_projected.x - origin.x) as Int;
	let screen_offset = (
			buf.wid as Int / 2 -   side_offset       - 1,
		- ( buf.hei as Int / 2 - ( side_offset / 2 ) - 1 )
	);

	let origin_2d = origin.sum_t(screen_offset);

	let dbg_forward = camera.forward.inversed().with_y_inverted();
	let dbg_side = camera.side.inversed().with_y_inverted();
	let dbg_up = camera.up.with_y_inverted();

	let mut draw_between = |dir: &Vec3, ch: char| {
		let ptr = screen_project(&(base_world_space + (dir * GIZMO_SIZE_WORLD)), &buf.render_mat, buf.wid, buf.hei).sum_t(screen_offset);
		render_bresenham_line(&origin_2d, &ptr, buf, ch);
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

pub fn render_fill_bres_circle(pos: &IVec2, rad: f32, fill: char, buf: &mut TerminalBuffer) {
	let mut x = 0 as Int;
	let mut y = rad as Int;

	let (base_x, base_y) = (pos.x, pos.y);

	let mut d = 3 - 2 * (rad as Int);

	// I will always start rendering from the right side ->
	// and the first mirrored version will be the leftmost <-

	// buf.write_debug(&format!("[{:}, {:}]\n", base_x + x, base_y + y));
	// plot_mirrored_octets_safe(x, y, base_x, base_y, buf);

	let scaled_y = y * 2;

	let left_st = IVec2::new(base_x - scaled_y, base_y);
	let right_st = IVec2::new(base_x + scaled_y, base_y);
	render_bresenham_line(&left_st, &right_st, buf, fill);

	while y >= x {
		x += 1;
		if d > 0 {
			y -= 1;
			d = d + 4 * (x - y) + 10;
		} else {
			d = d + 4 * x + 6;
		}

		let scaled_x = x * 2;
		let scaled_y = y * 2;

		let left_0 = IVec2::new(base_x - scaled_x, base_y + y);
		let left_1 = IVec2::new(base_x - scaled_y, base_y + x);
		let left_2 = IVec2::new(base_x - scaled_y, base_y - x);
		let left_3 = IVec2::new(base_x - scaled_x, base_y - y);

		let right_0 = IVec2::new(base_x + scaled_x, base_y + y);
		let right_1 = IVec2::new(base_x + scaled_y, base_y + x);
		let right_2 = IVec2::new(base_x + scaled_y, base_y - x);
		let right_3 = IVec2::new(base_x + scaled_x, base_y - y);

		render_bresenham_line(&left_0, &right_0, buf, fill);
		render_bresenham_line(&left_1, &right_1, buf, fill);
		render_bresenham_line(&left_2, &right_2, buf, fill);
		render_bresenham_line(&left_3, &right_3, buf, fill);
	}
}

pub fn render_bres_circle(pos: &IVec2, rad: f32, buf: &mut TerminalBuffer) {

	let mut x = 0 as Int;
	let mut y = rad as Int;

	let (base_x, base_y) = (pos.x, pos.y);

	let mut d = 3 - 2 * (rad as Int);

	// I will always start rendering from the right side ->
	// and the first mirrored version will be the leftmost <-

	// buf.write_debug(&format!("[{:}, {:}]\n", base_x + x, base_y + y));
	plot_mirrored_octets_safe(x, y, base_x, base_y, buf);

	while y >= x {
		x += 1;
		if d > 0 {
			y -= 1;
			d = d + 4 * (x - y) + 10;
		} else {
			d = d + 4 * x + 6;
		}

		// buf.write_debug(&format!("[{:}, {:}]\n", base_x + x, base_y + y));

		plot_mirrored_octets_safe(x, y, base_x, base_y, buf);
	}
}

pub fn plot_mirrored_octets_safe(x: Int, y: Int, base_x: Int, base_y: Int, buf: &mut TerminalBuffer) {

	let scaled_x = x * 2;
	let scaled_y = y * 2;

	// left
	safe_render_char_signed(CIRCLE_FILL_CHAR, base_x - scaled_x, base_y + y, buf);
	safe_render_char_signed(CIRCLE_FILL_CHAR, base_x - scaled_y, base_y + x, buf);
	safe_render_char_signed(CIRCLE_FILL_CHAR, base_x - scaled_y, base_y - x, buf);
	safe_render_char_signed(CIRCLE_FILL_CHAR, base_x - scaled_x, base_y - y, buf);
	// right
	safe_render_char_signed(CIRCLE_FILL_CHAR, base_x + scaled_x, base_y + y, buf);
	safe_render_char_signed(CIRCLE_FILL_CHAR, base_x + scaled_y, base_y + x, buf);
	safe_render_char_signed(CIRCLE_FILL_CHAR, base_x + scaled_y, base_y - x, buf);
	safe_render_char_signed(CIRCLE_FILL_CHAR, base_x + scaled_x, base_y - y, buf);

	// safe_render_char_signed(CIRCLE_CHAR, base_x + scaled_x, base_y + y, buf);
	// safe_render_char_signed(CIRCLE_CHAR, base_x - scaled_x, base_y + y, buf);
	// safe_render_char_signed(CIRCLE_CHAR, base_x + scaled_x, base_y - y, buf);
	// safe_render_char_signed(CIRCLE_CHAR, base_x - scaled_x, base_y - y, buf);

	// safe_render_char_signed(CIRCLE_CHAR, base_x + scaled_y, base_y + x, buf);
	// safe_render_char_signed(CIRCLE_CHAR, base_x + scaled_y, base_y - x, buf);
	// safe_render_char_signed(CIRCLE_CHAR, base_x - scaled_y, base_y + x, buf);
	// safe_render_char_signed(CIRCLE_CHAR, base_x - scaled_y, base_y - x, buf);
}
