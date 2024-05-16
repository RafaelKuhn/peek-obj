pub mod mesh;
pub mod camera;

pub mod renderer;
pub mod yade_renderer;
pub mod obj_renderer;
pub mod primitives;
pub mod utils;
pub mod bounding_box;
pub mod z_sorting_mode;
pub mod cull_mode;

pub use primitives::*;
pub use bounding_box::*;
pub use utils::*;
pub use z_sorting_mode::*;

use std::{cmp::Ordering, fmt};



use crate::{app::App, camera::Camera, file_readers::yade_dem_reader::{Tri, YadeDemData}, fps_measure::FpsMeasure, maths::*, mesh::Mesh, terminal::TerminalBuffer, timer::Timer, utils::*};

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

const FILL_CHAR: char = '@';
const YADE_WIRE_FILL_CHAR: char = '*';
const BALL_FILL_CHAR: char = '@';

pub static ASCII_BYTES_PER_CHAR: usize = 1;



#[derive(fmt::Debug)]
pub struct ScreenTriangle {
	pub p0: UVec2,
	pub p1: UVec2,
	pub p2: UVec2,
}

pub fn render_clear(buffer: &mut TerminalBuffer) {
	
	debug_assert!(BACKGROUND_FILL_CHAR.len_utf8() == 1, "Background fill should be ASCII");

	buffer.raw_ascii_screen.fill(BACKGROUND_FILL_CHAR as u8);
	// buffer.last_frame_vec.fill(BACKGROUND_FILL_CHAR as u8);


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


pub fn render_verbose(benchmark: &FpsMeasure, camera: &Camera, app: &mut App) {

	let is_free_mov = app.is_free_mov();
	let buf = &mut app.buf;

	let mut highest_pos = UVec2::new(0, 0);

	render_string(&format!("cam pos: {:} ", camera.position), &highest_pos, buf);
	highest_pos.y += 1;
	render_string(&format!("cam rot: {:} ", camera.rotation.rad_to_deg()), &highest_pos, buf);

	// highest_pos.y += 2;
	// render_string(&format!("cam sid: {:} m {:.4}", camera.side.inversed(), camera.side.magnitude()), &highest_pos, buffer);
	// highest_pos.y += 1;
	// render_string(&format!("cam  up: {:} m {:.4}", camera.up, camera.up.magnitude()), &highest_pos, buffer);
	// highest_pos.y += 1;
	// render_string(&format!("cam fwd: {:} m {:.4}", camera.forward.inversed(), camera.forward.magnitude()), &highest_pos, buffer);


	let mut lowest_pos = UVec2::new(0, buf.hei - 1);

	let wxh = buf.wid as u32 * buf.hei as u32;
	let aspect = buf.wid as f32 / buf.hei as f32;
	render_string(&format!("w: {}, h: {}, w*h: {}, a: {:.2} ", buf.wid, buf.hei, wxh, aspect), &lowest_pos, buf);
	lowest_pos.y -= 1;
	render_string(&format!("frame n: {} ", benchmark.total_frame_count), &lowest_pos, buf);
	lowest_pos.y -= 1;
	render_string(&format!("scaled time: {:.2} ", benchmark.time_aggr.as_millis() as f32 * 0.001), &lowest_pos, buf);
	lowest_pos.y -= 1;
	render_string(&format!("time scale: {:.1} ", benchmark.time_scale), &lowest_pos, buf);
	lowest_pos.y -= 1;
	render_string(&format!("dt: {:.4}ms ", benchmark.delta_time_millis), &lowest_pos, buf);
	lowest_pos.y -= 1;
	render_string(&format!("fps: {:.2} ", benchmark.fps), &lowest_pos, buf);


	let mut br_lowest_pos = UVec2::new(0, buf.hei - 2);

	render_string_snap_right(&format!(" z sort mode: {:} ", buf.get_sorting_mode()), &br_lowest_pos, buf);
	br_lowest_pos.y -= 1;
	render_string_snap_right(&format!(" cull mode: {:} ", buf.get_cull_mode()), &br_lowest_pos, buf);
	br_lowest_pos.y -= 1;
	render_string_snap_right(&format!(" move mode: {:} ", if is_free_mov { "free movement" } else { "orbital" }), &br_lowest_pos, buf);
}

pub fn render_string_snap_right(string: &str, pos: &UVec2, buf: &mut TerminalBuffer) {
	let new_pos = UVec2::new(buf.wid - string.len() as u16 - pos.x, pos.y);
	render_string(string, &new_pos, buf);
}

#[deprecated(since="0.0", note=r#"Call "render_yade_sorted", keeping this until I code Transform"#)]
pub fn render_yade(yade_data: &YadeDemData, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	todo!("figure standard render yade");
	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);

	let speed = 0.5;
	let start_ms = 0;
	let start_ms = 5196;
	let mut t = (timer.time_aggr.as_millis() * 0 + start_ms) as f32 * 0.001 * speed;
	if buf.test { t = 0.0; }

	// let t = 0.0;
	let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);
	let (angle_x, angle_y, angle_z) = (0.0, t, 0.0);
	// let (angle_x, angle_y, angle_z) = (t, 0.0, 0.0);
	// let (angle_x, angle_y, angle_z) = (t * 0.3, t, t * 2.1);

	let scale = 1.0;
	let (scale_x, scale_y, scale_z) = (scale, scale, scale);

	// // Funky zoom scale animation
	// let speed = 0.5;
	// let tmod = ((t * speed % 1.0) - 0.5).abs() * 2.0;
	// // // render_string(&format!("{}", tmod), &UVec2::new(0, 7), buf);
	// let animation_curve = smoothed_0_to_1_s(tmod, 4.0) * 0.5 + 0.25;
	// let scale = animation_curve;
	// let (scale_x, scale_y, scale_z) = (animation_curve, animation_curve, animation_curve);


	/* */ // let mut bench = Benchmark::named(" RENDER");
	/* */ // bench.start();
	buf.copy_projection_to_render_matrix();

	apply_identity_to_mat_4x4(&mut buf.transf_mat);

	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);
	/* */ // bench.end_and_log("setup", buf);

	for tri in yade_data.tris.iter() {

		let clip_p0 = tri.p0.get_transformed_by_mat4x4_w(&buf.render_mat);
		let clip_p1 = tri.p1.get_transformed_by_mat4x4_w(&buf.render_mat);
		let clip_p2 = tri.p2.get_transformed_by_mat4x4_w(&buf.render_mat);

		// buf.write_debug(&format!("p0 {:?} x {} y {}\n", clip_p0, clip_p0.x_in_w_range(), clip_p0.y_in_w_range()));
		// buf.write_debug(&format!("p1 {:?} x {} y {}\n", clip_p1, clip_p1.x_in_w_range(), clip_p1.y_in_w_range()));
		// buf.write_debug(&format!("p2 {:?} x {} y {}\n\n", clip_p2, clip_p2.x_in_w_range(), clip_p2.y_in_w_range()));

		let Some(screen_tri) = cull_tri_into_screen_space(clip_p0, clip_p1, clip_p2, buf) else { continue };

		render_bresenham_line(&screen_tri.p0, &screen_tri.p1, buf, YADE_WIRE_FILL_CHAR);
		render_bresenham_line(&screen_tri.p1, &screen_tri.p2, buf, YADE_WIRE_FILL_CHAR);
		render_bresenham_line(&screen_tri.p2, &screen_tri.p0, buf, YADE_WIRE_FILL_CHAR);
	}
	/* */ // bench.end_and_log("render tris", buf);

	// TODO: could make a buffer in TerminalBuffer for this
	let mut render_mat_without_transform = create_identity_4x4_arr();
	buf.copy_projection_to_mat4x4(&mut render_mat_without_transform);
	multiply_4x4_matrices(&mut render_mat_without_transform, &camera.view_matrix);

	// TODO: see how much data is copied by sorting this
	let mut indices_by_dist = Vec::<RenderBallData>::with_capacity(yade_data.balls.len());

	/* */ // bench.start();
	for (index, ball) in yade_data.balls.iter().enumerate() {

		let rad_scaled = ball.rad * scale;
		let clip_pos = ball.pos.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);

		let transformed_pos = ball.pos.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);

		// culling balls behind the camera
		let ball_to_cam = camera.position - transformed_pos;
		let dot = Vec3::dot_product(&camera.forward, &ball_to_cam);
		if dot < 0.0 { continue }

		let trs_pos_sd = transformed_pos.add_vec(&(camera.side * rad_scaled));
		let trs_pos_sd_proj = trs_pos_sd.get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform);
		let trs_pos_sd_projected_screen_f = clip_space_to_screen_space_f(&trs_pos_sd_proj, buf.wid, buf.hei);

		let screen_pos_f32 = clip_space_to_screen_space_f(&clip_pos, buf.wid, buf.hei);
		let rad = (trs_pos_sd_projected_screen_f.x - screen_pos_f32.x).abs();
		// buf.write_debug(&format!("{:?} scr {:?}\n", trs_pos_sd_projected_screen_f, screen_pos_f32));

		if cull_circle(&screen_pos_f32, rad, buf) { continue; }

		let dist_sq_to_camera = transformed_pos.squared_dist_to(&camera.position);

		let render_data = RenderBallData {
			rad,
			screen_pos: screen_pos_f32.into(),
			index,
		};

		indices_by_dist.push(render_data);
	}
	/* */ // bench.end_and_log("set up render balls", buf);
	
	// TODO: fix this version
	// indices_by_dist.sort_by(|a, b| b.dist_sq_to_camera.partial_cmp(&a.dist_sq_to_camera).unwrap());

	/* */ // bench.end_and_log("sort render balls", buf);
	for ball_data in indices_by_dist.iter() {

		let digit = ball_data.index as u32 % ('Z' as u32 - 'A' as u32) + ('A' as u32);
		let letter = char::from_u32(digit).unwrap();

		render_fill_bres_circle(&ball_data.screen_pos, ball_data.rad, letter, buf);
		// render_bres_circle(&ball_data.screen_pos, ball_data.rad, letter, buf);

		// // Renders chars at center, up and side of each sphere
		// safe_render_char_at('O', ball_data.screen_pos.x, ball_data.screen_pos.y, buf);
		// let transformed_pos = yade_data.balls[ball_data.index].pos.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
		// let trs_pos_sd_projected_screen_f = clip_space_to_screen_space_f(&transformed_pos.add_vec(&(camera.up * yade_data.balls[ball_data.index].rad * scale)).get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform), buf.wid, buf.hei);
		// safe_render_char_at('U', trs_pos_sd_projected_screen_f.x as i32, trs_pos_sd_projected_screen_f.y as i32, buf);
		// let trs_pos_sd_projected_screen_f = clip_space_to_screen_space_f(&transformed_pos.add_vec(&(camera.side * yade_data.balls[ball_data.index].rad * scale)).get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform), buf.wid, buf.hei);
		// safe_render_char_at('S', trs_pos_sd_projected_screen_f.x as i32, trs_pos_sd_projected_screen_f.y as i32, buf);
	}
	/* */ // bench.end_and_log("fill ball circle", buf);
}

pub struct RenderBallData {
	// dist_sq_to_camera: f32,
	index: usize,
	screen_pos: IVec2,
	rad: f32,
}

pub enum Primitive {
	Ball(RenderBallData),
	Line(IVec2, IVec2),
	// Tri(ScreenTri),
}

pub fn render_yade_sorted(yade_data: &YadeDemData, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);

	let speed = 0.5;
	let start_ms = 0;
	let mut t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001 * speed;
	if buf.test { t = 0.0; }

	let (angle_x, angle_y, angle_z) = (0.0, t, 0.0);

	let scale = 1.0;
	let (scale_x, scale_y, scale_z) = (scale, scale, scale);

	buf.copy_projection_to_render_matrix();

	apply_identity_to_mat_4x4(&mut buf.transf_mat);

	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);


	// TODO: see how much data is copied by sorting this
	// might need a type for distance sorting
	let mut render_data_by_dist = Vec::<(f32, Primitive)>::with_capacity(yade_data.balls.len());


	// could make a buffer in TerminalBuffer for this
	let mut render_mat_without_transform = create_identity_4x4_arr();
	buf.copy_projection_to_mat4x4(&mut render_mat_without_transform);
	multiply_4x4_matrices(&mut render_mat_without_transform, &camera.view_matrix);


	let tris_iterator = match buf.get_cull_mode() {
		CullMode::CullTris => [].iter(),
		_ => yade_data.tris.iter(),
	};

	fn min_of_each_tri_line(trs_p0: &Vec3, trs_p1: &Vec3, trs_p2: &Vec3, camera: &Camera, screen_tri: &ScreenTri, render_data: &mut Vec::<(f32, Primitive)>) {
		let dist0 = trs_p0.squared_dist_to(&camera.position).min(trs_p1.squared_dist_to(&camera.position));
		render_data.push((dist0, Primitive::Line(screen_tri.p0.clone(), screen_tri.p1.clone())));
		let dist1 = trs_p1.squared_dist_to(&camera.position).min(trs_p2.squared_dist_to(&camera.position));
		render_data.push((dist1, Primitive::Line(screen_tri.p1.clone(), screen_tri.p2.clone())));
		let dist2 = trs_p2.squared_dist_to(&camera.position).min(trs_p0.squared_dist_to(&camera.position));
		render_data.push((dist2, Primitive::Line(screen_tri.p2.clone(), screen_tri.p0.clone())));
	}

	fn max_of_each_tri_line(trs_p0: &Vec3, trs_p1: &Vec3, trs_p2: &Vec3, camera: &Camera, screen_tri: &ScreenTri, render_data: &mut Vec::<(f32, Primitive)>) {
		let dist0 = trs_p0.squared_dist_to(&camera.position).max(trs_p1.squared_dist_to(&camera.position));
		render_data.push((dist0, Primitive::Line(screen_tri.p0.clone(), screen_tri.p1.clone())));
		let dist1 = trs_p1.squared_dist_to(&camera.position).max(trs_p2.squared_dist_to(&camera.position));
		render_data.push((dist1, Primitive::Line(screen_tri.p1.clone(), screen_tri.p2.clone())));
		let dist2 = trs_p2.squared_dist_to(&camera.position).max(trs_p0.squared_dist_to(&camera.position));
		render_data.push((dist2, Primitive::Line(screen_tri.p2.clone(), screen_tri.p0.clone())));
	}

	let sorting_criteria = buf.get_sorting_mode();

	let triangle_sort_fn = if let ZSortingMode::FarthestPoint = sorting_criteria { max_of_each_tri_line } else { min_of_each_tri_line };

	for tri in tris_iterator {

		let clip_p0 = tri.p0.get_transformed_by_mat4x4_w(&buf.render_mat);
		let clip_p1 = tri.p1.get_transformed_by_mat4x4_w(&buf.render_mat);
		let clip_p2 = tri.p2.get_transformed_by_mat4x4_w(&buf.render_mat);

		// buf.write_debug(&format!("p0 {:?} x {} y {}\n", clip_p0, clip_p0.x_in_w_range(), clip_p0.y_in_w_range()));
		// buf.write_debug(&format!("p1 {:?} x {} y {}\n", clip_p1, clip_p1.x_in_w_range(), clip_p1.y_in_w_range()));
		// buf.write_debug(&format!("p2 {:?} x {} y {}\n\n", clip_p2, clip_p2.x_in_w_range(), clip_p2.y_in_w_range()));

		let Some(screen_tri) = cull_tri_into_screen_space(clip_p0, clip_p1, clip_p2, buf) else { continue };

		let trs_p0 = tri.p0.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
		let trs_p1 = tri.p1.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
		let trs_p2 = tri.p2.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);

		// Avg between each of the 3 points
		// let mid = Vec3::mid_point_of_tri(&trs_p0, &trs_p1, &trs_p2);
		// let mid_scr = clip_space_to_screen_space(&mid.get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform), buf.wid, buf.hei);
		// safe_render_char_i('@', &mid_scr, buf);
		// let sq_dist_to_camera = mid.squared_dist_to(&camera.position);
		// safe_render_string_signed(&format!("d {:.2}", sq_dist_to_camera), mid_scr.x, mid_scr.y, buf);
		// render_data_by_dist.push((sq_dist_to_camera, Primitive::Tri(screen_tri)));

		// Min of the 3 points
		// let sq_dist_to_scr = trs_p0.squared_dist_to(&camera.position)
		// 	.min(trs_p1.squared_dist_to(&camera.position))
		// 	.min(trs_p2.squared_dist_to(&camera.position));
		// render_data_by_dist.push((sq_dist_to_scr, Primitive::Tri(screen_tri)));

		triangle_sort_fn(&trs_p0, &trs_p1, &trs_p2, camera, &screen_tri, &mut render_data_by_dist);

		// Avg between each line
		// let dist0 = Vec3::new((trs_p0.x + trs_p1.x) * 0.5, (trs_p0.y + trs_p1.y) * 0.5, (trs_p0.z + trs_p1.z) * 0.5).squared_dist_to(&camera.position);
		// render_data_by_dist.push((dist0, Primitive::Line(screen_tri.p0.clone(), screen_tri.p1.clone())));
		// let dist1 = Vec3::new((trs_p1.x + trs_p2.x) * 0.5, (trs_p1.y + trs_p2.y) * 0.5, (trs_p1.z + trs_p2.z) * 0.5).squared_dist_to(&camera.position);
		// render_data_by_dist.push((dist1, Primitive::Line(screen_tri.p1.clone(), screen_tri.p2.clone())));
		// let dist2 = Vec3::new((trs_p2.x + trs_p0.x) * 0.5, (trs_p2.y + trs_p0.y) * 0.5, (trs_p2.z + trs_p0.z) * 0.5).squared_dist_to(&camera.position);
		// render_data_by_dist.push((dist2, Primitive::Line(screen_tri.p2.clone(), screen_tri.p0.clone())));
	}

	let balls_iterator = match buf.get_cull_mode() {
		CullMode::CullBalls => [].iter().enumerate(),
		_ => yade_data.balls.iter().enumerate(),
	};

	for (index, ball) in balls_iterator {

		let rad_scaled = ball.rad * scale;
		let clip_pos = ball.pos.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);

		let transformed_pos = ball.pos.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);

		// culling balls behind the camera
		let ball_to_cam = camera.position - transformed_pos;
		let dot = Vec3::dot_product(&camera.forward, &ball_to_cam);
		if dot < 0.0 { continue }

		let trs_pos_sd = transformed_pos.add_vec(&(camera.side * rad_scaled));
		let trs_pos_sd_proj = trs_pos_sd.get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform);
		let trs_pos_sd_projected_screen_f = clip_space_to_screen_space_f(&trs_pos_sd_proj, buf.wid, buf.hei);

		let screen_pos_f32 = clip_space_to_screen_space_f(&clip_pos, buf.wid, buf.hei);
		let rad = (trs_pos_sd_projected_screen_f.x - screen_pos_f32.x).abs();
		// buf.write_debug(&format!("{:?} scr {:?}\n", trs_pos_sd_projected_screen_f, screen_pos_f32));

		if cull_circle(&screen_pos_f32, rad, buf) { continue; }

		let sq_dist_to_camera = transformed_pos.squared_dist_to(&camera.position);
		let screen_pos = IVec2::from(&screen_pos_f32);

		// DEBUG
		// safe_render_string_signed(&format!("C {:.2}", sq_dist_to_camera), screen_pos.x, (screen_pos_f32.y as f32 - rad * 3.5) as i32, buf);

		let render_data = RenderBallData {
			rad,
			screen_pos,
			index,
		};

		render_data_by_dist.push((sq_dist_to_camera, Primitive::Ball(render_data)));
	}

	render_data_by_dist.sort_by(buf.get_sorting_mode().get_sorting_fn());


	for data_to_render_by_dist in render_data_by_dist.iter() {

		// buf.write_debug(&format!("- {}\n", data_to_render_by_dist.0));
		let (_, data_to_render) = data_to_render_by_dist;

		match data_to_render {
			Primitive::Ball(ball_data) => {
				let digit = ball_data.index as u32 % ('Z' as u32 - 'A' as u32) + ('A' as u32);
				let letter = char::from_u32(digit).unwrap();
				render_fill_bres_circle(&ball_data.screen_pos, ball_data.rad, letter, buf);
			},
			Primitive::Line(p0, p1) => {
				render_bresenham_line(&p0, p1, buf, YADE_WIRE_FILL_CHAR);
			},
			// Primitive::Tri(screen_tri) => {
			// 	render_bresenham_line(&screen_tri.p0, &screen_tri.p1, buf, YADE_WIRE_FILL_CHAR);
			// 	render_bresenham_line(&screen_tri.p1, &screen_tri.p2, buf, YADE_WIRE_FILL_CHAR);
			// 	render_bresenham_line(&screen_tri.p2, &screen_tri.p0, buf, YADE_WIRE_FILL_CHAR);
			// },
		}
	}
}

pub fn render_mesh(mesh: &Mesh, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);

	let start_ms = 89_340;
	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
	// teapot weirded out thing
	let start_ms = 95_374;
	let t = (start_ms) as f32 * 0.001;
	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);
	let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);

	let speed = 0.3;
	let sharpness = 2.5;

	let tri_wave = triangle_wave(t * speed);
	let t_smooth_wave = smoothed_0_to_1_s(tri_wave, sharpness);
	let tmod = lerp_f32(0.5, 1.0, t_smooth_wave);
	let (scale_x, scale_y, scale_z) = (tmod, tmod, tmod);
	let (scale_x, scale_y, scale_z) = (1.0, 1.0, 1.0);


	// this tweens the scale and rotation of the thing
	// let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);

	// let speed = 0.5;
	// let mut t = timer.time_aggr.as_millis() as f32 * 0.001 * speed;
	// if buf.test { t = 0.0; }

	// let (angle_x, angle_y, angle_z) = (t * 0.3, t, t * 2.1);

	// let speed = 0.6;
	// let tmod = ((t * speed % 1.0) - 0.5).abs() * 2.0;
	// render_string(&format!("{}", tmod), &UVec2::new(0, 7), buf);
	// let tmod_smooth = smoothed_0_to_1(tmod);
	// let animation_curve = tmod_smooth * 0.5 + 0.25;
	// let (scale_x, scale_y, scale_z) = (animation_curve, animation_curve, animation_curve);


	buf.copy_projection_to_render_matrix();

	apply_identity_to_mat_4x4(&mut buf.transf_mat);
	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);

	// buf.clear_debug();
	let num_tris = mesh.tris_indices.len() / 3;
	for tri_i in 0..num_tris {

		let p0_i = tri_i * 3 + 0;
		let p1_i = tri_i * 3 + 1;
		let p2_i = tri_i * 3 + 2;

		// buf.write_debug(&format!("gets p0 p1 p2   {} {} {}\n", p0_i, p1_i, p2_i));
		let p0 = mesh.get_vert_at(p0_i);
		let p1 = mesh.get_vert_at(p1_i);
		let p2 = mesh.get_vert_at(p2_i);

		let trs_p0 = p0.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);
		let trs_p1 = p1.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);
		let trs_p2 = p2.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);

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

pub fn render_axes(buf: &mut TerminalBuffer, camera: &Camera, render_marks: bool) {

	buf.copy_projection_to_render_matrix();

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);

	let origin  = screen_project(&Vec3::new(0.0, 0.0, 0.0), &buf.render_mat, buf.wid, buf.hei);

	// TODO: debug why this can get crappily signed values if AXIS_SZ_WORLD is > 500
	// TODO: this is already bugged at FUCKING 30
	// const AXIS_SZ_WORLD: f32 = 20.0;
	const AXIS_SZ_WORLD: f32 = 3.0;
	let up    = screen_project(&Vec3::new(0.0, AXIS_SZ_WORLD, 0.0), &buf.render_mat, buf.wid, buf.hei);
	let right = screen_project(&Vec3::new(AXIS_SZ_WORLD, 0.0, 0.0), &buf.render_mat, buf.wid, buf.hei);
	let front = screen_project(&Vec3::new(0.0, 0.0, AXIS_SZ_WORLD), &buf.render_mat, buf.wid, buf.hei);

	let offset = 0.1;

	render_bresenham_line(&origin, &up, buf, '|');
	render_bresenham_line(&origin, &right, buf, '-');
	render_bresenham_line(&origin, &front, buf, '/');

	if !render_marks { return }

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

pub fn render_gizmos(buf: &mut TerminalBuffer, camera: &Camera) {

	buf.copy_projection_to_render_matrix();

	const GIZMO_SIZE_WORLD: f32 = 0.15;

	// in world space, the gizmos is 8 units back (view matrix is irrelevant for these calculations)
	let base_world_space = Vec3::new(0.0, 0.0, -8.0);
	let origin = screen_project(&base_world_space, &buf.render_mat, buf.wid, buf.hei);
	let gizmos_side_reference_point = base_world_space.added_vec(&Vec3::new(GIZMO_SIZE_WORLD, 0.0, 0.0));
	let gizmos_side_reference_point_projected = screen_project(&gizmos_side_reference_point, &buf.render_mat, buf.wid, buf.hei);

	let side_offset = (gizmos_side_reference_point_projected.x - origin.x) as Int;
	let screen_offset = IVec2::new(
			buf.wid as Int / 2 -   side_offset       - 1,
		- ( buf.hei as Int / 2 - ( side_offset / 2 ) - 1 )
	);

	let origin_2d = origin.sum(&screen_offset);

	let dbg_forward = camera.forward.inversed().with_y_inverted();
	let dbg_side = camera.side.inversed().with_y_inverted();
	let dbg_up = camera.up.with_y_inverted();

	let mut draw_between = |dir: &Vec3, ch: char| {
		let ptr = screen_project(&(base_world_space + (dir * GIZMO_SIZE_WORLD)), &buf.render_mat, buf.wid, buf.hei).sum(&screen_offset);
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