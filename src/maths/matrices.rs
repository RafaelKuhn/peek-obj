use std::f32::consts::TAU;

use crate::maths::*;



pub fn create_identity_4x4() -> Vec<f32> {
	vec![
		1.0, 0.0, 0.0, 0.0,
		0.0, 1.0, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.0, 0.0, 0.0, 1.0,
	]
}

pub fn create_identity_4x4_arr() -> [f32; 16] {
	[
		1.0, 0.0, 0.0, 0.0,
		0.0, 1.0, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.0, 0.0, 0.0, 1.0,
	]
}

pub fn copy_mat4x4(vec_src: &[f32], vec_dst: &mut [f32]) {
	vec_dst.copy_from_slice(vec_src);
}

pub fn apply_identity_to_mat_4x4(mat: &mut [f32]) {
	const SZ: u16 = 4;

	mat[xy_to_it(0, 0, SZ)] = 1.0;
	mat[xy_to_it(1, 0, SZ)] = 0.0;
	mat[xy_to_it(2, 0, SZ)] = 0.0;
	mat[xy_to_it(3, 0, SZ)] = 0.0;

	mat[xy_to_it(0, 1, SZ)] = 0.0;
	mat[xy_to_it(1, 1, SZ)] = 1.0;
	mat[xy_to_it(2, 1, SZ)] = 0.0;
	mat[xy_to_it(3, 1, SZ)] = 0.0;

	mat[xy_to_it(0, 2, SZ)] = 0.0;
	mat[xy_to_it(1, 2, SZ)] = 0.0;
	mat[xy_to_it(2, 2, SZ)] = 1.0;
	mat[xy_to_it(3, 2, SZ)] = 0.0;

	mat[xy_to_it(0, 3, SZ)] = 0.0;
	mat[xy_to_it(1, 3, SZ)] = 0.0;
	mat[xy_to_it(2, 3, SZ)] = 0.0;
	mat[xy_to_it(3, 3, SZ)] = 1.0;
}

pub fn apply_projection_to_mat_4x4(mat: &mut [f32], width: u16, height: u16) {

	const ZN: f32 =   0.1;
	const ZF: f32 = 100.0;

	// height of the characters is double the width of the characters
	let aspect_ratio = (height as f32 * 2.0) / width as f32;
	const FOV: f32 = 0.25 * TAU; // 90 degrees

	let inv_tan_half_fov = 1.0 / ((FOV / 2.0).tan());
	let z_range = ZF - ZN;

	let fir = aspect_ratio * inv_tan_half_fov;
	let sec = inv_tan_half_fov;
	let thi = ZF / z_range;
	let fou = (-ZF * ZN) / z_range;

	// let mut proj_mat = vec![
	// 	fir, 0.0, 0.0, 0.0,
	// 	0.0, sec, 0.0, 0.0,
	// 	0.0, 0.0, thi, 1.0,
	// 	0.0, 0.0, fou, 0.0,

	// 	// transposed:
	// let mut proj_mat = vec![
	// 	fir, 0.0, 0.0, 0.0,
	// 	0.0, sec, 0.0, 0.0,
	// 	0.0, 0.0, thi, fou,
	// 	0.0, 0.0, 1.0, 0.0,
	// ];

	const SZ: u16 = 4;
	mat[xy_to_it(0, 0, SZ)] = fir;
	mat[xy_to_it(1, 1, SZ)] = sec;
	mat[xy_to_it(2, 2, SZ)] = thi;
	mat[xy_to_it(3, 3, SZ)] = 0.0;
	
	mat[xy_to_it(2, 3, SZ)] = fou;
	mat[xy_to_it(3, 2, SZ)] = 1.0;
}

pub fn apply_scale_to_mat_4x4(mat: &mut [f32], scale_x: f32, scale_y: f32, scale_z: f32) {
	const SZ: u16 = 4;

	mat[xy_to_it(0, 0, SZ)] = mat[xy_to_it(0, 0, SZ)] * scale_x;
	mat[xy_to_it(0, 1, SZ)] = mat[xy_to_it(0, 1, SZ)] * scale_x;
	mat[xy_to_it(0, 2, SZ)] = mat[xy_to_it(0, 2, SZ)] * scale_x;

	mat[xy_to_it(1, 0, SZ)] = mat[xy_to_it(1, 0, SZ)] * scale_y;
	mat[xy_to_it(1, 1, SZ)] = mat[xy_to_it(1, 1, SZ)] * scale_y;
	mat[xy_to_it(1, 2, SZ)] = mat[xy_to_it(1, 2, SZ)] * scale_y;

	mat[xy_to_it(2, 0, SZ)] = mat[xy_to_it(2, 0, SZ)] * scale_z;
	mat[xy_to_it(2, 1, SZ)] = mat[xy_to_it(2, 1, SZ)] * scale_z;
	mat[xy_to_it(2, 2, SZ)] = mat[xy_to_it(2, 2, SZ)] * scale_z;
}

pub fn apply_rotation_to_mat_4x4(mat: &mut [f32], angle_x: f32, angle_y: f32, angle_z: f32) {
	let cos_x = angle_x.cos();
	let sin_x = angle_x.sin();

	let cos_y = angle_y.cos();
	let sin_y = angle_y.sin();

	let cos_z = angle_z.cos();
	let sin_z = angle_z.sin();

	// clockwise rotation (right handed coordinate system)
	// let rot_mat = [
	// 	             cos_y*cos_z          ,              cos_y*sin_z         ,    -sin_y   , 0.0,
	// 	sin_x*sin_y*cos_z +  cos_x*-sin_z , sin_x*sin_y*sin_z +  cos_x*cos_z , sin_x*cos_y , 0.0,
	// 	cos_x*sin_y*cos_z + -sin_x*-sin_z , cos_x*sin_y*sin_z + -sin_x*cos_z , cos_x*cos_y , 0.0,
	// 	     0.0,           0.0           ,               0.0                ,               1.0,
	// ];

	// counterclockwise rotation
	let rot_mat = [
		             cos_y*cos_z           ,              cos_y*-sin_z           ,    sin_y     , 0.0,
		-sin_x*-sin_y*cos_z +  cos_x*sin_z , -sin_x*-sin_y*-sin_z +  cos_x*cos_z , -sin_x*cos_y , 0.0,
		cos_x*-sin_y*cos_z + sin_x*sin_z   , cos_x*-sin_y*-sin_z + sin_x*cos_z   , cos_x*cos_y  , 0.0,
		           0.0                     ,                0.0                  ,    0.0       , 1.0,
	];

	// this is actually fucking wrong because it multiplied Z by XY and not XY by Z
	// let rot_mat = [
	// 	  cos_y*cos_z + -sin_x*-sin_y*-sin_z   ,          cos_x*-sin_z       ,    sin_y*cos_z + -sin_y*cos_y*-sin_z   , 0.0,
	// 	   cos_y*sin_z + -sin_x*-sin_y*cos_z   ,           cos_x*cos_z       ,    sin_y*sin_z + -sin_x*cos_y*cos_z    , 0.0,
	// 	               cos_x-sin_y             ,              sin_x          ,            cos_x*cos_y                 , 0.0,
	// 	                 0.0                   ,              0.0            ,               0.0                      , 1.0,
	// ];


	multiply_4x4_matrices(mat, &rot_mat);

	// OR:

	// const SZ: u16 = 4;
	// let x0_y0_rot = cos_y * cos_z;
	// let x0_y1_rot = -cos_y * sin_z;
	// let x0_y2_rot = sin_y;

	// let x1_y0_rot = cos_x * sin_z + sin_x * sin_y * cos_z;
	// let x1_y1_rot = cos_x * cos_z - sin_x * sin_y * sin_z;
	// let x1_y2_rot = -sin_x * cos_y;

	// let x2_y0_rot = sin_x * sin_z - cos_x * sin_y * cos_z;
	// let x2_y1_rot = sin_x * cos_z + cos_x * sin_y * sin_z;
	// let x2_y2_rot = cos_x * cos_y;

	// let x0_y0 = mat[xy_to_it(0, 0, SZ)] * x0_y0_rot  +  mat[xy_to_it(1, 0, SZ)] * x0_y1_rot  +  mat[xy_to_it(2, 0, SZ)] * x0_y2_rot;
	// let x0_y1 = mat[xy_to_it(0, 0, SZ)] * x1_y0_rot  +  mat[xy_to_it(1, 0, SZ)] * x1_y1_rot  +  mat[xy_to_it(2, 0, SZ)] * x1_y2_rot;
	// let x0_y2 = mat[xy_to_it(0, 0, SZ)] * x2_y0_rot  +  mat[xy_to_it(1, 0, SZ)] * x2_y1_rot  +  mat[xy_to_it(2, 0, SZ)] * x2_y2_rot;

	// let x1_y0 = mat[xy_to_it(0, 1, SZ)] * x0_y0_rot  +  mat[xy_to_it(1, 1, SZ)] * x0_y1_rot  +  mat[xy_to_it(2, 1, SZ)] * x0_y2_rot;
	// let x1_y1 = mat[xy_to_it(0, 1, SZ)] * x1_y0_rot  +  mat[xy_to_it(1, 1, SZ)] * x1_y1_rot  +  mat[xy_to_it(2, 1, SZ)] * x1_y2_rot;
	// let x1_y2 = mat[xy_to_it(0, 1, SZ)] * x2_y0_rot  +  mat[xy_to_it(1, 1, SZ)] * x2_y1_rot  +  mat[xy_to_it(2, 1, SZ)] * x2_y2_rot;

	// let x2_y0 = mat[xy_to_it(0, 2, SZ)] * x0_y0_rot  +  mat[xy_to_it(1, 2, SZ)] * x0_y1_rot  +  mat[xy_to_it(2, 2, SZ)] * x0_y2_rot;
	// let x2_y1 = mat[xy_to_it(0, 2, SZ)] * x1_y0_rot  +  mat[xy_to_it(1, 2, SZ)] * x1_y1_rot  +  mat[xy_to_it(2, 2, SZ)] * x1_y2_rot;
	// let x2_y2 = mat[xy_to_it(0, 2, SZ)] * x2_y0_rot  +  mat[xy_to_it(1, 2, SZ)] * x2_y1_rot  +  mat[xy_to_it(2, 2, SZ)] * x2_y2_rot;

	// mat[xy_to_it(0, 0, SZ)] = x0_y0;
	// mat[xy_to_it(1, 0, SZ)] = x0_y1;
	// mat[xy_to_it(2, 0, SZ)] = x0_y2;

	// mat[xy_to_it(0, 1, SZ)] = x1_y0;
	// mat[xy_to_it(1, 1, SZ)] = x1_y1;
	// mat[xy_to_it(2, 1, SZ)] = x1_y2;

	// mat[xy_to_it(0, 2, SZ)] = x2_y0;
	// mat[xy_to_it(1, 2, SZ)] = x2_y1;
	// mat[xy_to_it(2, 2, SZ)] = x2_y2;
}

pub fn apply_pos_to_mat_4x4(mat: &mut [f32], pos_x: f32, pos_y: f32, pos_z: f32) {
	const SZ: u16 = 4;
	mat[xy_to_it(3, 0, SZ)] = pos_x;
	mat[xy_to_it(3, 1, SZ)] = pos_y;
	mat[xy_to_it(3, 2, SZ)] = pos_z;
}

pub fn multiply_4x4_matrices(dump: &mut [f32], mat: &[f32]) {
	const SZ: u16 = 4;

	let x0_y0 = dump[xy_to_it(0, 0, SZ)] * mat[xy_to_it(0, 0, SZ)]  +  dump[xy_to_it(1, 0, SZ)] * mat[xy_to_it(0, 1, SZ)]  +  dump[xy_to_it(2, 0, SZ)] * mat[xy_to_it(0, 2, SZ)]  +  dump[xy_to_it(3, 0, SZ)] * mat[xy_to_it(0, 3, SZ)];
	let x0_y1 = dump[xy_to_it(0, 0, SZ)] * mat[xy_to_it(1, 0, SZ)]  +  dump[xy_to_it(1, 0, SZ)] * mat[xy_to_it(1, 1, SZ)]  +  dump[xy_to_it(2, 0, SZ)] * mat[xy_to_it(1, 2, SZ)]  +  dump[xy_to_it(3, 0, SZ)] * mat[xy_to_it(1, 3, SZ)];
	let x0_y2 = dump[xy_to_it(0, 0, SZ)] * mat[xy_to_it(2, 0, SZ)]  +  dump[xy_to_it(1, 0, SZ)] * mat[xy_to_it(2, 1, SZ)]  +  dump[xy_to_it(2, 0, SZ)] * mat[xy_to_it(2, 2, SZ)]  +  dump[xy_to_it(3, 0, SZ)] * mat[xy_to_it(2, 3, SZ)];
	let x0_y3 = dump[xy_to_it(0, 0, SZ)] * mat[xy_to_it(3, 0, SZ)]  +  dump[xy_to_it(1, 0, SZ)] * mat[xy_to_it(3, 1, SZ)]  +  dump[xy_to_it(2, 0, SZ)] * mat[xy_to_it(3, 2, SZ)]  +  dump[xy_to_it(3, 0, SZ)] * mat[xy_to_it(3, 3, SZ)];

	let x1_y0 = dump[xy_to_it(0, 1, SZ)] * mat[xy_to_it(0, 0, SZ)]  +  dump[xy_to_it(1, 1, SZ)] * mat[xy_to_it(0, 1, SZ)]  +  dump[xy_to_it(2, 1, SZ)] * mat[xy_to_it(0, 2, SZ)]  +  dump[xy_to_it(3, 1, SZ)] * mat[xy_to_it(0, 3, SZ)];
	let x1_y1 = dump[xy_to_it(0, 1, SZ)] * mat[xy_to_it(1, 0, SZ)]  +  dump[xy_to_it(1, 1, SZ)] * mat[xy_to_it(1, 1, SZ)]  +  dump[xy_to_it(2, 1, SZ)] * mat[xy_to_it(1, 2, SZ)]  +  dump[xy_to_it(3, 1, SZ)] * mat[xy_to_it(1, 3, SZ)];
	let x1_y2 = dump[xy_to_it(0, 1, SZ)] * mat[xy_to_it(2, 0, SZ)]  +  dump[xy_to_it(1, 1, SZ)] * mat[xy_to_it(2, 1, SZ)]  +  dump[xy_to_it(2, 1, SZ)] * mat[xy_to_it(2, 2, SZ)]  +  dump[xy_to_it(3, 1, SZ)] * mat[xy_to_it(2, 3, SZ)];
	let x1_y3 = dump[xy_to_it(0, 1, SZ)] * mat[xy_to_it(3, 0, SZ)]  +  dump[xy_to_it(1, 1, SZ)] * mat[xy_to_it(3, 1, SZ)]  +  dump[xy_to_it(2, 1, SZ)] * mat[xy_to_it(3, 2, SZ)]  +  dump[xy_to_it(3, 1, SZ)] * mat[xy_to_it(3, 3, SZ)];

	let x2_y0 = dump[xy_to_it(0, 2, SZ)] * mat[xy_to_it(0, 0, SZ)]  +  dump[xy_to_it(1, 2, SZ)] * mat[xy_to_it(0, 1, SZ)]  +  dump[xy_to_it(2, 2, SZ)] * mat[xy_to_it(0, 2, SZ)]  +  dump[xy_to_it(3, 2, SZ)] * mat[xy_to_it(0, 3, SZ)];
	let x2_y1 = dump[xy_to_it(0, 2, SZ)] * mat[xy_to_it(1, 0, SZ)]  +  dump[xy_to_it(1, 2, SZ)] * mat[xy_to_it(1, 1, SZ)]  +  dump[xy_to_it(2, 2, SZ)] * mat[xy_to_it(1, 2, SZ)]  +  dump[xy_to_it(3, 2, SZ)] * mat[xy_to_it(1, 3, SZ)];
	let x2_y2 = dump[xy_to_it(0, 2, SZ)] * mat[xy_to_it(2, 0, SZ)]  +  dump[xy_to_it(1, 2, SZ)] * mat[xy_to_it(2, 1, SZ)]  +  dump[xy_to_it(2, 2, SZ)] * mat[xy_to_it(2, 2, SZ)]  +  dump[xy_to_it(3, 2, SZ)] * mat[xy_to_it(2, 3, SZ)];
	let x2_y3 = dump[xy_to_it(0, 2, SZ)] * mat[xy_to_it(3, 0, SZ)]  +  dump[xy_to_it(1, 2, SZ)] * mat[xy_to_it(3, 1, SZ)]  +  dump[xy_to_it(2, 2, SZ)] * mat[xy_to_it(3, 2, SZ)]  +  dump[xy_to_it(3, 2, SZ)] * mat[xy_to_it(3, 3, SZ)];

	let x3_y0 = dump[xy_to_it(0, 3, SZ)] * mat[xy_to_it(0, 0, SZ)]  +  dump[xy_to_it(1, 3, SZ)] * mat[xy_to_it(0, 1, SZ)]  +  dump[xy_to_it(2, 3, SZ)] * mat[xy_to_it(0, 2, SZ)]  +  dump[xy_to_it(3, 3, SZ)] * mat[xy_to_it(0, 3, SZ)];
	let x3_y1 = dump[xy_to_it(0, 3, SZ)] * mat[xy_to_it(1, 0, SZ)]  +  dump[xy_to_it(1, 3, SZ)] * mat[xy_to_it(1, 1, SZ)]  +  dump[xy_to_it(2, 3, SZ)] * mat[xy_to_it(1, 2, SZ)]  +  dump[xy_to_it(3, 3, SZ)] * mat[xy_to_it(1, 3, SZ)];
	let x3_y2 = dump[xy_to_it(0, 3, SZ)] * mat[xy_to_it(2, 0, SZ)]  +  dump[xy_to_it(1, 3, SZ)] * mat[xy_to_it(2, 1, SZ)]  +  dump[xy_to_it(2, 3, SZ)] * mat[xy_to_it(2, 2, SZ)]  +  dump[xy_to_it(3, 3, SZ)] * mat[xy_to_it(2, 3, SZ)];
	let x3_y3 = dump[xy_to_it(0, 3, SZ)] * mat[xy_to_it(3, 0, SZ)]  +  dump[xy_to_it(1, 3, SZ)] * mat[xy_to_it(3, 1, SZ)]  +  dump[xy_to_it(2, 3, SZ)] * mat[xy_to_it(3, 2, SZ)]  +  dump[xy_to_it(3, 3, SZ)] * mat[xy_to_it(3, 3, SZ)];

	dump[xy_to_it(0, 0, SZ)] = x0_y0;
	dump[xy_to_it(1, 0, SZ)] = x0_y1;
	dump[xy_to_it(2, 0, SZ)] = x0_y2;
	dump[xy_to_it(3, 0, SZ)] = x0_y3;

	dump[xy_to_it(0, 1, SZ)] = x1_y0;
	dump[xy_to_it(1, 1, SZ)] = x1_y1;
	dump[xy_to_it(2, 1, SZ)] = x1_y2;
	dump[xy_to_it(3, 1, SZ)] = x1_y3;

	dump[xy_to_it(0, 2, SZ)] = x2_y0;
	dump[xy_to_it(1, 2, SZ)] = x2_y1;
	dump[xy_to_it(2, 2, SZ)] = x2_y2;
	dump[xy_to_it(3, 2, SZ)] = x2_y3;

	dump[xy_to_it(0, 3, SZ)] = x3_y0;
	dump[xy_to_it(1, 3, SZ)] = x3_y1;
	dump[xy_to_it(2, 3, SZ)] = x3_y2;
	dump[xy_to_it(3, 3, SZ)] = x3_y3;
}