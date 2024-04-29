use std::f32::consts::TAU;

use crate::{*, camera::Camera, maths::*, timer::Timer, terminal_wrapper::TerminalBuffer, ASCII_BYTES_PER_CHAR, YADE_SCALE_TEMP};

use super::CIRCLE_FILL_CHAR;



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

pub fn safe_render_char_at(ch: char, x: Int, y: Int, buf: &mut TerminalBuffer) {
	if x < 0 || x >= buf.wid as Int || y < 0 || y >= buf.hei as Int { return }
	render_char(ch, &UVec2::new(x as u16, y as u16), buf);
}

pub fn safe_render_char(ch: char, pos: &IVec2, buf: &mut TerminalBuffer) {
	if pos.x < 0 || pos.x >= buf.wid as Int || pos.y < 0 || pos.y >= buf.hei as Int { return }
	render_char(ch, &pos.into(), buf);
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

	let left_st  = IVec2::new(base_x - scaled_y, base_y);
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
	safe_render_char_at(CIRCLE_FILL_CHAR, base_x - scaled_x, base_y + y, buf);
	safe_render_char_at(CIRCLE_FILL_CHAR, base_x - scaled_y, base_y + x, buf);
	safe_render_char_at(CIRCLE_FILL_CHAR, base_x - scaled_y, base_y - x, buf);
	safe_render_char_at(CIRCLE_FILL_CHAR, base_x - scaled_x, base_y - y, buf);
	// right
	safe_render_char_at(CIRCLE_FILL_CHAR, base_x + scaled_x, base_y + y, buf);
	safe_render_char_at(CIRCLE_FILL_CHAR, base_x + scaled_y, base_y + x, buf);
	safe_render_char_at(CIRCLE_FILL_CHAR, base_x + scaled_y, base_y - x, buf);
	safe_render_char_at(CIRCLE_FILL_CHAR, base_x + scaled_x, base_y - y, buf);
}

// TODO: fix and make this more general
pub fn render_sphere(pos: &Vec3, rad: f32, ch: char, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	buf.copy_projection_to_render_matrix();
	apply_identity_to_mat_4x4(&mut buf.transf_mat);

	buf.clear_debug();

	// let mut rot_only_mat = create_identity_4x4();

	let mut render_mat_without_transform = create_identity_4x4();
	buf.copy_projection_to_mat4x4(&mut render_mat_without_transform);
	multiply_4x4_matrices(&mut render_mat_without_transform, &camera.view_matrix);


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
	buf.write_debug(&format!("{:.6} RAD * SCALE\n", rad_sc));


	apply_rotation_to_mat_4x4(&mut buf.transf_mat, 0.0, t, 0.0);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);

	let rot_pos = pos.rotated_y(t);
	// buf.write_debug(&format!("{:?} rot only\n", rot_pos));
	let rot_pos_up = rot_pos.added_vec(&(camera.up * rad_sc));
	// buf.write_debug(&format!("{:?} rot up\n", rot_pos_up));
	let rot_pos_up_proj = rot_pos_up.get_transformed_by_mat4x4_uniform(&render_mat_without_transform);
	// buf.write_debug(&format!("{:?} rot up projected\n", rot_pos_up_proj));
	let rot_pos_up_proj_2d_f32 = clip_space_to_screen_space_f32(&rot_pos_up_proj, buf.wid, buf.hei);


	let projected = pos.get_transformed_by_mat4x4_uniform(&buf.render_mat);
	// buf.write_debug(&format!("\n\n{:?} projected\n", projected));
	let projected_2d = clip_space_to_screen_space(&projected, buf.wid, buf.hei);
	// buf.write_debug(&format!("{:?} 2d of wh {:?}\n", projected_2d, IVec2::new(buf.wid.into(), buf.hei.into())));


	let screen_circ_f32 = clip_space_to_screen_space_f32(&projected, buf.wid, buf.hei);
	let dist = (rot_pos_up_proj_2d_f32.1 - screen_circ_f32.1).abs();

	let screen_circ = clip_space_to_screen_space(&projected, buf.wid, buf.hei);
	render_fill_bres_circle(&screen_circ, dist, ch, buf);

	safe_render_char_at('C', screen_circ.x, screen_circ.y, buf);
}
