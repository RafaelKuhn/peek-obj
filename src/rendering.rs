// #![allow(unused_variables)]

pub mod mesh;
pub mod camera;
pub mod utils;

pub mod renderer;
pub mod yade_renderer;
pub mod obj_renderer;

use std::{f32::consts::TAU, fmt, io::Write};


use crate::{benchmark::Benchmark, file_readers::yade_dem_reader::YadeDemData, maths::*, terminal_wrapper::{TerminalBuffer}, timer::Timer};

use self::{camera::Camera, mesh::Mesh, utils::xy_to_it};


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

// TODO: figure out a way of drawing this with ascii only / braille
pub fn render_char(ch: char, pos: &UVec2, buffer: &mut TerminalBuffer) {
	debug_assert!(pos.x < buffer.wid.into());
	debug_assert!(pos.y < buffer.hei.into());

	let index = xy_to_it(pos.x, pos.y, buffer.wid);
	encode_char_in(ch, index, &mut buffer.vec);
}

pub fn encode_char_in(ch: char, index: usize, vec: &mut [u8]) {
	ch.encode_utf8(&mut vec[index .. index+ASCII_BYTES_PER_CHAR]);
}

pub fn render_string(string: &str, pos: &UVec2, buffer: &mut TerminalBuffer) {
	// string can't overflow the line
	debug_assert!(pos.x as usize + string.len() - 1 < buffer.wid.into());

	let mut index = xy_to_it(pos.x, pos.y, buffer.wid);
	for byte in string.bytes() {
		buffer.vec[index] = byte;
		index += ASCII_BYTES_PER_CHAR
	}
}

pub fn render_bresenham_line(p0: &UVec2, p1: &UVec2, buf: &mut TerminalBuffer, fill_char: char) {
	let last_x = buf.wid - 1;
	let last_y = buf.hei - 1;

	// cull lines completely out of the canvas
	if p0.x > last_x && p1.x > last_x { return }
	if p0.y > last_y && p1.y > last_y { return }

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

	debug_assert!(x >= 0 && y >= 0, "p0 and p1 are unsigned so it does not make sense for them to be negative");

	loop {

		// handle out of bounds
		if x < buf.wid.into() && y < buf.hei.into() {
			let index = xy_to_it(x as u16, y as u16, buf.wid);

			// TODO: figure out how would this work with UTF8
			fill_char.encode_utf8(&mut buf.vec[index..index + ASCII_BYTES_PER_CHAR]);
		}

		if x == x1 && y == y1 {
			break;
		}

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


pub fn render_benchmark(benchmark: &Benchmark, buffer: &mut TerminalBuffer) {
	let mut lowest_pos = UVec2::new(0, buffer.hei - 6);

	let wxh = buffer.wid as u32 * buffer.hei as u32;
	let aspect = buffer.wid as f32 / buffer.hei as f32;

	render_string(&format!("fps: {:.2}", benchmark.fps), &lowest_pos, buffer);
	lowest_pos.y += 1;
	render_string(&format!("dt: {:.4}ms", benchmark.delta_time_millis), &lowest_pos, buffer);
	lowest_pos.y += 1;
	render_string(&format!("time scale: {:.1}", benchmark.time_scale), &lowest_pos, buffer);
	lowest_pos.y += 1;
	render_string(&format!("scaled time: {:.2}", benchmark.time_aggr.as_millis() as f32 * 0.001), &lowest_pos, buffer);
	lowest_pos.y += 1;
	render_string(&format!("frame n: {}", benchmark.total_frame_count), &lowest_pos, buffer);
	lowest_pos.y += 1;
	render_string(&format!("w: {}, h: {}, w*h: {}, a: {:.2}", buffer.wid, buffer.hei, wxh, aspect), &lowest_pos, buffer);
}

pub const YADE_SCALE_TEMP: f32 = 15.0;

pub fn render_yade(yade_data: &YadeDemData, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	// TODO: figure out crappy camera
	let (pos_x, pos_y, pos_z) = (0.0, 0.5, 0.0);

	let start_ms = 89_340;
	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
	// let t = 0.0;
	let (angle_x, angle_y, angle_z) = (TAU * 0.25, t, 0.0);

	let (scale_x, scale_y, scale_z) = (YADE_SCALE_TEMP, YADE_SCALE_TEMP, YADE_SCALE_TEMP);

	buf.copy_projection_to_render_matrix();

	apply_identity_to_mat_4x4(&mut buf.transf_mat);
	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);


	for tri in yade_data.tris.iter() {

		let p0 = &tri.p0 + &tri.pos;
		let p1 = &tri.p1 + &tri.pos;
		let p2 = &tri.p2 + &tri.pos;

		let trs_p0 = p0.get_transformed_by_mat4x4(&buf.render_mat);
		let trs_p1 = p1.get_transformed_by_mat4x4(&buf.render_mat);
		let trs_p2 = p2.get_transformed_by_mat4x4(&buf.render_mat);

		let screen_p0 = clip_space_to_screen_space(&trs_p0, buf.wid, buf.hei);
		let screen_p1 = clip_space_to_screen_space(&trs_p1, buf.wid, buf.hei);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, buf.wid, buf.hei);

		render_bresenham_line(&screen_p0, &screen_p1, buf, FILL_CHAR);
		render_bresenham_line(&screen_p1, &screen_p2, buf, FILL_CHAR);
		render_bresenham_line(&screen_p2, &screen_p0, buf, FILL_CHAR);
	}

	// buf.clear_debug();
	// buf.write_debug(&format!("w {}, h {}\n", buf.wid, buf.hei));

	return;

	for (i, circ) in yade_data.circs.iter().enumerate() {

		let circ_pos = circ.pos.get_transformed_by_mat4x4(&buf.render_mat);
		let screen_circ = clip_space_to_screen_space(&circ_pos, buf.wid, buf.hei);

		// buf.write_debug(&format!("{: <4}", i));
		// buf.write_debug(&format!("{:?} ", &screen_circ));
		// buf.write_debug(&format!("{:?} ", screen_circ.x > buf.wid));
		// buf.write_debug("\n");


		// render_circle(&screen_circ, 15.0, buf);
		// render_char('R', &screen_circ, buf);
	}

	// buf.clear_debug();
	// buf.write_debug(&format!("_ {:} {:}\n", buf.wid, buf.hei));
	
	// for circ in yade_data.circs.iter() {
	for (i, circ) in yade_data.circs.iter().enumerate() {
		// if i >= 1 { break; }

		let circ_pos = circ.pos.get_transformed_by_mat4x4(&buf.render_mat);
		let screen_circ = clip_space_to_screen_space(&circ_pos, buf.wid, buf.hei);

		if screen_circ.x >= buf.wid { continue }
		render_circle(&screen_circ, 5., buf);
		// buf.write_debug(&format!("_ -> {:}\n", circ.rad * YADE_SCALE_TEMP));
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
	let (scale_x, scale_y, scale_z) = (1.0, 1.0, 1.0);

	buf.copy_projection_to_render_matrix();

	apply_identity_to_mat_4x4(&mut buf.transf_mat);
	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);



	let num_tris = mesh.tris_indices.len() / 3;
	for tri_i in 0..num_tris {

		let p0_i = tri_i * 3 + 0;
		let p1_i = tri_i * 3 + 1;
		let p2_i = tri_i * 3 + 2;

		let p0 = mesh.get_vert_at(p0_i);
		let p1 = mesh.get_vert_at(p1_i);
		let p2 = mesh.get_vert_at(p2_i);

		let trs_p0 = p0.get_transformed_by_mat4x4(&buf.render_mat);
		let trs_p1 = p1.get_transformed_by_mat4x4(&buf.render_mat);
		let trs_p2 = p2.get_transformed_by_mat4x4(&buf.render_mat);

		let screen_p0 = clip_space_to_screen_space(&trs_p0, buf.wid, buf.hei);
		let screen_p1 = clip_space_to_screen_space(&trs_p1, buf.wid, buf.hei);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, buf.wid, buf.hei);

		render_bresenham_line(&screen_p0, &screen_p1, buf, FILL_CHAR);
		render_bresenham_line(&screen_p1, &screen_p2, buf, FILL_CHAR);
		render_bresenham_line(&screen_p2, &screen_p0, buf, FILL_CHAR);
	}
}

fn screen_project(vec: &Vec3, render_mat: &[f32], wid: u16, hei: u16) -> UVec2 {
	let projected_3d = vec.get_transformed_by_mat4x4(render_mat);
	let projected_2d = clip_space_to_screen_space(&projected_3d, wid, hei);
	projected_2d
}

fn render_vec3_dbg(vec: &Vec3, pos: &UVec2, buf: &mut TerminalBuffer) {
	render_string(&format!("[{:+.2},{:+.2},{:+.2}]", vec.x, vec.y, vec.z), pos, buf);
}

fn render_uvec2_dbg(vec: &UVec2, pos: &UVec2, buf: &mut TerminalBuffer) {
	render_string(&format!("[{},{}]", vec.x, vec.y), pos, buf);
}

fn render_uvec_dbg(vec: &UVec2, pos: &UVec2, buf: &mut TerminalBuffer) {
	render_string(&format!("[{},{}]", vec.x, vec.y), pos, buf);
}

pub fn render_axes(buf: &mut TerminalBuffer, camera: &Camera) {

	buf.copy_projection_to_render_matrix();
	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);

	let origin  = screen_project(&Vec3::new(0.0, 0.0, 0.0), &buf.render_mat, buf.wid, buf.hei);

	const LINE_SZ: f32 = 10.0; 
	let up    = screen_project(&Vec3::new(0.0, LINE_SZ, 0.0), &buf.render_mat, buf.wid, buf.hei);
	let right = screen_project(&Vec3::new(LINE_SZ, 0.0, 0.0), &buf.render_mat, buf.wid, buf.hei);
	let front = screen_project(&Vec3::new(0.0, 0.0, LINE_SZ), &buf.render_mat, buf.wid, buf.hei);

	render_bresenham_line(&up, &origin, buf, '|');
	render_bresenham_line(&right, &origin, buf, '-');
	render_bresenham_line(&front, &origin, buf, '/');
}

// TODO: implement
pub fn render_sphere(pos: &Vec3, rad: f32, buf: &mut TerminalBuffer, timer: &Timer) {
// pub fn render_sphere(pos: &Vec3, buf: &mut TerminalBuffer, timer: &Timer) {

}

pub fn render_circle(pos: &UVec2, rad: f32, buf: &mut TerminalBuffer) {
	let mut x = 0 as i32;
	let mut y = rad as i32;

	let (base_x, base_y) = (pos.x as i32, pos.y as i32);

	let mut d = 3 - 2 * (rad as i32);

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


const CIRCLE_CHAR: char = '*';
fn safe_render_char_signed(x: i32, y: i32, buf: &mut TerminalBuffer) {
	if x < 0 || x >= buf.wid.into() || y < 0 || y >= buf.hei.into() { return }
	render_char(CIRCLE_CHAR, &UVec2::new(x as u16, y as u16), buf);
}

pub fn plot_mirrored_octets_safe(x: i32, y: i32, base_x: i32, base_y: i32, buf: &mut TerminalBuffer) {

	let scaled_x = x * 2;
	let scaled_y = y * 2;

	safe_render_char_signed(base_x + scaled_x, base_y + y, buf);
	safe_render_char_signed(base_x - scaled_x, base_y + y, buf);
	safe_render_char_signed(base_x + scaled_x, base_y - y, buf);
	safe_render_char_signed(base_x - scaled_x, base_y - y, buf);

	safe_render_char_signed(base_x + scaled_y, base_y + x, buf);
	safe_render_char_signed(base_x + scaled_y, base_y - x, buf);
	safe_render_char_signed(base_x - scaled_y, base_y + x, buf);
	safe_render_char_signed(base_x - scaled_y, base_y - x, buf);

	// return;

	// let x0 = (base_x + scaled_x) as u16; let y0 = (base_y + y) as u16;
	// buf.write_debug(&format!(" MIRRORED [{:}, {:}]\n", x0, y0));
	// let coord = xy_to_it(x0, y0, buf.wid);
	// encode_char_in(FILL_CHAR, coord, &mut buf.vec);
	
	// let x0 = (base_x - scaled_x) as u16; let y0 = (base_y + y) as u16;
	// buf.write_debug(&format!(" MIRRORED [{:}, {:}]\n", x0, y0));
	// let coord = xy_to_it(x0, y0, buf.wid);
	// encode_char_in(FILL_CHAR, coord, &mut buf.vec);
	
	// let x0 = (base_x + scaled_x) as u16; let y0 = (base_y - y) as u16;
	// buf.write_debug(&format!(" MIRRORED [{:}, {:}]\n", x0, y0));
	// let coord = xy_to_it(x0, y0, buf.wid);
	// encode_char_in(FILL_CHAR, coord, &mut buf.vec);
	
	// let x0 = (base_x - scaled_x) as u16; let y0 = (base_y - y) as u16;
	// buf.write_debug(&format!(" MIRRORED [{:}, {:}]\n", x0, y0));
	// let coord = xy_to_it(x0, y0, buf.wid);
	// encode_char_in(FILL_CHAR, coord, &mut buf.vec);

	
	// let x0 = (base_x + scaled_y) as u16; let y0 = (base_y + x) as u16;
	// buf.write_debug(&format!(" MIRRORED [{:}, {:}]\n", x0, y0));
	// let coord = xy_to_it(x0, y0, buf.wid);
	// encode_char_in(FILL_CHAR, coord, &mut buf.vec);
	
	// let x0 = (base_x + scaled_y) as u16; let y0 = (base_y - x) as u16;
	// buf.write_debug(&format!(" MIRRORED [{:}, {:}]\n", x0, y0));
	// let coord = xy_to_it(x0, y0, buf.wid);
	// encode_char_in(FILL_CHAR, coord, &mut buf.vec);
	
	// let x0 = (base_x - scaled_y) as u16; let y0 = (base_y + x) as u16;
	// buf.write_debug(&format!(" MIRRORED [{:}, {:}]\n", x0, y0));
	// let coord = xy_to_it(x0, y0, buf.wid);
	// encode_char_in(FILL_CHAR, coord, &mut buf.vec);
	
	// let x0 = (base_x - scaled_y) as u16; let y0 = (base_y - x) as u16;
	// buf.write_debug(&format!(" MIRRORED [{:}, {:}]\n", x0, y0));
	// let coord = xy_to_it(x0, y0, buf.wid);
	// encode_char_in(FILL_CHAR, coord, &mut buf.vec);
}







// pub fn draw_string(string: &str, pos: &UVec2, buffer: &mut [char], screen_width: u16) {
// 	let mut index = pos.y as usize * screen_width as usize + pos.x as usize;
// 	for ch in string.chars() {
// 		// TODO: bounds check
// 		if index > buffer.len() { continue; }
// 		buffer[index] = ch;
// 		index += 1;
// 	}
// }

// pub fn draw_char(ch: char, pos: &UVec2, buffer: &mut [char], screen_width: u16) {
// 	let index = pos.y as usize * screen_width as usize + pos.x as usize;
// 	if index > buffer.len() { return }
// 	buffer[index] = ch;
// }

// pub fn draw_mat4x4(mat: &[f32], pos: &UVec2, buffer: &mut [char], screen_width: u16) {
// 	let r0 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[ 0], mat[ 1], mat[ 2], mat[ 3]);
// 	draw_string(&r0, pos, buffer, screen_width);
	
// 	let r1 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[ 4], mat[ 5], mat[ 6], mat[ 7]);
// 	draw_string(&r1, &UVec2::new(pos.x, pos.y+1), buffer, screen_width);
	
// 	let r2 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[ 8], mat[ 9], mat[10], mat[11]);
// 	draw_string(&r2, &UVec2::new(pos.x, pos.y+2), buffer, screen_width);
	
// 	let r3 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[12], mat[13], mat[14], mat[15]);
// 	draw_string(&r3, &UVec2::new(pos.x, pos.y+3), buffer, screen_width);
// }

// pub fn draw_mesh_wire_and_normals(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), timer: &Timer, matrices: (&mut [f32], &mut [f32]), _camera: &Camera) {
// 	let (screen_width, screen_height) = width_height;
// 	let (proj_mat, transform_mat) = matrices;
	
// 	// apply_identity_to_mat_4x4(proj_mat);
// 	// apply_projection_to_mat_4x4(proj_mat, screen_width, screen_height);

// 	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

// 	let t = timer.time_aggr.as_millis() as f32 * 0.001;
// 	let (_angle_x, _angle_y, _angle_z) = (0.0, 0.0, 0.0);
// 	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);

// 	let _speed = 0.2;
// 	// let tmod = ((t * speed % 1.0) - 0.5).abs() * 2.0;
// 	let tmod = 0.6;
// 	let (scale_x, scale_y, scale_z) = (0.2 + 0.2 * tmod, 0.2 + 0.2 * tmod, 0.2 + 0.2 * tmod);


// 	apply_identity_to_mat_4x4(transform_mat);
	
// 	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
// 	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
// 	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

// 	multiply_4x4_matrices(proj_mat, transform_mat);

// 	let num_tris = mesh.tris_indices.len() / 3;
// 	for tri_i in 0..num_tris {

// 		let p0_i = tri_i * 3 + 0;
// 		let p1_i = tri_i * 3 + 1;
// 		let p2_i = tri_i * 3 + 2;


// 		let n0 = mesh.get_normal_at(p0_i);
// 		let trs_n0 = n0.get_transformed_by_mat4x4(proj_mat);

// 		let n1 = mesh.get_normal_at(p1_i);
// 		let trs_n1 = n1.get_transformed_by_mat4x4(proj_mat);

// 		let n2 = mesh.get_normal_at(p2_i);
// 		let trs_n2 = n2.get_transformed_by_mat4x4(proj_mat);

		
// 		let p0 = mesh.get_vert_at(p0_i);
// 		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);

// 		let p1 = mesh.get_vert_at(p1_i);
// 		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);

// 		let p2 = mesh.get_vert_at(p2_i);
// 		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);


// 		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
// 		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

// 		let screen_n0 = clip_space_to_screen_space(&(&trs_p0 + &trs_n0), screen_width, screen_height);
// 		let screen_n1 = clip_space_to_screen_space(&(&trs_p1 + &trs_n1), screen_width, screen_height);
// 		let screen_n2 = clip_space_to_screen_space(&(&trs_p2 + &trs_n2), screen_width, screen_height);

// 		// draw_string(&format!("n0 {:.2},{:.2},{:.2}", n0.x, n0.y, n0.z), &UVec2::new(0, 0), buffer, screen_width);
// 		// draw_string(&format!("n1 {:.2},{:.2},{:.2}", n1.x, n1.y, n1.z), &UVec2::new(0, 1), buffer, screen_width);
// 		// draw_string(&format!("n2 {:.2},{:.2},{:.2}", n2.x, n2.y, n2.z), &UVec2::new(0, 2), buffer, screen_width);

// 		// draw_string(&format!("n0 {:.2},{:.2},{:.2}", trs_n0.x, trs_n0.y, trs_n0.z), &UVec2::new(0, 4), buffer, screen_width);
// 		// draw_string(&format!("n1 {:.2},{:.2},{:.2}", trs_n1.x, trs_n1.y, trs_n1.z), &UVec2::new(0, 5), buffer, screen_width);
// 		// draw_string(&format!("n2 {:.2},{:.2},{:.2}", trs_n2.x, trs_n2.y, trs_n2.z), &UVec2::new(0, 6), buffer, screen_width);

// 		// draw_string(&format!("s n0 {},{}", screen_n0.x, screen_n0.y), &UVec2::new(0, 8), buffer, screen_width);
// 		// draw_string(&format!("s n1 {},{}", screen_n1.x, screen_n1.y), &UVec2::new(0, 9), buffer, screen_width);
// 		// draw_string(&format!("s n2 {},{}", screen_n2.x, screen_n2.y), &UVec2::new(0, 10), buffer, screen_width);

// 		draw_bresenham_line(&clip_space_to_screen_space(&trs_p0, screen_width, screen_height), &screen_n0, buffer, screen_width, '.');
// 		draw_bresenham_line(&screen_p1, &screen_n1, buffer, screen_width, '.');
// 		draw_bresenham_line(&screen_p2, &screen_n2, buffer, screen_width, '.');

// 		draw_bresenham_line(&clip_space_to_screen_space(&trs_p0, screen_width, screen_height), &screen_p1, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p2, &clip_space_to_screen_space(&trs_p0, screen_width, screen_height), buffer, screen_width, FILL_CHAR);

// 		draw_point(&screen_n0, buffer, screen_width, '@');
// 		draw_point(&screen_n1, buffer, screen_width, '@');
// 		draw_point(&screen_n2, buffer, screen_width, '@');
// 	}
// }

// pub fn draw_mesh_wire(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), timer: &Timer, matrices: (&mut [f32], &mut [f32]), _camera: &Camera) {
// 	let (screen_width, screen_height) = width_height;
// 	let (proj_mat, transform_mat) = matrices;

// 	apply_identity_to_mat_4x4(proj_mat);
// 	apply_projection_to_mat_4x4(proj_mat, screen_width, screen_height);


// 	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

// 	let start_ms = 89_340;
// 	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
// 	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);
// 	// let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);

// 	let speed = 0.3;
// 	let sharpness = 2.5;

// 	let tri_wave = triangle_wave(t * speed);
// 	let t_smooth_wave = smoothed_0_to_1(tri_wave, sharpness);
// 	let tmod = lerp_f32(0.2, 0.4, t_smooth_wave);
// 	// let tmod = 1.0;
// 	let (scale_x, scale_y, scale_z) = (tmod, tmod, tmod);

// 	draw_string(&format!("{:.2}", t), &UVec2::new(0, 0), buffer, screen_width);
// 	draw_string(&format!("{:.2}", t_smooth_wave), &UVec2::new(0, 1), buffer, screen_width);
// 	draw_string(&format!("{:.2}", tmod), &UVec2::new(0, 2), buffer, screen_width);

// 	apply_identity_to_mat_4x4(transform_mat);
// 	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
// 	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
// 	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

// 	multiply_4x4_matrices(proj_mat, transform_mat);

// 	let num_tris = mesh.tris_indices.len() / 3;
// 	for tri_i in 0..num_tris {

// 		let p0_i = tri_i * 3 + 0;
// 		let p1_i = tri_i * 3 + 1;
// 		let p2_i = tri_i * 3 + 2;

// 		let p0 = mesh.get_vert_at(p0_i);
// 		let p1 = mesh.get_vert_at(p1_i);
// 		let p2 = mesh.get_vert_at(p2_i);

// 		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);
// 		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);
// 		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);

// 		let screen_p0 = clip_space_to_screen_space(&trs_p0, screen_width, screen_height);
// 		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
// 		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

// 		draw_bresenham_line(&screen_p0, &screen_p1, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p2, &screen_p0, buffer, screen_width, FILL_CHAR);
// 	}
// }

// // TODO: could pass in a global data object with the timer and the matrices
// pub fn draw_mesh_filled(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), _timer: &Timer, matrices: (&mut [f32], &mut [f32]), camera: &Camera) {
// 	let (screen_width, screen_height) = width_height;
// 	let (proj_mat, transform_mat) = matrices;

// 	apply_identity_to_mat_4x4(proj_mat);
// 	apply_identity_to_mat_4x4(transform_mat);

// 	apply_projection_to_mat_4x4(proj_mat, screen_width, screen_height);

// 	// TODO: apply object scale and rotation here
// 	// apply_scale_to_mat_4x4(transform_mat, 1.0, 1.0, 1.0);
// 	// apply_rotation_to_mat_4x4(transform_mat, TAU * 3.8, TAU * 1.4, 0.0);
// 	apply_pos_to_mat_4x4(transform_mat, mesh.pos.x, mesh.pos.y, mesh.pos.z);

// 	// DRAWS 
// 	// let st = &format!("pos {:.2} {:.2} {:.2}", mesh.pos.x, mesh.pos.y, mesh.pos.z);
// 	// draw_string(st, &UVec2::new(0, 2), buffer, screen_width);
// 	// draw_string("transform", &UVec2::new(0, 3), buffer, screen_width);
// 	// draw_mat4x4(&transform_mat, &UVec2::new(4, 4), buffer, screen_width);

// 	multiply_4x4_matrices(transform_mat, &camera.view_matrix);

// 	// let st = &format!("pos {:.2} {:.2} {:.2}", camera.get_pos().x, camera.get_pos().y, camera.get_pos().z);
// 	// draw_string(st, &UVec2::new(0, 9), buffer, screen_width);
// 	// let st = &format!("rot {:.2} {:.2} {:.2}", camera.rotation.x, camera.rotation.y, camera.rotation.z);
// 	// draw_string(st, &UVec2::new(0, 10), buffer, screen_width);
// 	// draw_string("view", &UVec2::new(0, 11), buffer, screen_width);
// 	// draw_mat4x4(&camera.view_matrix, &UVec2::new(4, 12), buffer, screen_width);

// 	multiply_4x4_matrices(proj_mat, transform_mat);
// 	// draw_string("end proj mat", &UVec2::new(0, 35), buffer, screen_width);
// 	// draw_mat4x4(&proj_mat, &UVec2::new(4, 36), buffer, screen_width);

// 	let tris_amt = mesh.tris_indices.len() / 3;
// 	for tri_i in 0..tris_amt {
// 		let p0_i = tri_i * 3 + 0;
// 		let p1_i = tri_i * 3 + 1;
// 		let p2_i = tri_i * 3 + 2;

// 		// // TODO: remove
// 		// let p0 = mesh.get_vert_at(p0_i); // .get_transformed_by_mat4x4(proj_mat);
// 		// let p1 = mesh.get_vert_at(p1_i); // .get_transformed_by_mat4x4(proj_mat);
// 		// let p2 = mesh.get_vert_at(p2_i); // .get_transformed_by_mat4x4(proj_mat);
// 		// let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);
// 		// let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);
// 		// let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);

// 		let trs_p0 = mesh.get_vert_at(p0_i).get_transformed_by_mat4x4(proj_mat);
// 		let trs_p1 = mesh.get_vert_at(p1_i).get_transformed_by_mat4x4(proj_mat);
// 		let trs_p2 = mesh.get_vert_at(p2_i).get_transformed_by_mat4x4(proj_mat);

// 		let screen_p0 = clip_space_to_screen_space(&trs_p0, screen_width, screen_height);
// 		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
// 		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

// 		draw_bresenham_line(&screen_p0, &screen_p1, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p2, &screen_p0, buffer, screen_width, FILL_CHAR);
// 	}
// }

// pub fn draw_mesh_filled_and_normals(_screen_space_tris: &mut [ScreenTriangle], _buffer: &mut [char], _screen_width: u16) {
// 	todo!()
// }

// pub fn draw_yade(yade_data: &YadeDemData, buffer: &mut [char], width_height: (u16, u16), timer: &Timer, matrices: (&mut [f32], &mut [f32]), _camera: &Camera) {
// 	let (screen_width, screen_height) = width_height;
// 	let (proj_mat, transform_mat) = matrices;

// 	apply_identity_to_mat_4x4(proj_mat);
// 	apply_projection_to_mat_4x4(proj_mat, screen_width, screen_height);


// 	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

// 	let start_ms = 89_340;
// 	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
// 	let (angle_x, angle_y, angle_z) = (TAU * 0.25, t, 0.0);

// 	let speed = 0.3;
// 	let sharpness = 2.5;

// 	let tri_wave = triangle_wave(t * speed);
// 	let t_smooth_wave = smoothed_0_to_1(tri_wave, sharpness);
// 	let tmod = lerp_f32(0.2, 0.4, t_smooth_wave) * 15.0;
// 	// let tmod = 1.0;
// 	// let (scale_x, scale_y, scale_z) = (tmod, tmod, tmod);

// 	let scale = 12.0;
// 	let (scale_x, scale_y, scale_z) = (scale, scale, scale);

// 	draw_string(&format!("{:.2}", t), &UVec2::new(0, 0), buffer, screen_width);
// 	draw_string(&format!("{:.2}", t_smooth_wave), &UVec2::new(0, 1), buffer, screen_width);
// 	draw_string(&format!("{:.2}", tmod), &UVec2::new(0, 2), buffer, screen_width);

// 	apply_identity_to_mat_4x4(transform_mat);
// 	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
// 	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
// 	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

// 	multiply_4x4_matrices(proj_mat, transform_mat);

// 	for tri in yade_data.tris.iter() {

// 		let p0 = &tri.p0 + &tri.pos;
// 		let p1 = &tri.p1 + &tri.pos;
// 		let p2 = &tri.p2 + &tri.pos;

// 		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);
// 		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);
// 		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);

// 		let screen_p0 = clip_space_to_screen_space(&trs_p0, screen_width, screen_height);
// 		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
// 		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

// 		draw_bresenham_line(&screen_p0, &screen_p1, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p2, &screen_p0, buffer, screen_width, FILL_CHAR);
// 	}

// 	for circ in yade_data.circs.iter() {

// 		let circ_pos = circ.pos.get_transformed_by_mat4x4(proj_mat);
// 		let screen_circ = clip_space_to_screen_space(&circ_pos, screen_width, screen_height);

// 		draw_char('R', &screen_circ, buffer, screen_width);
// 	}
// }


// pub fn draw_point(p: &UVec2, buffer: &mut [char], screen_width: u16, fill_char: char) {
// 	let index: usize = p.y as usize * screen_width as usize + p.x as usize;
// 	if index < buffer.len() {
// 		buffer[index] = fill_char;
// 	}
// }

// fn draw_bresenham_line(p0: &UVec2, p1: &UVec2, buffer: &mut [char], screen_width: u16, fill_char: char) {
// 	let x0 = p0.x as i32;
// 	let y0 = p0.y as i32;
// 	let x1 = p1.x as i32;
// 	let y1 = p1.y as i32;
	
// 	let mut x = x0;
// 	let mut y = y0;
// 	let dx = (x1 - x0).abs();
// 	let dy = (y1 - y0).abs();
// 	let sx = if x0 < x1 { 1 } else { -1 };
// 	let sy = if y0 < y1 { 1 } else { -1 };
// 	let mut deriv_diff = dx - dy;

// 	let i_screen_width = screen_width as i32;
// 	let mut index: usize;
// 	loop {
// 		index = (y * i_screen_width + x) as usize;
			
// 		// handle out of bounds
// 		if index < buffer.len() {
// 			buffer[index] = fill_char;
// 		}

// 		if x == x1 && y == y1 {
// 			break;
// 		}

// 		let double_deriv_diff = deriv_diff * 2;
// 		if double_deriv_diff > -dy {
// 			deriv_diff -= dy;
// 			x += sx;
// 		}
// 		if double_deriv_diff < dx {
// 			deriv_diff += dx;
// 			y += sy;
// 		}
// 	}

// }

// ////////////
// // TESTS: //
// ////////////

// pub fn draw_triangles_wire(screen_space_tris: &[ScreenTriangle], buffer: &mut [char], screen_width: u16) {
// 	for (index, tri) in screen_space_tris.iter().enumerate() {
// 		let i = index as u16;
// 		draw_bresenham_line(&tri.p0, &tri.p1, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&tri.p1, &tri.p2, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&tri.p2, &tri.p0, buffer, screen_width, FILL_CHAR);


// 		draw_string(&format!("p0 {:?}", &tri.p0), &UVec2 { x: &tri.p0.x + 3, y: &tri.p0.y - 2 + i }, buffer, screen_width);
// 		draw_string(&format!("p1 {:?}", &tri.p1), &UVec2 { x: &tri.p1.x - 3, y: &tri.p1.y + 2 + i }, buffer, screen_width);
// 		draw_string(&format!("p2 {:?}", &tri.p2), &UVec2 { x: &tri.p2.x + 3, y: &tri.p2.y - 1 + i }, buffer, screen_width);
		
// 		let (topmost, secmost, trimost) = sort_by_y_prefer_left(&tri.p0, &tri.p1, &tri.p2);

// 		// TODO: learn the vector shit behind this
// 		let shortside_in_left =
// 			(secmost.y as f32 - topmost.y as f32) * (trimost.x as f32 - topmost.x as f32) >
// 			(secmost.x as f32 - topmost.x as f32) * (trimost.y as f32 - topmost.y as f32);
// 		if shortside_in_left {
// 			draw_string("the bend is on left",  &UVec2 { x: 4, y: 4+i }, buffer, screen_width);
// 		} else {
// 			draw_string("the bend is on right", &UVec2 { x: 4, y: 4+i }, buffer, screen_width);
// 		}

// 		// TODO: bounds check
// 		let index = (tri.p0.y * screen_width + tri.p0.x) as usize;
// 		buffer[index] = '@';
		
// 		let index = (tri.p1.y * screen_width + tri.p1.x) as usize;
// 		buffer[index] = '@';

// 		let index = (tri.p2.y * screen_width + tri.p2.x) as usize;
// 		buffer[index] = '@';
// 	}
// }

// // TODO: decent test
// pub fn test_bresenham(buffer: &mut [char], screen_width: u16, screen_height: u16, time_seed: i32) {
// 	draw_string(&format!("w:{}, h:{}", screen_width, screen_height), &UVec2::new(0, 0), buffer, screen_width);

// 	let middle = UVec2::new(screen_width / 2, screen_height / 2);
	
// 	let len = 20.0;
// 	let modulus = time_seed / 2 % 1000;
// 	let t = modulus as f32 / 1000.0;
// 	// let t_2 = ((t-0.5)).abs() * 2.0;

// 	let angle = t * TAU;

// 	let x = (angle.cos() * len * 2.0) as i16;
// 	let y = (angle.sin() * len) as i16;

// 	let up = UVec2::new((middle.x as i16 + x) as u16, (middle.y as i16 + y) as u16);

// 	// let up = UVec2::new((middle.x as i16) as u16, (middle.y as i16 + 15) as u16);
// 	// let up = UVec2::new((middle.x + 15), middle.y + 7);


// 	let direction: char;
// 	if angle < (TAU * 1.0/8.0) {
// 		direction = '↘';
// 	} else if angle < (TAU * 2.0/8.0) {
// 		direction = '↓';
// 	} else if angle < (TAU * 3.0/8.0) {
// 		direction = '↙';
// 	} else if angle < (TAU * 4.0/8.0) {
// 		direction = '←';
// 	} else if angle < (TAU * 5.0/8.0) {
// 		direction = '↖';
// 	} else if angle < (TAU * 6.0/8.0) {
// 		direction = '↑';
// 	} else if angle < (TAU * 7.0/8.0) {
// 		direction = '↗';
// 	} else {
// 		direction = '→';
// 	}

// 	draw_bresenham_line(&middle, &up, buffer, screen_width, direction);

// 	draw_point(&up, buffer, screen_width, '@');

// 	draw_string(&format!("{}", angle), &UVec2::new(0, 1), buffer, screen_width);
// 	draw_string(&format!("{}", up),    &UVec2::new(up.x+2, up.y), buffer, screen_width);

// 	// let right = &UVec2::new(middle.x + len, middle.y);
// 	// let left  = &UVec2::new(middle.x - len, middle.y);
// 	// let up    = &UVec2::new(middle.x, middle.y - len);
// 	// let down  = &UVec2::new(middle.x, middle.y + len/4);
	
// 	// let up_r  = &UVec2::new(right.x, up.y);
// 	// let up_l  = &UVec2::new(left.x, up.y);
// 	// ↖ ↑ ↗
// 	// ← · →
// 	// ↙ ↓ ↘

// 	// draw_bresenham_line(&middle, right, buffer, screen_width, '→');
// 	// draw_bresenham_line(&middle, left,  buffer, screen_width, '←');
// 	// draw_bresenham_line(&middle, up,    buffer, screen_width, '↑');
// 	// draw_bresenham_line(&middle, down,  buffer, screen_width, '↓');

// 	// draw_bresenham_line(&middle, up_l,  buffer, screen_width, '↖');
// 	// draw_bresenham_line(&middle, up_r,  buffer, screen_width, '↗');
	
// 	// draw_point(&middle, buffer, screen_width, '·');
// }


// fn sort_by_y_prefer_left<'a>(p0: &'a UVec2, p1: &'a UVec2, p2: &'a UVec2) -> (&'a UVec2, &'a UVec2, &'a UVec2) {
// 	let topmost: &UVec2;
// 	let secmost: &UVec2;
// 	let trimost: &UVec2;

// 	if p0.y < p1.y && p0.y < p2.y {
// 		topmost = p0;
// 		if p1.y == p2.y {
// 			(trimost, secmost) = sort_by_x(p1, p2);
// 		} else {
// 			(trimost, secmost) = sort_by_y(p1, p2);
// 		}
// 	} else if p1.y < p0.y && p1.y < p2.y {
// 		topmost = p1;
// 		if p0.y == p2.y {
// 			(trimost, secmost) = sort_by_x(p0, p2);
// 		} else {
// 			(trimost, secmost) = sort_by_y(p0, p2);
// 		}
// 	} else {
// 		topmost = p2;
// 		if p0.y == p1.y {
// 			(trimost, secmost) = sort_by_x(p0, p1)
// 		} else {
// 			(trimost, secmost) = sort_by_y(p0, p1);
// 		}
// 	}

// 	(topmost, secmost, trimost)
// }

// fn sort_by_x<'a>(first: &'a UVec2, sec: &'a UVec2) -> (&'a UVec2, &'a UVec2) {
// 	if first.x > sec.x { (first, sec) } else { (sec, first) }
// }

// fn sort_by_y<'a>(first: &'a UVec2, sec: &'a UVec2) -> (&'a UVec2, &'a UVec2) {
// 	if first.y > sec.y { (first, sec) } else { (sec, first) }
// }
