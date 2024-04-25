use std::f32::consts::TAU;

use crate::{*, camera::Camera, maths::*, timer::Timer, terminal_wrapper::TerminalBuffer, ASCII_BYTES_PER_CHAR, YADE_SCALE_TEMP};



pub fn encode_char_in(ch: char, index: usize, vec: &mut [u8]) {
	ch.encode_utf8(&mut vec[index .. index+ASCII_BYTES_PER_CHAR]);
}

pub fn render_char_i(ch: char, pos: &IVec2, buffer: &mut TerminalBuffer) {
	debug_assert!(pos.x >= 0 && pos.x < buffer.wid.into());
	debug_assert!(pos.y >= 0 && pos.y < buffer.hei.into());

	render_char(ch, &pos.into(), buffer)
}

pub fn render_char(ch: char, pos: &UVec2, buffer: &mut TerminalBuffer) {
	debug_assert!(pos.x < buffer.wid.into());
	debug_assert!(pos.y < buffer.hei.into());

	let index = xy_to_it(pos.x, pos.y, buffer.wid);
	encode_char_in(ch, index, &mut buffer.vec);
}

pub fn render_string(string: &str, pos: &UVec2, buf: &mut TerminalBuffer) {
	// string can't overflow the line
	debug_assert!(pos.x as usize + string.len() - 1 < buf.wid.into(), "trying to render string after line end");
	debug_assert!(!string.contains('\n'), "can't render a string that has a line end!");

	let mut index = xy_to_it(pos.x, pos.y, buf.wid);
	for byte in string.bytes() {
		buf.vec[index] = byte;
		index += ASCII_BYTES_PER_CHAR
	}
}

pub fn safe_render_string_signed(string: &str, x: Int, y: Int, buf: &mut TerminalBuffer) {
	if x < 0 || x as usize + string.len() - 1 >= buf.wid.into() || y < 0 || y >= buf.hei as Int { return }
	render_string(string, &UVec2::new(x as u16, y as u16), buf);
}

pub fn safe_render_char_signed(ch: char, x: Int, y: Int, buf: &mut TerminalBuffer) {
	if x < 0 || x >= buf.wid as Int || y < 0 || y >= buf.hei as Int { return }
	render_char(ch, &UVec2::new(x as u16, y as u16), buf);
}

pub fn render_bresenham_line(p0: &IVec2, p1: &IVec2, buf: &mut TerminalBuffer, fill_char: char) {
	// let last_x = buf.wid - 1;
	// let last_y = buf.hei - 1;

	// // cull lines completely out of the canvas
	// if p0.x > last_x && p1.x > last_x { return }
	// if p0.y > last_y && p1.y > last_y { return }

	let x0 = p0.x as i32;
	let y0 = p0.y as i32;
	let x1 = p1.x as i32;
	let y1 = p1.y as i32;

	let dx = (x1 - x0).abs();
	let dy = (y1 - y0).abs();
	let sx = if x0 < x1 { 1 } else { -1 };
	let sy = if y0 < y1 { 1 } else { -1 };

	let mut deriv_diff = dx - dy;
	let mut x = x0;
	let mut y = y0;

	// buf.write_debug(&format!("w {} h {} \np0 {:} p1 {:} dx {} dy {}  sx {} sy {}\n ", buf.wid, buf.hei, p0, p1, dx, dy, sy, sy));

	loop {

		// handle out of bounds
		if x >= 0 && x < buf.wid.into() && y >= 0 && y < buf.hei.into() {
			let index = xy_to_it(x as u16, y as u16, buf.wid);

			// TODO: figure out how would this work with UTF8
			fill_char.encode_utf8(&mut buf.vec[index..index + ASCII_BYTES_PER_CHAR]);
		}

		// if (x < 0 && sx == -1) || (x > wid && sx == 1) { return }
		// if (y < 0 && sy == -1) || (y > hei && sy == 1) { return }

		if x == x1 && y == y1 { return }

		let double_deriv_diff = deriv_diff * 2;
		if double_deriv_diff > -dy {
			deriv_diff -= dy;
			x += sx;
		}
		if double_deriv_diff < dx {
			deriv_diff += dx;
			y += sy;
		}
	}
}

pub fn render_sphere(pos: &Vec3, rad: f32, ch: char, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	buf.copy_projection_to_render_matrix();
	apply_identity_to_mat_4x4(&mut buf.transf_mat);

	buf.clear_debug();

	let mut rot_only_mat = create_identity_4x4();

	let mut proj_only_mat = create_identity_4x4();
	proj_only_mat.copy_from_slice(&buf.render_mat);


	let speed = 0.5;
	let mut t = timer.time_aggr.as_millis() as f32 * 0.001 * speed;
	// let mut t = TAU * 1./4.;
	// if buf.test { t = 0.0; }

	// let rad = 0.009926;
	// let rad = 0.01;
	// let pos = Vec3::new(-0.034109, -0.002092, -0.080507);
	// let pos = Vec3::new(0.02, 0., 0.);

	let pos = pos * YADE_SCALE_TEMP;
	let rad_sc = rad * YADE_SCALE_TEMP;

	buf.write_debug(&format!("{:?} original\n", pos));
	// buf.write_debug(&format!("{:.6} RAD\n", rad));
	buf.write_debug(&format!("{:.6} RAD * SCALE\n", rad_sc));

	// apply_scale_to_mat_4x4(&mut buf.transf_mat, YADE_SCALE_TEMP, YADE_SCALE_TEMP, YADE_SCALE_TEMP);
	// let scaled = pos.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
	// buf.write_debug(&format!("{:?} scaled\n", scaled));

	// DO IT BEFORE ROTATION IS APPLIED
	// multiply_4x4_matrices(&mut proj_only_mat, &camera.view_matrix);
	// multiply_4x4_matrices(&mut proj_only_mat, &buf.transf_mat);

	// TODO: do it with a 3x3 matrix buffer and transform the point

	apply_rotation_to_mat_4x4_simple(&mut rot_only_mat, 0.0, t, 0.0);
	let rot_pos = pos.get_transformed_by_mat4x4_discard_w(&rot_only_mat);
	buf.write_debug(&format!("{:?} rot only\n", rot_pos));

	let rot_pos_up = rot_pos.add_vec(&(camera.up * rad_sc));
	buf.write_debug(&format!("{:?} rot up\n", rot_pos_up));

	apply_rotation_to_mat_4x4_simple(&mut buf.transf_mat, 0.0, t, 0.0);

	// let rot = pos.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
	// buf.write_debug(&format!("{:?} rot, t {}\n", rot, t));

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);

	multiply_4x4_matrices(&mut proj_only_mat, &camera.view_matrix);


	let rot_pos_up_proj = rot_pos_up.get_transformed_by_mat4x4_uniform(&proj_only_mat);
	buf.write_debug(&format!("{:?} rot up projected\n", rot_pos_up_proj));

	let rot_pos_up_proj_2d_f32 = clip_space_to_screen_space_f32(&rot_pos_up_proj, buf.wid, buf.hei);

	let rot_pos_up_proj_2d = clip_space_to_screen_space(&rot_pos_up_proj, buf.wid, buf.hei);
	buf.write_debug(&format!("{:?} rot_pos_up_proj_2d, BUT: {:?}\n", rot_pos_up_proj_2d, rot_pos_up_proj_2d_f32));

	let rot_pos_up_proj_2d_f = clip_space_to_screen_space_f32(&rot_pos_up_proj, buf.wid, buf.hei);
	// safe_render_char_signed('X', rot_pos_up_proj_2d.x, rot_pos_up_proj_2d.y, buf);



	let projected = pos.get_transformed_by_mat4x4_uniform(&buf.render_mat);
	buf.write_debug(&format!("\n\n{:?} projected\n", projected));
	// let projected_clip = normalize_clip_space(&projected);
	// buf.write_debug(&format!("{:?} normalized\n", projected_clip));
	let projected_2d = clip_space_to_screen_space(&projected, buf.wid, buf.hei);
	buf.write_debug(&format!("{:?} 2d of wh {:?}\n", projected_2d, IVec2::new(buf.wid.into(), buf.hei.into())));


	// let up_proj = rot_pos.add_vec(&(&camera.up * rad * YADE_SCALE_TEMP)).get_transformed_by_mat4x4_uniform(&buf.transf_mat);
	// // let up_proj = pos.add_vec(&(&camera.up * rad)).get_transformed_by_mat4x4_uniform(&proj_only_mat);
	// let scr = up_proj.clip_space_to_screen_space(buf.wid, buf.hei);
	// safe_render_char_signed('P', scr.x, scr.y, buf);

	
	let rot_pos_side = rot_pos.add_vec(&(&camera.side.inversed() * rad_sc));
	let rot_pos_side_proj = rot_pos_side.get_transformed_by_mat4x4_uniform(&proj_only_mat);
	let rot_pos_side_proj_2d = clip_space_to_screen_space(&rot_pos_side_proj, buf.wid, buf.hei);




	// safe_render_char_signed('c', projected_2d.x, projected_2d.y, buf);

	let ball_up = pos.add_vec(&(camera.up * rad * YADE_SCALE_TEMP)). get_transformed_by_mat4x4_uniform(&buf.render_mat);
	let ball_right = pos.add_vec(&(&camera.side.inversed() * rad * YADE_SCALE_TEMP)).get_transformed_by_mat4x4_uniform(&buf.render_mat);
	// let ball_up = ball_pos.add_vec(&(&camera.up * ball.rad * YADE_SCALE_TEMP));

	// buf.write_debug(&format!("{:}, r {:.6}, pos {:?} -> {:?} up {:?}\n", i, ball.rad, ball.pos, ball_pos, ball_up));

	let screen_circ = clip_space_to_screen_space(&projected, buf.wid, buf.hei);
	let screen_circ_f32 = clip_space_to_screen_space_f32(&projected, buf.wid, buf.hei);

	// safe_render_char_signed('U', screen_up.x, screen_up.y, buf);
	// safe_render_char_signed('S', screen_side.x, screen_side.y, buf);


	let dist = (rot_pos_up_proj_2d_f32.1 - screen_circ_f32.1).abs();
	// render_fill_bres_circle(&screen_circ, dist, '.', buf);
	render_fill_bres_circle(&screen_circ, dist, ch, buf);

	safe_render_char_signed('C', screen_circ.x, screen_circ.y, buf);

	safe_render_char_signed('U', rot_pos_up_proj_2d.x, rot_pos_up_proj_2d.y, buf);
	safe_render_char_signed('S', rot_pos_side_proj_2d.x, rot_pos_side_proj_2d.y, buf);
}