use core::{panic, time};
use std::{thread, time::Duration};

use crate::{camera::Camera, file_readers::yade_dem_reader::YadeDemData, renderer::Renderer, terminal::TerminalBuffer, timer::Timer, maths::*, rendering::*};


const TRIS_WIRE_FILL_CHAR: char = '*';

pub struct YadeRenderer {
	data: YadeDemData,
}

impl YadeRenderer {
	pub fn new(data: YadeDemData) -> Self {
		Self { data }
	}
}

impl Renderer for YadeRenderer {
	fn render(&self, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
		render_yade_sorted(&self.data, buf, timer, camera);
	}
}



pub fn render_yade_sorted(yade_data: &YadeDemData, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);

	let speed = 0.5;
	let start_ms = 0;
	let mut t = (timer.time_aggr.as_millis() * 0 + start_ms) as f32 * 0.001 * speed;
	// let mut t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001 * speed;
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
	let mut render_data_by_dist = Vec::<(f32, YadePrimitive)>::with_capacity(yade_data.balls.len());


	// could make a buffer in TerminalBuffer for this
	let mut render_mat_without_transform = create_identity_4x4_arr();
	buf.copy_projection_to_mat4x4(&mut render_mat_without_transform);
	multiply_4x4_matrices(&mut render_mat_without_transform, &camera.view_matrix);



	// let balls_iterator = match buf.get_cull_mode() {
	// 	// CullMode::CullBalls => [].iter().enumerate(),
	// 	_ => yade_data.balls.iter().enumerate(),
	// };

	let balls_iterator = yade_data.balls.iter().enumerate();

	let mut ball_painter = BallPainter::new(buf.get_ball_fill_mode());

	let mut smallest_rad_3d = f32::MAX;
	for (index, ball) in balls_iterator {
	// for (index, ball) in balls_iterator.skip((buf.test_i as usize) % yade_data.balls.len()).take(5) {

		let rad_scaled_3d = ball.rad * scale;
		smallest_rad_3d = smallest_rad_3d.min(rad_scaled_3d);

		if let CullMode::CullBalls = buf.get_cull_mode() { continue }

		let clip_pos = ball.pos.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);

		let transformed_pos = ball.pos.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);

		// culling balls too far away
		if transformed_pos.squared_dist_to(&camera.position) > SQUARED_ZF { continue }

		// culling balls behind the camera
		let ball_to_cam = camera.position - transformed_pos;
		let dot = Vec3::dot_product(&camera.forward, &ball_to_cam);
		if dot < 0.0 { continue }

		let reference_pos = transformed_pos.add_vec(&(camera.side * rad_scaled_3d));
		let reference_pos_proj = reference_pos.get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform);
		let reference_pos_proj_screen_f = clip_space_to_screen_space_f(&reference_pos_proj, buf.wid, buf.hei);

		let screen_pos_f32 = clip_space_to_screen_space_f(&clip_pos, buf.wid, buf.hei);
		let rad_2d = (reference_pos_proj_screen_f.x - screen_pos_f32.x).abs();
		// buf.write_debug(&format!("{:?} scr {:?}\n", trs_pos_sd_projected_screen_f, screen_pos_f32));

		if cull_circle(&screen_pos_f32, rad_2d, buf) { continue; }

		let screen_pos = IVec2::from(&screen_pos_f32);

		let sq_dist_to_camera = transformed_pos.squared_dist_to(&camera.position);

		// DEBUG
		// safe_render_string_signed(&format!("C {:.2}", sq_dist_to_camera), screen_pos.x, (screen_pos_f32.y as f32 - rad * 3.5) as i32, buf);

		let cam_to_pos_vec = transformed_pos - camera.position;
		let sq_dist_xz = cam_to_pos_vec.x * cam_to_pos_vec.x + cam_to_pos_vec.z * cam_to_pos_vec.z;

		ball_painter.find_min_max(&transformed_pos, sq_dist_xz);

		let render_data = RenderBallData {
			height: transformed_pos.y,
			sq_dist_to_camera: sq_dist_xz,
			rad: rad_2d,
			screen_pos,
			index,
		};

		render_data_by_dist.push((sq_dist_to_camera, YadePrimitive::Ball(render_data)));
	}



	let tris_iterator = match buf.get_cull_mode() {
		CullMode::CullTris => [].iter(),
		_ => yade_data.tris.iter(),
	};

	let triangle_lines_distance_fn = match buf.get_sorting_mode() {
		ZSortingMode::FarthestPoint => max_of_each_tri_line,
		ZSortingMode::ClosestPoint  => min_of_each_tri_line,
		_ => min_of_each_tri_line, // average?
	};

	// for tri in tris_iterator.skip((buf.test_i as usize) % yade_data.tris.len()).take(1) {
	// for tri in tris_iterator.skip(30).take(2) { // middle bladder
	// for tri in tris_iterator.skip(5).take(5) { // bunch of tris
	for tri in tris_iterator {

		let Some(screen_tri) = cull_tri_into_screen_space(&tri.p0, &tri.p1, &tri.p2, camera, buf) else { continue };

		let trs_p0 = tri.p0.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
		let trs_p1 = tri.p1.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
		let trs_p2 = tri.p2.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);

		if let ZSortingMode::Optimized = buf.get_sorting_mode() {
			cut_line_and_insert(&trs_p0, &trs_p1, smallest_rad_3d, &render_mat_without_transform, &camera, &mut render_data_by_dist, buf);
			cut_line_and_insert(&trs_p1, &trs_p2, smallest_rad_3d, &render_mat_without_transform, &camera, &mut render_data_by_dist, buf);
			cut_line_and_insert(&trs_p2, &trs_p0, smallest_rad_3d, &render_mat_without_transform, &camera, &mut render_data_by_dist, buf);
		} else {
			// TODO: take ownership
			triangle_lines_distance_fn(&trs_p0, &trs_p1, &trs_p2, camera, &screen_tri, &mut render_data_by_dist);
		}
	}


	render_data_by_dist.sort_by(buf.get_sorting_mode().get_sorting_fn());

	let mut lines = 0;
	for data_to_render_by_dist in render_data_by_dist.iter() {

		// buf.write_debug(&format!("- {}\n", data_to_render_by_dist.0));
		let (_, data_to_render) = data_to_render_by_dist;

		// buf.write_debug(&format!("cur {:.2} min {:.2} max {:.2} \n", dist_sq, min_dist_sq, max_dist_sq));
		match data_to_render {
			YadePrimitive::Ball(ball_data) => {
				let letter = ball_painter.get_fill_letter(&ball_data);
				render_fill_bres_circle(&ball_data.screen_pos, ball_data.rad, letter, buf);
			},
			YadePrimitive::Line(line) => {
				render_bresenham_line(&line.p0, &line.p1, buf, TRIS_WIRE_FILL_CHAR);

				// TODO: DEBUG OPTION different line drawing modes
				// buf.write_debug(&format!(" LINE {}, {} %\n", lines, lines % 2));
				// if lines % 2 == 0 {
				// 	render_bresenham_line(&line.p0, &line.p1, buf, '.');
				// } else {
				// 	render_bresenham_line(&line.p0, &line.p1, buf, '@');
				// }

				lines += 1;
			},
		}
	}

	// buf.write_debug(&format!("lines {}\n", lines));
}

fn cut_line_and_insert(
	p0: &Vec3,
	p1: &Vec3,
	segment_threshold: f32,
	render_mat: &[f32], 
	camera: &Camera,
	render_data: &mut Vec::<(f32, YadePrimitive)>,
	buf: &mut TerminalBuffer
) {
	let p0_to_p1 = p1 - p0;

	let mag = p0_to_p1.magnitude();
	let p0_to_p1_dir = p0_to_p1 / mag;

	let segs = (mag / segment_threshold) as i32;

	// buf.write_debug(&format!("p0 {:} p1 {:}\ndir {:} \nmag p0p1 {:.4} smallest rad {:.4}\n", p0, p1, p0_to_p1_dir, mag, segment_threshold));
	// buf.write_debug(&format!("segs {}, to int {segs} \n", mag / segment_threshold));
	// buf.write_debug("\n");

	let total_mag = segs as f32 * segment_threshold;

	let mut last_point = Option::<Vec3>::None;
	for seg_i in 0..segs {
		let seg_ith = seg_i as f32;
		let seg_next = (seg_i + 1) as f32;

		let ith_t  = seg_ith  / segs as f32;
		let next_t = seg_next / segs as f32;

		// buf.write_debug(&format!(" cur {:.4} next {:.4} *MAG {:.4}, {:.4} \n", ith_t, next_t, ith_t * total_mag, next_t * total_mag));

		let seg_i_p0 = p0 + &(&p0_to_p1_dir * (ith_t  * total_mag));
		let seg_i_p1 = p0 + &(&p0_to_p1_dir * (next_t * total_mag));

		// buf.write_debug(&format!("  f {:} to {:} MAG {:.4}\n", seg_i_p0, seg_i_p1, (seg_i_p0 - seg_i_p1).magnitude()));

		let seg_p0 = screen_project(&seg_i_p0, &render_mat, buf.wid, buf.hei);
		let seg_p1 = screen_project(&seg_i_p1, &render_mat, buf.wid, buf.hei);

		// buf.write_debug(&format!("    screen p0 {:} to {:}\n", seg_p0, seg_p1));

		let dist = seg_i_p0.squared_dist_to(&camera.position).min(seg_i_p1.squared_dist_to(&camera.position));

		last_point = Some(seg_i_p1);

		let line = YadePrimitive::Line(Line { p0: seg_p0, p1: seg_p1 });
		render_data.push( (dist, line) );
	}

	// draws from the last drawn point to p1
	let last_drawn_p = last_point.unwrap();

	let last_drawn_p_scr = screen_project(&last_drawn_p, &render_mat, buf.wid, buf.hei);
	let p1_scr = screen_project(&p1, &render_mat, buf.wid, buf.hei);

	let dist = last_drawn_p.squared_dist_to(&camera.position).min(p1.squared_dist_to(&camera.position));

	let line = YadePrimitive::Line(Line { p0: last_drawn_p_scr, p1: p1_scr });
	render_data.push( (dist, line) );
}


fn min_of_each_tri_line(trs_p0: &Vec3, trs_p1: &Vec3, trs_p2: &Vec3, camera: &Camera, screen_tri: &ScreenTri, render_data: &mut Vec::<(f32, YadePrimitive)>) {
	let dist0 = trs_p0.squared_dist_to(&camera.position).min(trs_p1.squared_dist_to(&camera.position));
	let line_p0_p1 = YadePrimitive::Line( (screen_tri.p0.clone(), screen_tri.p1.clone()).into() );
	render_data.push((dist0, line_p0_p1));

	let dist1 = trs_p1.squared_dist_to(&camera.position).min(trs_p2.squared_dist_to(&camera.position));
	let line_p1_p2 = YadePrimitive::Line( (screen_tri.p1.clone(), screen_tri.p2.clone()).into() );
	render_data.push((dist1, line_p1_p2));

	let dist2 = trs_p2.squared_dist_to(&camera.position).min(trs_p0.squared_dist_to(&camera.position));
	let line_p2_p0 = YadePrimitive::Line( (screen_tri.p2.clone(), screen_tri.p0.clone()).into() );
	render_data.push((dist2, line_p2_p0));
}

fn max_of_each_tri_line(trs_p0: &Vec3, trs_p1: &Vec3, trs_p2: &Vec3, camera: &Camera, screen_tri: &ScreenTri, render_data: &mut Vec::<(f32, YadePrimitive)>) {
	let dist0 = trs_p0.squared_dist_to(&camera.position).max(trs_p1.squared_dist_to(&camera.position));
	let line_p0_p1 = YadePrimitive::Line( (screen_tri.p0.clone(), screen_tri.p1.clone()).into() );
	render_data.push((dist0, line_p0_p1));

	let dist1 = trs_p1.squared_dist_to(&camera.position).max(trs_p2.squared_dist_to(&camera.position));
	let line_p1_p2 = YadePrimitive::Line( (screen_tri.p1.clone(), screen_tri.p2.clone()).into() );
	render_data.push((dist1, line_p1_p2));

	let dist2 = trs_p2.squared_dist_to(&camera.position).max(trs_p0.squared_dist_to(&camera.position));
	let line_p2_p0 = YadePrimitive::Line( (screen_tri.p2.clone(), screen_tri.p0.clone()).into() );
	render_data.push((dist2, line_p2_p0));
}





#[deprecated(since="0.0", note=r#"Call "render_yade_sorted", keeping this until I code Transform"#)]
pub fn render_yade(yade_data: &YadeDemData, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

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

		let Some(screen_tri) = cull_tri_into_screen_space(&tri.p0, &tri.p1, &tri.p2, camera, buf) else { continue };

		render_bresenham_line(&screen_tri.p0, &screen_tri.p1, buf, TRIS_WIRE_FILL_CHAR);
		render_bresenham_line(&screen_tri.p1, &screen_tri.p2, buf, TRIS_WIRE_FILL_CHAR);
		render_bresenham_line(&screen_tri.p2, &screen_tri.p0, buf, TRIS_WIRE_FILL_CHAR);
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

		let sq_dist_to_camera = transformed_pos.squared_dist_to(&camera.position);

		let render_data = RenderBallData {
			height: transformed_pos.y,
			sq_dist_to_camera,
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