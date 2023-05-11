pub mod mesh;

use std::f32::consts::TAU;

use crate::{maths::{*, self}, benchmark::Benchmark};

use self::mesh::Mesh;


// ascii luminance:
// . , - ~ : ; = ! & # @"
pub static BACKGROUND_FILL_CHAR: char = ' ';

static LUMIN: &str = " .,-~:;=!&#@";
// static DIRS: &str = "
// ↖ ↑ ↗\
// ← · →\
// ↙ ↓ ↘";

static FILL_CHAR: char = '*';


#[derive(Debug)]
pub struct ScreenTriangle {
	pub p0: UVec2,
	pub p1: UVec2,
	pub p2: UVec2,
}



pub fn render_clear(buffer: &mut Vec<char>) {
	for i in 0..buffer.len() {
		buffer[i] = BACKGROUND_FILL_CHAR;
	}
}

pub fn draw_string(str: &str, pos: &UVec2, buffer: &mut Vec<char>, screen_width: u16) {
	let mut index = pos.y as usize * screen_width as usize + pos.x as usize;
	for ch in str.chars() {
		// TODO: bounds check
		if index > buffer.len() { continue; }
		buffer[index] = ch;
		index += 1;
	}
}

fn vec3_by_mat4x4(x: f32, y: f32, z: f32, mat: &Vec<f32>) -> (f32, f32, f32) {
	let mut x2 = x * mat[0*4+0] + y * mat[0*4+1] + z * mat[0*4+2] + mat[0*4+3];
	let mut y2 = x * mat[1*4+0] + y * mat[1*4+1] + z * mat[1*4+2] + mat[1*4+3];
	let mut z2 = x * mat[2*4+0] + y * mat[2*4+1] + z * mat[2*4+2] + mat[2*4+3];
	let w      = x * mat[3*4+0] + y * mat[3*4+1] + z * mat[3*4+2] + mat[3*4+3];

	if w != 0.0 {
		x2 /= w;
		y2 /= w;
		z2 /= w;
	}

	(x2, y2, z2)
}

pub fn draw_mesh(mesh: &Mesh, buffer: &mut Vec<char>, screen_width: u16, screen_height: u16) {

	// let a = screen_height as f32 / screen_width as f32;
	let a = screen_width as f32 / screen_height as f32;
	let fov = 0.25 * TAU;
	let zn =   0.1;
	let zf = 100.0;
	let q  = zf / (zf - zn);

	// let fv = 1.0 / (fov * 0.5).tan();

	let znq = -zn * q;

	let proj_mat = vec![
		1.0, 0.0, 0.0, 0.0,
		0.0, 1.0, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.0, 0.0, 0.0, 1.0,
	];

	let cam_pos = (-3.0, -3.0, -6.0);
	// let cam_pos = (0.0, 0.0, 0.0);

	let projection_factor = 1.0 / (fov * 0.5).tan();

	let num_tris = mesh.tris.len()/3;

	// for vert in mesh.verts {
	for tr_i in 0..num_tris {

		let i_tri_p0 = mesh.tris[tr_i * 3 + 0];
		let i_tri_p1 = mesh.tris[tr_i * 3 + 1];
		let i_tri_p2 = mesh.tris[tr_i * 3 + 2];

		// let p0x = (mesh.verts[(i_tri_p0 * 3 + 0) as usize] + 1.0) * 0.5;
		// let p0y = (mesh.verts[(i_tri_p0 * 3 + 1) as usize] + 1.0) * 0.5;
		// let p0z = (mesh.verts[(i_tri_p0 * 3 + 2) as usize] + 1.0) * 0.5;
		
		// let p1x = (mesh.verts[(i_tri_p1 * 3 + 0) as usize] + 1.0) * 0.5;
		// let p1y = (mesh.verts[(i_tri_p1 * 3 + 1) as usize] + 1.0) * 0.5;
		// let p1z = (mesh.verts[(i_tri_p1 * 3 + 2) as usize] + 1.0) * 0.5;

		// let p2x = (mesh.verts[(i_tri_p2 * 3 + 0) as usize] + 1.0) * 0.5;
		// let p2y = (mesh.verts[(i_tri_p2 * 3 + 1) as usize] + 1.0) * 0.5;
		// let p2z = (mesh.verts[(i_tri_p2 * 3 + 2) as usize] + 1.0) * 0.5;


		let p0x = mesh.verts[(i_tri_p0 * 3 + 0) as usize];
		let p0y = mesh.verts[(i_tri_p0 * 3 + 1) as usize];
		let p0z = mesh.verts[(i_tri_p0 * 3 + 2) as usize];
		
		let p1x = mesh.verts[(i_tri_p1 * 3 + 0) as usize];
		let p1y = mesh.verts[(i_tri_p1 * 3 + 1) as usize];
		let p1z = mesh.verts[(i_tri_p1 * 3 + 2) as usize];

		let p2x = mesh.verts[(i_tri_p2 * 3 + 0) as usize];
		let p2y = mesh.verts[(i_tri_p2 * 3 + 1) as usize];
		let p2z = mesh.verts[(i_tri_p2 * 3 + 2) as usize];
		

		// draw_string(&format!("{} [{:.2}, {:.2}, {:.2}]", tr_i, p0x, p0y, p0z), &UVec2::new(0, tr_i as u16), buffer, screen_width);
		// draw_string(&format!("{} [{:.2}, {:.2}, {:.2}]", tr_i, p1x, p1y, p1z), &UVec2::new(30, tr_i as u16), buffer, screen_width);
		// draw_string(&format!("{} [{:.2}, {:.2}, {:.2}]", tr_i, p2x, p2y, p2z), &UVec2::new(60, tr_i as u16), buffer, screen_width);


		let half_screen_width  = screen_width  as f32 * 0.5;
		let half_screen_height = screen_height as f32 * 0.5;

        let screen_p0_x = (p0x - cam_pos.0 * projection_factor) / (p0z - cam_pos.2 + zn) * half_screen_width  + half_screen_width;
        let screen_p0_y = (p0y - cam_pos.1 * projection_factor) / (p0z - cam_pos.2 + zn) * half_screen_height + half_screen_height;

		let screen_p1_x = (p1x - cam_pos.0 * projection_factor) / (p1z - cam_pos.2 + zn) * half_screen_width  + half_screen_width;
        let screen_p1_y = (p1y - cam_pos.1 * projection_factor) / (p1z - cam_pos.2 + zn) * half_screen_height + half_screen_height;

		let screen_p2_x = (p2x - cam_pos.0 * projection_factor) / (p2z - cam_pos.2 + zn) * half_screen_width  + half_screen_width;
        let screen_p2_y = (p2y - cam_pos.1 * projection_factor) / (p2z - cam_pos.2 + zn) * half_screen_height + half_screen_height;


		draw_besenham_line(
			&UVec2::new(screen_p0_x as u16, screen_p0_y as u16),
			&UVec2::new(screen_p1_x as u16, screen_p1_y as u16),
			buffer,
			screen_width,
			'*');
		
		draw_besenham_line(
			&UVec2::new(screen_p1_x as u16, screen_p1_y as u16),
			&UVec2::new(screen_p2_x as u16, screen_p2_y as u16),
			buffer,
			screen_width,
			'*');
		
		draw_besenham_line(
			&UVec2::new(screen_p2_x as u16, screen_p2_y as u16),
			&UVec2::new(screen_p0_x as u16, screen_p0_y as u16),
			buffer,
			screen_width,
			'*');



continue;



		// draw_string(&format!("{} [{:.2}, {:.2}, {:.2}]", tr_i, p0x, p0y, p0z), &UVec2::new(0, tr_i as u16), buffer, screen_width);
		// draw_string(&format!("{} [{:.2}, {:.2}, {:.2}]", tr_i, p1x, p1y, p1z), &UVec2::new(30, tr_i as u16), buffer, screen_width);
		// draw_string(&format!("{} [{:.2}, {:.2}, {:.2}]", tr_i, p2x, p2y, p2z), &UVec2::new(60, tr_i as u16), buffer, screen_width);
		
		let (mut p0x2, mut p0y2, mut p0z2) = vec3_by_mat4x4(p0x, p0y, p0z, &proj_mat);
		let (mut p1x2, mut p1y2, mut p1z2) = vec3_by_mat4x4(p0x, p0y, p0z, &proj_mat);
		let (mut p2x2, mut p2y2, mut p2z2) = vec3_by_mat4x4(p0x, p0y, p0z, &proj_mat);

		// clip	space to normalized space
		p0x2 = (p0x2 + 1.0 * 0.5);
		p0y2 = (p0y2 + 1.0 * 0.5);
		// p0z2 = (p0z2 + 1.0 * 0.5);
		p1x2 = (p1x2 + 1.0 * 0.5);
		p1y2 = (p1y2 + 1.0 * 0.5);
		// // p1z2 = (p1z2 + 1.0 * 0.5);
		p2x2 = (p2x2 + 1.0 * 0.5);
		p2y2 = (p2y2 + 1.0 * 0.5);
		// // p2z2 = (p2z2 + 1.0 * 0.5);

		// normalized space to screen space
		
		p0x2 = (p0x2 * 0.5 * screen_width  as f32);
		p0y2 = (p0y2 * 0.5 * screen_height as f32);
		p1x2 = (p1x2 * 0.5 * screen_width  as f32);
		p1y2 = (p1y2 * 0.5 * screen_height as f32);
		p2x2 = (p2x2 * 0.5 * screen_width  as f32);
		p2y2 = (p2y2 * 0.5 * screen_height as f32);

		// x2 *= (0.5 * screen_width as f32);
		// y2 *= (0.5 * screen_height as f32);
		// z2 *= screen_height as f32;

		// x2 += 10.0;

		// draw_string(&format!("[{}, {}, {}]", x, y, z), &UVec2::new(0, i as u16), buffer, screen_width);
		// draw_string(&format!("[{}, {}, {}]", x2, y2, z2), &UVec2::new(0, (i + mesh.verts.len()/3 + 2) as u16), buffer, screen_width);

		draw_point(&UVec2::new(p0x2 as u16, p0y2 as u16), buffer, screen_width, '@');
		draw_point(&UVec2::new(p1x2 as u16, p1y2 as u16), buffer, screen_width, '@');
		draw_point(&UVec2::new(p2x2 as u16, p2y2 as u16), buffer, screen_width, '@');
	}
}

pub fn draw_triangles_wire(screen_space_tris: &Vec<ScreenTriangle>, buffer: &mut Vec<char>, screen_width: u16) {
	let mut i: u16 = 0;
	for tri in screen_space_tris.iter() {
		// let top_to_sec_slope = slope_of_line(&tri.p0, secmost);
		// let top_to_tri_slope = slope_of_line(&tri.p0, trimost);
		// let sec_to_tri_slope = slope_of_line(secmost, trimost);

		draw_besenham_line(&tri.p0, &tri.p1, buffer, screen_width, FILL_CHAR);
		draw_besenham_line(&tri.p1, &tri.p2, buffer, screen_width, FILL_CHAR);
		draw_besenham_line(&tri.p2, &tri.p0, buffer, screen_width, FILL_CHAR);


		draw_string(&format!("p0 {:?}", &tri.p0), &UVec2 { x: &tri.p0.x + 3, y: &tri.p0.y - 2 + i }, buffer, screen_width);
		draw_string(&format!("p1 {:?}", &tri.p1), &UVec2 { x: &tri.p1.x - 3, y: &tri.p1.y + 2 + i }, buffer, screen_width);
		draw_string(&format!("p2 {:?}", &tri.p2), &UVec2 { x: &tri.p2.x + 3, y: &tri.p2.y - 1 + i }, buffer, screen_width);
		
		let (topmost, secmost, trimost) = sort_by_y_prefer_left(&tri.p0, &tri.p1, &tri.p2);

		// TODO: learn what this does
		let shortside_in_left = (secmost.y as f32 - topmost.y as f32) * (trimost.x as f32 - topmost.x as f32) > (secmost.x as f32 - topmost.x as f32) * (trimost.y as f32 - topmost.y as f32);
		if shortside_in_left {
			draw_string(&format!("the bend is on left"),  &UVec2 { x: 4, y: 4+i }, buffer, screen_width);
		} else {
			draw_string(&format!("the bend is on right"), &UVec2 { x: 4, y: 4+i }, buffer, screen_width);
		}

		// TODO: bounds check
		let index = (tri.p0.y * screen_width + tri.p0.x) as usize;
		buffer[index] = '@';
		
		let index = (tri.p1.y * screen_width + tri.p1.x) as usize;
		buffer[index] = '@';

		let index = (tri.p2.y * screen_width + tri.p2.x) as usize;
		buffer[index] = '@';

		i += 1;
	}
}

// TODO: decent test
pub fn test_besenham(buffer: &mut Vec<char>, screen_width: u16, screen_height: u16, time_spent: i32) {
	draw_string(&format!("w:{}, h:{}", screen_width, screen_height), &UVec2::new(0, 0), buffer, screen_width);
	
	let middle = UVec2::new(screen_width / 2, screen_height / 2);
	
	let len = 20.0;
	let modulus = time_spent / 2 % 1000;
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
		direction = '→';
	} else if angle < (TAU * 1.0/8.0) {
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

	draw_besenham_line(&middle, &up, buffer, screen_width, direction);

	draw_point(&up, buffer, screen_width, '@');

	draw_string(&format!("{}", angle),  &UVec2::new(0, 1), buffer, screen_width);
	draw_string(&format!("{}", up),     &UVec2::new(up.x+2, up.y), buffer, screen_width);

	// let right = &UVec2::new(middle.x + len, middle.y);
	// let left  = &UVec2::new(middle.x - len, middle.y);
	// let up    = &UVec2::new(middle.x, middle.y - len);
	// let down  = &UVec2::new(middle.x, middle.y + len/4);
	
	// let up_r  = &UVec2::new(right.x, up.y);
	// let up_l  = &UVec2::new(left.x, up.y);
	// ↖ ↑ ↗
	// ← · →
	// ↙ ↓ ↘

	// draw_besenham_line(&middle, right, buffer, screen_width, '→');
	// draw_besenham_line(&middle, left,  buffer, screen_width, '←');
	// draw_besenham_line(&middle, up,    buffer, screen_width, '↑');
	// draw_besenham_line(&middle, down,  buffer, screen_width, '↓');

	// draw_besenham_line(&middle, up_l,  buffer, screen_width, '↖');
	// draw_besenham_line(&middle, up_r,  buffer, screen_width, '↗');
	
	// draw_point(&middle, buffer, screen_width, '·');
}

fn lerp(a: u16, b: u16, t: f32) -> u16 {
	(a as f32 * (1.0 - t) + b as f32 * t) as u16
}


pub fn draw_benchmark(buffer: &mut Vec<char>, screen_width: u16, screen_height: u16, benchmark: &Benchmark) {
	draw_string(&format!("dt: {}ms", benchmark.delta_time),          &UVec2::new(0, screen_height-2-3), buffer, screen_width);
	draw_string(&format!("fps: {}", benchmark.fps),                  &UVec2::new(0, screen_height-2-2), buffer, screen_width);
	draw_string(&format!("frames: {}", benchmark.total_frame_count), &UVec2::new(0, screen_height-2-1), buffer, screen_width)
}



pub fn draw_point(p: &UVec2, buffer: &mut Vec<char>, screen_width: u16, fill_char: char) {
	// TODO: bounds check
	let index: usize = (p.y as usize * screen_width as usize + p.x as usize);
	if index < buffer.len() {
		buffer[index] = fill_char;
	}
}

fn draw_besenham_line(p0: &UVec2, p1: &UVec2, buffer: &mut Vec<char>, screen_width: u16, fill_char: char) {
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
    let mut dd_deriv = dx - dy;

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

        let dd_deriv2 = dd_deriv * 2;
        if dd_deriv2 > -dy {
            dd_deriv -= dy;
            x += sx;
        }
        if dd_deriv2 < dx {
            dd_deriv += dx;
            y += sy;
        }
    }

	// return;
	
	// let slope = slope_of_line(p0, p1);
	// let mut index: usize;

	// draw_string(&format!("slope {slope:?}"), UVec2 { x: 3, y: 5 }, buffer, screen_width);
	// // the slope goes more up/down than left/right
	// if slope.abs() > 1f32 {
	// 	todo!()
	// } else {

	// 	// if p0.y > p1.y {
	// 	// 	let temp = p0;
	// 	// 	p0 = p1;
	// 	// 	p1 = temp;
	// 	// }
	// 	let direction_inc: i16 = if slope > 0.0 { 1 } else { -1 };

	// 	draw_string(&format!("p0 {p0:?}"), UVec2 { x: 3, y: 6 }, buffer, screen_width);
	// 	draw_string(&format!("p1 {p1:?}"), UVec2 { x: 3, y: 7 }, buffer, screen_width);

	// 	let mut accum_slope = 0_f32;
	// 	let mut y = 0;
	// 	let mut accum_threshold = 0.5_f32;
		
	// 	let diff = p1.x.abs_diff(p0.x);

	// 	let slope_abs = slope.abs();

	// 	// for x in p0.x..p1.x {
	// 		// let x0 = x;
	// 	let x_start = u16::min(p0.x, p1.x);
	// 	for i in 0..diff {
	// 		let x0 = x_start + i;

	// 		accum_slope += slope_abs;
	// 		// let y0 = topmost.y + (accum_slope as u16);

	// 		draw_string(&format!("accum_slope {accum_slope:?}"), UVec2 { x: 3, y: 8+i+1 }, buffer, screen_width);
	// 		draw_string(&format!("accum_threshold {accum_threshold:?}"), UVec2 { x: 3, y: 8+i+diff+2 }, buffer, screen_width);
	// 		if accum_slope.abs() > accum_threshold {
	// 			accum_threshold += 1.0;
	// 			draw_string(&format!("inc {y:?} += {direction_inc:?}"), UVec2 { x: 3, y: 8+i+diff+diff+3 }, buffer, screen_width);
	// 			y += direction_inc;
	// 		}
			
	// 		let y0 = (p0.y as i16 + y) as u16;
			
	// 		index = (y0 * screen_width + x0) as usize;
			
	// 		// handle out of bounds
	// 		// buffer[index] = FILL_CHAR;
	// 		if index < buffer.len() {
	// 			// buffer[index] = FILL_CHAR;
	// 			buffer[index] = fill_char;
	// 		}
	// 	}
	// }
}

pub fn draw_triangles_filled(screen_space_tris: &mut Vec<ScreenTriangle>, buffer: &mut Vec<char>, screen_width: u16) {
	todo!()
}


fn slope_of_line(p0: &UVec2, p1: &UVec2) -> f32 {
	(p1.y as f32 - p0.y as f32) / (p1.x as f32 - p0.x as f32)
}

fn sort_by_y_prefer_left<'a>(p0: &'a UVec2, p1: &'a UVec2, p2: &'a UVec2) -> (&'a UVec2, &'a UVec2, &'a UVec2) {
	let topmost: &UVec2;
	let secmost: &UVec2;
	let trimost: &UVec2;

	if p0.y < p1.y && p0.y < p2.y {
		topmost = &p0;
		if p1.y == p2.y {
			(trimost, secmost) = sort_by_x(&p1, &p2);
		} else {
			(trimost, secmost) = sort_by_y(&p1, &p2);
		}
	} else if p1.y < p0.y && p1.y < p2.y {
		topmost = &p1;
		if p0.y == p2.y {
			(trimost, secmost) = sort_by_x(&p0, &p2);
		} else {
			(trimost, secmost) = sort_by_y(&p0, &p2);
		}
	} else {
		topmost = &p2;
		if p0.y == p1.y {
			(trimost, secmost) = sort_by_x(&p0, &p1)
		} else {
			(trimost, secmost) = sort_by_y(&p0, &p1);
		}
	}

	(topmost, secmost, trimost)
}

fn sort_by_x<'a>(first: &'a UVec2, sec: &'a UVec2) -> (&'a UVec2, &'a UVec2) {
	return if first.x > sec.x { (first, sec) } else { (sec, first) }
}

fn sort_by_y<'a>(first: &'a UVec2, sec: &'a UVec2) -> (&'a UVec2, &'a UVec2) {
	return if first.y > sec.y { (first, sec) } else { (sec, first) }
}
