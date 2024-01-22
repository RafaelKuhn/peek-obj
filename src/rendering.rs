#![allow(unused_variables)]

pub mod mesh;
pub mod camera;

use std::f32::consts::TAU;

use crate::{maths::*, benchmark::Benchmark, timer::AppTimer};

use self::{mesh::Mesh, camera::Camera};


// ascii luminance:
// . , - ~ : ; = ! & # @"
pub static BACKGROUND_FILL_CHAR: char = ' ';

// static LUMIN: &str = " .,-~:;=!&#@";
// static DIRS: &str =
//   "↖ ↑ ↗" +
//   "← · →" +
//   "↙ ↓ ↘";

static FILL_CHAR: char = '*';


#[derive(Debug)]
pub struct ScreenTriangle {
	pub p0: UVec2,
	pub p1: UVec2,
	pub p2: UVec2,
}



pub fn render_clear(buffer: &mut [char]) {
	for ch in buffer { *ch = BACKGROUND_FILL_CHAR; }
	// same as
	// for i in 0..buffer.len() { buffer[i] = BACKGROUND_FILL_CHAR; }
}

pub fn draw_string(str: &str, pos: &UVec2, buffer: &mut [char], screen_width: u16) {
	let mut index = pos.y as usize * screen_width as usize + pos.x as usize;
	for ch in str.chars() {
		// TODO: bounds check
		if index > buffer.len() { continue; }
		buffer[index] = ch;
		index += 1;
	}
}

pub fn draw_mat4x4(mat: &[f32], pos: &UVec2, buffer: &mut [char], screen_width: u16) {
	let r0 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[ 0], mat[ 1], mat[ 2], mat[ 3]);
	draw_string(&r0, pos, buffer, screen_width);
	
	let r1 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[ 4], mat[ 5], mat[ 6], mat[ 7]);
	draw_string(&r1, &UVec2::new(pos.x, pos.y+1), buffer, screen_width);
	
	let r2 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[ 8], mat[ 9], mat[10], mat[11]);
	draw_string(&r2, &UVec2::new(pos.x, pos.y+2), buffer, screen_width);
	
	let r3 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[12], mat[13], mat[14], mat[15]);
	draw_string(&r3, &UVec2::new(pos.x, pos.y+3), buffer, screen_width);
}

pub fn draw_mesh_wire_and_normals(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), timer: &AppTimer, matrices: (&mut [f32], &mut [f32]), camera: &Camera) {
	let (screen_width, screen_height) = width_height;
	let (proj_mat, transform_mat) = matrices;
	
	apply_identity_to_mat_4x4(proj_mat);
	apply_projection_to_mat_4x4(proj_mat, width_height);

	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

	let t = timer.time_aggr.as_millis() as f32 * 0.001;
	let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);
	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);

	let speed = 0.2;
	// let tmod = ((t * speed % 1.0) - 0.5).abs() * 2.0;
	let tmod = 0.6;
	let (scale_x, scale_y, scale_z) = (0.2 + 0.2 * tmod, 0.2 + 0.2 * tmod, 0.2 + 0.2 * tmod);


	apply_identity_to_mat_4x4(transform_mat);
	
	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(proj_mat, transform_mat);

	let num_tris = mesh.tris_indices.len() / 3;
	for tri_i in 0..num_tris {

		let p0_i = tri_i * 3 + 0;
		let p1_i = tri_i * 3 + 1;
		let p2_i = tri_i * 3 + 2;


		let n0 = mesh.get_normal_at(p0_i);
		let trs_n0 = n0.get_transformed_by_mat4x4(proj_mat);

		let n1 = mesh.get_normal_at(p1_i);
		let trs_n1 = n1.get_transformed_by_mat4x4(proj_mat);

		let n2 = mesh.get_normal_at(p2_i);
		let trs_n2 = n2.get_transformed_by_mat4x4(proj_mat);

		
		let p0 = mesh.get_vert_at(p0_i);
		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);

		let p1 = mesh.get_vert_at(p1_i);
		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);

		let p2 = mesh.get_vert_at(p2_i);
		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);


		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

		let screen_n0 = clip_space_to_screen_space(&(&trs_p0 + &trs_n0), screen_width, screen_height);
		let screen_n1 = clip_space_to_screen_space(&(&trs_p1 + &trs_n1), screen_width, screen_height);
		let screen_n2 = clip_space_to_screen_space(&(&trs_p2 + &trs_n2), screen_width, screen_height);

		// draw_string(&format!("n0 {:.2},{:.2},{:.2}", n0.x, n0.y, n0.z), &UVec2::new(0, 0), buffer, screen_width);
		// draw_string(&format!("n1 {:.2},{:.2},{:.2}", n1.x, n1.y, n1.z), &UVec2::new(0, 1), buffer, screen_width);
		// draw_string(&format!("n2 {:.2},{:.2},{:.2}", n2.x, n2.y, n2.z), &UVec2::new(0, 2), buffer, screen_width);

		// draw_string(&format!("n0 {:.2},{:.2},{:.2}", trs_n0.x, trs_n0.y, trs_n0.z), &UVec2::new(0, 4), buffer, screen_width);
		// draw_string(&format!("n1 {:.2},{:.2},{:.2}", trs_n1.x, trs_n1.y, trs_n1.z), &UVec2::new(0, 5), buffer, screen_width);
		// draw_string(&format!("n2 {:.2},{:.2},{:.2}", trs_n2.x, trs_n2.y, trs_n2.z), &UVec2::new(0, 6), buffer, screen_width);

		// draw_string(&format!("s n0 {},{}", screen_n0.x, screen_n0.y), &UVec2::new(0, 8), buffer, screen_width);
		// draw_string(&format!("s n1 {},{}", screen_n1.x, screen_n1.y), &UVec2::new(0, 9), buffer, screen_width);
		// draw_string(&format!("s n2 {},{}", screen_n2.x, screen_n2.y), &UVec2::new(0, 10), buffer, screen_width);

		draw_bresenham_line(&clip_space_to_screen_space(&trs_p0, screen_width, screen_height), &screen_n0, buffer, screen_width, '.');
		draw_bresenham_line(&screen_p1, &screen_n1, buffer, screen_width, '.');
		draw_bresenham_line(&screen_p2, &screen_n2, buffer, screen_width, '.');

		draw_bresenham_line(&clip_space_to_screen_space(&trs_p0, screen_width, screen_height), &screen_p1, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p2, &clip_space_to_screen_space(&trs_p0, screen_width, screen_height), buffer, screen_width, FILL_CHAR);

		draw_point(&screen_n0, buffer, screen_width, '@');
		draw_point(&screen_n1, buffer, screen_width, '@');
		draw_point(&screen_n2, buffer, screen_width, '@');
	}
}

pub fn draw_mesh_wire(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), timer: &AppTimer, matrices: (&mut [f32], &mut [f32]), camera: &Camera) {
	let (screen_width, screen_height) = width_height;
	let (proj_mat, transform_mat) = matrices;

	apply_identity_to_mat_4x4(proj_mat);
	apply_projection_to_mat_4x4(proj_mat, width_height);


	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

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

	draw_string(&format!("{:.2}", t), &UVec2::new(0, 0), buffer, screen_width);
	draw_string(&format!("{:.2}", t_smooth_wave), &UVec2::new(0, 1), buffer, screen_width);
	draw_string(&format!("{:.2}", tmod), &UVec2::new(0, 2), buffer, screen_width);

	apply_identity_to_mat_4x4(transform_mat);
	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(proj_mat, transform_mat);

	let num_tris = mesh.tris_indices.len() / 3;
	for tri_i in 0..num_tris {

		let p0_i = tri_i * 3 + 0;
		let p1_i = tri_i * 3 + 1;
		let p2_i = tri_i * 3 + 2;

		let p0 = mesh.get_vert_at(p0_i);
		let p1 = mesh.get_vert_at(p1_i);
		let p2 = mesh.get_vert_at(p2_i);

		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);
		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);
		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);

		let screen_p0 = clip_space_to_screen_space(&trs_p0, screen_width, screen_height);
		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

		draw_bresenham_line(&screen_p0, &screen_p1, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p2, &screen_p0, buffer, screen_width, FILL_CHAR);
	}
}

// TODO: could pass in a global data object with the timer and the matrices
pub fn draw_mesh_filled(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), timer: &AppTimer, matrices: (&mut [f32], &mut [f32]), camera: &Camera) {
	let (screen_width, screen_height) = width_height;
	let (proj_mat, transform_mat) = matrices;

	apply_identity_to_mat_4x4(proj_mat);
	apply_identity_to_mat_4x4(transform_mat);

	apply_projection_to_mat_4x4(proj_mat, width_height);

	// TODO: apply object scale and rotation here
	// apply_scale_to_mat_4x4(transform_mat, 1.0, 1.0, 1.0);
	// apply_rotation_to_mat_4x4(transform_mat, TAU * 3.8, TAU * 1.4, 0.0);
	apply_pos_to_mat_4x4(transform_mat, mesh.pos.x, mesh.pos.y, mesh.pos.z);

	// DRAWS 
	// let st = &format!("pos {:.2} {:.2} {:.2}", mesh.pos.x, mesh.pos.y, mesh.pos.z);
	// draw_string(st, &UVec2::new(0, 2), buffer, screen_width);
	// draw_string("transform", &UVec2::new(0, 3), buffer, screen_width);
	// draw_mat4x4(&transform_mat, &UVec2::new(4, 4), buffer, screen_width);

	multiply_4x4_matrices(transform_mat, &camera.view_matrix);

	// let st = &format!("pos {:.2} {:.2} {:.2}", camera.get_pos().x, camera.get_pos().y, camera.get_pos().z);
	// draw_string(st, &UVec2::new(0, 9), buffer, screen_width);
	// let st = &format!("rot {:.2} {:.2} {:.2}", camera.rotation.x, camera.rotation.y, camera.rotation.z);
	// draw_string(st, &UVec2::new(0, 10), buffer, screen_width);
	// draw_string("view", &UVec2::new(0, 11), buffer, screen_width);
	// draw_mat4x4(&camera.view_matrix, &UVec2::new(4, 12), buffer, screen_width);

	multiply_4x4_matrices(proj_mat, transform_mat);
	// draw_string("end proj mat", &UVec2::new(0, 35), buffer, screen_width);
	// draw_mat4x4(&proj_mat, &UVec2::new(4, 36), buffer, screen_width);

	let tris_amt = mesh.tris_indices.len() / 3;
	for tri_i in 0..tris_amt {
		let p0_i = tri_i * 3 + 0;
		let p1_i = tri_i * 3 + 1;
		let p2_i = tri_i * 3 + 2;

		// // TODO: remove
		// let p0 = mesh.get_vert_at(p0_i); // .get_transformed_by_mat4x4(proj_mat);
		// let p1 = mesh.get_vert_at(p1_i); // .get_transformed_by_mat4x4(proj_mat);
		// let p2 = mesh.get_vert_at(p2_i); // .get_transformed_by_mat4x4(proj_mat);
		// let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);
		// let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);
		// let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);

		let trs_p0 = mesh.get_vert_at(p0_i).get_transformed_by_mat4x4(proj_mat);
		let trs_p1 = mesh.get_vert_at(p1_i).get_transformed_by_mat4x4(proj_mat);
		let trs_p2 = mesh.get_vert_at(p2_i).get_transformed_by_mat4x4(proj_mat);

		let screen_p0 = clip_space_to_screen_space(&trs_p0, screen_width, screen_height);
		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

		draw_bresenham_line(&screen_p0, &screen_p1, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p2, &screen_p0, buffer, screen_width, FILL_CHAR);
	}
}

pub fn draw_mesh_filled_and_normals(screen_space_tris: &mut [ScreenTriangle], buffer: &mut [char], screen_width: u16) {
	todo!()
}

pub fn draw_benchmark(buffer: &mut [char], screen_width: u16, screen_height: u16, benchmark: &Benchmark) {
	let mut lowest_pos = UVec2::new(0, screen_height - 3 - 6);

	draw_string(&format!("time scale: {:.1} playing: {}", if benchmark.is_paused { 0.0 } else { 1.0 },  !benchmark.is_paused), &lowest_pos, buffer, screen_width);
	lowest_pos.y += 1;
	draw_string(&format!("dt: {:.2}ms", benchmark.delta_time_millis), &lowest_pos, buffer, screen_width);
	lowest_pos.y += 1;
	draw_string(&format!("fps: {}", benchmark.fps),                   &lowest_pos, buffer, screen_width);
	lowest_pos.y += 1;
	draw_string(&format!("w: {}, h: {}, w*h: {}", screen_width, screen_height, screen_width as u32 * screen_height as u32), &lowest_pos, buffer, screen_width);
	lowest_pos.y += 1;
	draw_string(&format!("iteration: {}", benchmark.total_frame_count),  &lowest_pos, buffer, screen_width);
}

pub fn draw_timer(buffer: &mut [char], screen_width: u16, screen_height: u16, timer: &AppTimer) {
	draw_string(&format!("scaled time: {:.4}", timer.time_aggr.as_millis()), &UVec2::new(0, screen_height - 3), buffer, screen_width);
}

pub fn draw_point(p: &UVec2, buffer: &mut [char], screen_width: u16, fill_char: char) {
	let index: usize = p.y as usize * screen_width as usize + p.x as usize;
	if index < buffer.len() {
		buffer[index] = fill_char;
	}
}

fn draw_bresenham_line(p0: &UVec2, p1: &UVec2, buffer: &mut [char], screen_width: u16, fill_char: char) {
	let x0 = p0.x as i32;
	let y0 = p0.y as i32;
	let x1 = p1.x as i32;
	let y1 = p1.y as i32;
	
	let mut x = x0;
	let mut y = y0;
	let dx = (x1 - x0).abs();
	let dy = (y1 - y0).abs();
	let sx = if x0 < x1 { 1 } else { -1 };
	let sy = if y0 < y1 { 1 } else { -1 };
	let mut deriv_diff = dx - dy;

	let i_screen_width = screen_width as i32;
	let mut index: usize;
	loop {
		index = (y * i_screen_width + x) as usize;
			
		// handle out of bounds
		if index < buffer.len() {
			buffer[index] = fill_char;
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

////////////
// TESTS: //
////////////

pub fn draw_triangles_wire(screen_space_tris: &[ScreenTriangle], buffer: &mut [char], screen_width: u16) {
	for (index, tri) in screen_space_tris.iter().enumerate() {
		let i = index as u16;
		draw_bresenham_line(&tri.p0, &tri.p1, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&tri.p1, &tri.p2, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&tri.p2, &tri.p0, buffer, screen_width, FILL_CHAR);


		draw_string(&format!("p0 {:?}", &tri.p0), &UVec2 { x: &tri.p0.x + 3, y: &tri.p0.y - 2 + i }, buffer, screen_width);
		draw_string(&format!("p1 {:?}", &tri.p1), &UVec2 { x: &tri.p1.x - 3, y: &tri.p1.y + 2 + i }, buffer, screen_width);
		draw_string(&format!("p2 {:?}", &tri.p2), &UVec2 { x: &tri.p2.x + 3, y: &tri.p2.y - 1 + i }, buffer, screen_width);
		
		let (topmost, secmost, trimost) = sort_by_y_prefer_left(&tri.p0, &tri.p1, &tri.p2);

		// TODO: learn the vector shit behind this
		let shortside_in_left =
			(secmost.y as f32 - topmost.y as f32) * (trimost.x as f32 - topmost.x as f32) >
			(secmost.x as f32 - topmost.x as f32) * (trimost.y as f32 - topmost.y as f32);
		if shortside_in_left {
			draw_string("the bend is on left",  &UVec2 { x: 4, y: 4+i }, buffer, screen_width);
		} else {
			draw_string("the bend is on right", &UVec2 { x: 4, y: 4+i }, buffer, screen_width);
		}

		// TODO: bounds check
		let index = (tri.p0.y * screen_width + tri.p0.x) as usize;
		buffer[index] = '@';
		
		let index = (tri.p1.y * screen_width + tri.p1.x) as usize;
		buffer[index] = '@';

		let index = (tri.p2.y * screen_width + tri.p2.x) as usize;
		buffer[index] = '@';
	}
}

// TODO: decent test
pub fn test_bresenham(buffer: &mut [char], screen_width: u16, screen_height: u16, time_seed: i32) {
	draw_string(&format!("w:{}, h:{}", screen_width, screen_height), &UVec2::new(0, 0), buffer, screen_width);

	let middle = UVec2::new(screen_width / 2, screen_height / 2);
	
	let len = 20.0;
	let modulus = time_seed / 2 % 1000;
	let t = modulus as f32 / 1000.0;
	// let t_2 = ((t-0.5)).abs() * 2.0;

	let angle = t * TAU;

	let x = (angle.cos() * len * 2.0) as i16;
	let y = (angle.sin() * len) as i16;

	let up = UVec2::new((middle.x as i16 + x) as u16, (middle.y as i16 + y) as u16);

	// let up = UVec2::new((middle.x as i16) as u16, (middle.y as i16 + 15) as u16);
	// let up = UVec2::new((middle.x + 15), middle.y + 7);


	let direction: char;
	if angle < (TAU * 1.0/8.0) {
		direction = '↘';
	} else if angle < (TAU * 2.0/8.0) {
		direction = '↓';
	} else if angle < (TAU * 3.0/8.0) {
		direction = '↙';
	} else if angle < (TAU * 4.0/8.0) {
		direction = '←';
	} else if angle < (TAU * 5.0/8.0) {
		direction = '↖';
	} else if angle < (TAU * 6.0/8.0) {
		direction = '↑';
	} else if angle < (TAU * 7.0/8.0) {
		direction = '↗';
	} else {
		direction = '→';
	}

	draw_bresenham_line(&middle, &up, buffer, screen_width, direction);

	draw_point(&up, buffer, screen_width, '@');

	draw_string(&format!("{}", angle), &UVec2::new(0, 1), buffer, screen_width);
	draw_string(&format!("{}", up),    &UVec2::new(up.x+2, up.y), buffer, screen_width);

	// let right = &UVec2::new(middle.x + len, middle.y);
	// let left  = &UVec2::new(middle.x - len, middle.y);
	// let up    = &UVec2::new(middle.x, middle.y - len);
	// let down  = &UVec2::new(middle.x, middle.y + len/4);
	
	// let up_r  = &UVec2::new(right.x, up.y);
	// let up_l  = &UVec2::new(left.x, up.y);
	// ↖ ↑ ↗
	// ← · →
	// ↙ ↓ ↘

	// draw_bresenham_line(&middle, right, buffer, screen_width, '→');
	// draw_bresenham_line(&middle, left,  buffer, screen_width, '←');
	// draw_bresenham_line(&middle, up,    buffer, screen_width, '↑');
	// draw_bresenham_line(&middle, down,  buffer, screen_width, '↓');

	// draw_bresenham_line(&middle, up_l,  buffer, screen_width, '↖');
	// draw_bresenham_line(&middle, up_r,  buffer, screen_width, '↗');
	
	// draw_point(&middle, buffer, screen_width, '·');
}

fn slope_of_line(p0: &UVec2, p1: &UVec2) -> f32 {
	(p1.y as f32 - p0.y as f32) / (p1.x as f32 - p0.x as f32)
}

fn sort_by_y_prefer_left<'a>(p0: &'a UVec2, p1: &'a UVec2, p2: &'a UVec2) -> (&'a UVec2, &'a UVec2, &'a UVec2) {
	let topmost: &UVec2;
	let secmost: &UVec2;
	let trimost: &UVec2;

	if p0.y < p1.y && p0.y < p2.y {
		topmost = p0;
		if p1.y == p2.y {
			(trimost, secmost) = sort_by_x(p1, p2);
		} else {
			(trimost, secmost) = sort_by_y(p1, p2);
		}
	} else if p1.y < p0.y && p1.y < p2.y {
		topmost = p1;
		if p0.y == p2.y {
			(trimost, secmost) = sort_by_x(p0, p2);
		} else {
			(trimost, secmost) = sort_by_y(p0, p2);
		}
	} else {
		topmost = p2;
		if p0.y == p1.y {
			(trimost, secmost) = sort_by_x(p0, p1)
		} else {
			(trimost, secmost) = sort_by_y(p0, p1);
		}
	}

	(topmost, secmost, trimost)
}

fn sort_by_x<'a>(first: &'a UVec2, sec: &'a UVec2) -> (&'a UVec2, &'a UVec2) {
	if first.x > sec.x { (first, sec) } else { (sec, first) }
}

fn sort_by_y<'a>(first: &'a UVec2, sec: &'a UVec2) -> (&'a UVec2, &'a UVec2) {
	if first.y > sec.y { (first, sec) } else { (sec, first) }
}
