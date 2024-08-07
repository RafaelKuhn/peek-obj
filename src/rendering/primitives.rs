use std::{f32::consts::TAU, num::Wrapping};

use crate::{*, camera::Camera, maths::*, timer::Timer, terminal::TerminalBuffer, ASCII_BYTES_PER_CHAR};


pub enum YadePrimitive {
	Ball(RenderBallData),
	Line(Line),
}

pub struct RenderBallData {
	pub height: f32,
	pub sq_dist_to_camera: f32,
	pub index: usize,
	pub screen_pos: IVec2,
	pub rad: f32,
}

pub struct Line {
	pub p0: IVec2,
	pub p1: IVec2,
}

pub struct ScreenTri {
	pub p0: IVec2,
	pub p1: IVec2,
	pub p2: IVec2,
}


impl Line {
	pub fn from_clip_space(p0: Vec4, p1: Vec4, wid: u16, hei: u16) -> Self {
		let p0 = clip_space_to_screen_space(&p0.homogeneous(), wid, hei);
		let p1 = clip_space_to_screen_space(&p1.homogeneous(), wid, hei);
		Line { p0, p1 }
	}

	pub fn from_screen_points(p0: FVec2, p1: FVec2) -> Self {
		Line { p0: p0.into(), p1: p1.into() }
	}
}

impl From<Line> for (IVec2, IVec2) {
	fn from(line: Line) -> Self {
		(line.p0, line.p1)
	}
}

impl From<(IVec2, IVec2)> for Line {
	fn from(value: (IVec2, IVec2)) -> Self {
		Line { p0: value.0, p1: value.1 }
	}
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


pub fn encode_char_in(ch: char, index: usize, vec: &mut [u8]) {
	ch.encode_utf8(&mut vec[index .. index+ASCII_BYTES_PER_CHAR]);
}

pub fn render_char_i(ch: char, pos: &IVec2, buffer: &mut TerminalBuffer) {
	debug_assert!(pos.x >= 0 && pos.x < buffer.wid.into());
	debug_assert!(pos.y >= 0 && pos.y < buffer.hei.into());

	render_char(ch, &pos.into(), buffer)
}

pub fn render_char(ch: char, pos: &UVec2, buffer: &mut TerminalBuffer) {
	debug_assert!(pos.x < buffer.wid);
	debug_assert!(pos.y < buffer.hei);

	let index = xy_to_it(pos.x, pos.y, buffer.wid);
	encode_char_in(ch, index, &mut buffer.raw_ascii_screen);
}

pub fn render_string(string: &str, pos: &UVec2, buf: &mut TerminalBuffer) {
	// string can't overflow the line
	assert!(pos.x as usize + string.len() - 1 < buf.wid.into(), "trying to render a string that overflows a line!");
	assert!(!string.contains('\n'), "can't render a string that has a line end!");

	let mut index = xy_to_it(pos.x, pos.y, buf.wid);
	for byte in string.bytes() {
		buf.raw_ascii_screen[index] = byte;
		index += ASCII_BYTES_PER_CHAR
	}
}

pub fn safe_render_string_signed(string: &str, x: Int, y: Int, buf: &mut TerminalBuffer) {
	if x < 0 || x as usize + string.len() > buf.wid.into() || y < 0 || y >= buf.hei as Int { return }
	render_string(string, &UVec2::new(x as u16, y as u16), buf);
}

pub fn safe_render_char_at(ch: char, x: Int, y: Int, buf: &mut TerminalBuffer) {
	if x < 0 || x >= buf.wid as Int || y < 0 || y >= buf.hei as Int { return }
	render_char(ch, &UVec2::new(x as u16, y as u16), buf);
}

pub fn safe_render_char_i(ch: char, pos: &IVec2, buf: &mut TerminalBuffer) {
	if pos.x < 0 || pos.x >= buf.wid as Int || pos.y < 0 || pos.y >= buf.hei as Int { return }
	render_char(ch, &pos.into(), buf);
}

pub fn render_straight_x_line_safe(p0x: Int, p1x: Int, y: Int, fill_char: char, buf: &mut TerminalBuffer) {
	
	if y < 0 || y >= buf.hei.into() { return; }

	// debug_assert!(p1x > p0x, "p1 larger, switch them?");

	// starts after the screen or ends before the screen
	if p0x >= buf.wid.into() || p1x < 0 { return }

	let p0x = p0x.clamp(0, (buf.wid - 1).into());
	let p1x = p1x.clamp(0, (buf.wid - 1).into());

	render_straight_x_line(p0x, p1x, y, fill_char, buf);
}

pub fn render_straight_x_line(p0x: Int, p1x: Int, y: Int, fill_char: char, buf: &mut TerminalBuffer) {

	let start = xy_to_it(p0x as u16, y as u16, buf.wid);
	let end_inclusive = xy_to_it(p1x as u16, y as u16, buf.wid);

	// this can happen if the ball is too far away
	if p0x > p1x { return }

	debug_assert!(fill_char.len_utf8() == 1, "NOT ASCII");
	let ascii_fill_char = fill_char as u8;

	// "safer" version
	#[cfg(debug_assertions)]
	buf.raw_ascii_screen[start..=end_inclusive].fill(ascii_fill_char);

	// faster version
	#[cfg(not(debug_assertions))]
	unsafe {
		let vec_start = buf.raw_ascii_screen.as_mut_ptr();
		let write_start = vec_start.add(start);

		let range = end_inclusive - start + 1;

		write_start.write_bytes(ascii_fill_char, range);
	}
}

pub fn render_bresenham_line(p0: &IVec2, p1: &IVec2, buf: &mut TerminalBuffer, fill_char: char) {
	let last_x = buf.wid - 1;
	let last_y = buf.hei - 1;

	// // cull lines completely out of the canvas
	if p0.x > last_x.into() && p1.x > last_x.into() { return }
	if p0.y > last_y.into() && p1.y > last_y.into() { return }

	let x0 = p0.x;
	let y0 = p0.y;
	let x1 = p1.x;
	let y1 = p1.y;

	let dx = (x1 - x0).abs();
	let dy = (y1 - y0).abs();
	let sx = if x0 < x1 { 1 } else { -1 };
	let sy = if y0 < y1 { 1 } else { -1 };

	let mut deriv_diff = dx - dy;
	let mut x = x0;
	let mut y = y0;

	// buf.write_debug(&format!("bresenham p0 {:} p1 {:}   dx {} dy {}  sx {} sy {} (w {} h {})\n", p0, p1, dx, dy, sy, sy, buf.wid, buf.hei));

	loop {

		// TODO: debug_assert, handle out of bounds above
		if x >= 0 && x < buf.wid.into() && y >= 0 && y < buf.hei.into() {
			// buf.write_debug(&format!("   bres char {} [{},{}]\n", fill_char, x, y));
			let index = xy_to_it(x as u16, y as u16, buf.wid);
			fill_char.encode_utf8(&mut buf.raw_ascii_screen[index..index + ASCII_BYTES_PER_CHAR]);
		}

		if x == x1 && y == y1 { return }

		// TODO: clip bresenham lines to screen
		// meanwhile use this version to prevent crashes in very long lines
		let double_deriv_diff = (Wrapping(deriv_diff) * Wrapping(2)).0;
		// let double_deriv_diff = deriv_diff * 2;

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

pub fn render_fill_bres_circle(pos: &IVec2, rad_x: f32, fill: char, buf: &mut TerminalBuffer) {

	// we have to divide by 2 because the radius calculation is done in X (in the terminal, X is double the Y)
	let sc_rad = (rad_x) / 2.0;

	let mut x = 0 as Int;
	let mut y = sc_rad as Int;

	let (base_x, base_y) = (pos.x, pos.y);

	let mut d = 3 - 2 * (sc_rad as Int);

	// I will always start rendering from the right side ->
	// and the first mirrored version will be the leftmost <-

	let scaled_y = y * 2;

	render_straight_x_line_safe(base_x - scaled_y, base_x + scaled_y, base_y, fill, buf);

	while y >= x {
		x += 1;
		if d > 0 {
			y -= 1;
			d = d + 4*(x-y) + 10;
		} else {
			d = d + 4*x + 6;
		}

		let scaled_x = x * 2;
		let scaled_y = y * 2;

		// let left_0 = IVec2::new(base_x - scaled_x, base_y + y);
		// let left_1 = IVec2::new(base_x - scaled_y, base_y + x);
		// let left_2 = IVec2::new(base_x - scaled_y, base_y - x);
		// let left_3 = IVec2::new(base_x - scaled_x, base_y - y);
		// let right_0 = IVec2::new(base_x + scaled_x, base_y + y);
		// let right_1 = IVec2::new(base_x + scaled_y, base_y + x);
		// let right_2 = IVec2::new(base_x + scaled_y, base_y - x);
		// let right_3 = IVec2::new(base_x + scaled_x, base_y - y);

		// render_bresenham_line(&left_0, &right_0, buf, fill);
		// render_bresenham_line(&left_1, &right_1, buf, fill);
		// render_bresenham_line(&left_2, &right_2, buf, fill);
		// render_bresenham_line(&left_3, &right_3, buf, fill);

		render_straight_x_line_safe(base_x - scaled_x, base_x + scaled_x, base_y + y, fill, buf);
		render_straight_x_line_safe(base_x - scaled_y, base_x + scaled_y, base_y + x, fill, buf);
		render_straight_x_line_safe(base_x - scaled_y, base_x + scaled_y, base_y - x, fill, buf);
		render_straight_x_line_safe(base_x - scaled_x, base_x + scaled_x, base_y - y, fill, buf);
	}
}

pub fn render_bres_circle(pos: &IVec2, rad: f32, ch: char, buf: &mut TerminalBuffer) {

	let mut x = 0 as Int;
	let mut y = rad as Int;

	let (base_x, base_y) = (pos.x, pos.y);

	let mut d = 3 - 2 * (rad as Int);

	// I will always start rendering from the right side ->
	// and the first mirrored version will be the leftmost <-

	// buf.write_debug(&format!("[{:}, {:}]\n", base_x + x, base_y + y));
	plot_mirrored_octets_safe(x, y, base_x, base_y, ch, buf);

	while y >= x {
		x += 1;
		if d > 0 {
			y -= 1;
			d = d + 4 * (x - y) + 10;
		} else {
			d = d + 4 * x + 6;
		}

		// buf.write_debug(&format!("[{:}, {:}]\n", base_x + x, base_y + y));
		plot_mirrored_octets_safe(x, y, base_x, base_y, ch, buf);
	}
}

pub fn plot_mirrored_octets_safe(x: Int, y: Int, base_x: Int, base_y: Int, ch: char, buf: &mut TerminalBuffer) {

	let scaled_x = x * 2;
	let scaled_y = y * 2;

	// left
	safe_render_char_at(ch, base_x - scaled_x, base_y + y, buf);
	safe_render_char_at(ch, base_x - scaled_y, base_y + x, buf);
	safe_render_char_at(ch, base_x - scaled_y, base_y - x, buf);
	safe_render_char_at(ch, base_x - scaled_x, base_y - y, buf);
	// right
	safe_render_char_at(ch, base_x + scaled_x, base_y + y, buf);
	safe_render_char_at(ch, base_x + scaled_y, base_y + x, buf);
	safe_render_char_at(ch, base_x + scaled_y, base_y - x, buf);
	safe_render_char_at(ch, base_x + scaled_x, base_y - y, buf);
}

pub fn test_render_spheres(buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
	// do in main:
	// camera.set_initial_pos(13.593422, 9.671257, 16.239632);
	// camera.set_initial_rot(0.392699, -0.687223, 0.000000);

	render_sphere(&Vec3::new(0.0, 0.0, 0.0), 0.25, '0', buf, timer, camera);

	let its = 20;
	for i in 0..its {
		let it = i as f32 / (its as f32 - 1.0);
		let angle = it * TAU;
		let (cos, sin) = (angle.cos(), angle.sin());

		let pos = Vec3::new(cos, sin, 0.0);
		render_sphere(&pos, 0.1, 'Z', buf, timer, camera);	
	}

	let mult = 3.0;
	let its = its * (mult * 0.8) as i32;
	for i in 0..its {
		let it = i as f32 / (its as f32 - 1.0);
		let angle = it * TAU;
		let (cos, sin) = (angle.cos() * mult, angle.sin() * mult);

		let pos = Vec3::new(sin, 0.0, cos);
		render_sphere(&pos, 0.1, 'Y', buf, timer, camera);
	}

	let mult = 2.0;
	let its = its * (mult * 0.8) as i32;
	for i in 0..its {
		let it = i as f32 / (its as f32 - 1.0);
		let angle = it * TAU;
		let (cos, sin) = (angle.cos() * mult, angle.sin() * mult);

		let pos = Vec3::new(0.0, cos, sin);
		render_sphere(&pos, 0.1, 'X', buf, timer, camera);
	}
}


#[deprecated]
pub fn render_sphere(pos: &Vec3, rad: f32, ch: char, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	buf.copy_projection_to_render_matrix();

	apply_identity_to_mat_4x4(&mut buf.transf_mat);

	let (scale_x, scale_y, scale_z) = (1.0, 1.0, 1.0);
	let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);
	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);

	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);

	// buf.clear_debug();


	let mut render_mat_without_transform = create_identity_4x4_arr();
	buf.copy_projection_to_mat4x4(&mut render_mat_without_transform);
	multiply_4x4_matrices(&mut render_mat_without_transform, &camera.view_matrix);


	let transformed_pos = pos.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
	// buf.write_debug(&format!("{:?} rot only\n", transformed_pos));
	let rot_pos_up = transformed_pos.add_vec(&(camera.up * rad));
	// buf.write_debug(&format!("{:?} rot up\n", rot_pos_up));
	let rot_pos_up_proj = rot_pos_up.get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform);
	// buf.write_debug(&format!("{:?} rot up projected\n", rot_pos_up_proj));

	let ball_pos_projected_clip = pos.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);

	let ball_up_projected_2d_f32 = clip_space_to_screen_space_f(&rot_pos_up_proj, buf.wid, buf.hei);
	let screen_circ_f32 = clip_space_to_screen_space_f(&ball_pos_projected_clip, buf.wid, buf.hei);
	let dist = (ball_up_projected_2d_f32.y - screen_circ_f32.y).abs();

	let screen_circ = clip_space_to_screen_space(&ball_pos_projected_clip, buf.wid, buf.hei);
	// calculating Y radius, is half of X because of terminal chars
	render_fill_bres_circle(&screen_circ, dist * 2.0, ch, buf);

	// safe_render_char_at('O', screen_circ.x, screen_circ.y, buf);
	// safe_render_char_at('U', ball_up_projected_2d_f32.x as i32, ball_up_projected_2d_f32.y as i32, buf);

	// render_bresenham_line(&screen_circ, &ball_up_projected_2d_f32.into(), buf, '*');
}