use core::fmt;
use std::{f32::consts::TAU, fmt::Display};

use crate::utils::xy_to_it;


#[derive(Clone, Copy)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}

	pub fn zero() -> Self {
		Self { x: 0.0, y: 0.0, z: 0.0 }
	}

	pub fn side() -> Self {
		Self { x: 0.0, y: 0.0, z: 1.0 }
	}
	pub fn up() -> Self {
		Self { x: 0.0, y: 0.0, z: 1.0 }
	}
	pub fn forward() -> Self {
		Self { x: 0.0, y: 0.0, z: 1.0 }
	}

	pub fn invert_y(&self) -> Self {
		Self { x: self.x, y: -self.y, z: self.z }
	}

	pub fn get_transformed_by_mat3x3(&self, mat: &[f32]) -> Self {
		let x = self.x * mat[0*3 + 0] + self.y * mat[0*3 + 1] + self.z * mat[0*3 + 2];
		let y = self.x * mat[1*3 + 0] + self.y * mat[1*3 + 1] + self.z * mat[1*3 + 2];
		let z = self.x * mat[2*3 + 0] + self.y * mat[2*3 + 1] + self.z * mat[2*3 + 2];

		Self { x, y, z }
	}

	pub fn transform_by_mat3x3(&mut self, mat: &[f32]) {
		let x = self.x * mat[0*3 + 0] + self.y * mat[0*3 + 1] + self.z * mat[0*3 + 2];
		let y = self.x * mat[1*3 + 0] + self.y * mat[1*3 + 1] + self.z * mat[1*3 + 2];
		let z = self.x * mat[2*3 + 0] + self.y * mat[2*3 + 1] + self.z * mat[2*3 + 2];

		self.x = x;
		self.y = y;
		self.z = z;
	}

	pub fn get_transformed_by_mat4x4(&self, mat: &[f32]) -> Self {
		let mut x = self.x * mat[0*4 + 0] + self.y * mat[0*4 + 1] + self.z * mat[0*4 + 2] + 1.0 * mat[0*4 + 3];
		let mut y = self.x * mat[1*4 + 0] + self.y * mat[1*4 + 1] + self.z * mat[1*4 + 2] + 1.0 * mat[1*4 + 3];
		let mut z = self.x * mat[2*4 + 0] + self.y * mat[2*4 + 1] + self.z * mat[2*4 + 2] + 1.0 * mat[2*4 + 3];
		let w     = self.x * mat[3*4 + 0] + self.y * mat[3*4 + 1] + self.z * mat[3*4 + 2] + 1.0 * mat[3*4 + 3];

		if w != 0.0 {
			x /= w;
			y /= w;
			z /= w;
		}

		Self { x, y, z }
	}

	pub fn add_vec(&self, rhs: &Vec3) -> Self {
		Vec3 {
			x: rhs.x + self.x,
			y: rhs.y + self.y,
			z: rhs.z + self.z,
		}
	}

	pub fn mul_vec(&self, rhs: f32) -> Self {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}

	pub fn inversed(&self) -> Self {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

impl std::ops::Add<&Vec3> for &Vec3 {
	type Output = Vec3;

	fn add(self, rhs: &Vec3) -> Self::Output {
		self.add_vec(rhs)
	}
}

impl std::ops::Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Self::Output {
		self.add_vec(&rhs)
	}
}

impl std::ops::Mul<f32> for &Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f32) -> Self::Output {
		self.mul_vec(rhs)
	}
}

impl std::ops::Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f32) -> Self::Output {
		self.mul_vec(rhs)
	}
}

impl Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{:+.2}, {:+.2}, {:+.2}]", self.x, self.y, self.z)
	}
}


// TODO: generic? Vec<i16> and Vec<f32>

pub type Int = i32;

pub struct IVec2 {
	pub x: Int,
	pub y: Int,
}

impl IVec2 {
	pub fn new(x: Int, y: Int) -> Self {
		Self { x, y }
	}

	pub fn sum_t(&self, rhs: (Int, Int)) -> Self {
		Self { x: self.x as Int + rhs.0, y: self.y as Int + rhs.1 }
	}
}

impl Into<UVec2> for IVec2 {
	fn into(self) -> UVec2 {
		debug_assert!(self.x >= 0, "ivec {:} x is < 0", self);
		debug_assert!(self.y >= 0, "ivec {:} y is < 0", self);
		UVec2::new(self.x as u16, self.y as u16)
	}
}

impl fmt::Display for IVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}

// #[derive(Clone, Copy)]
pub struct UVec2 {
	pub x: u16,
	pub y: u16,
}

impl UVec2 {
	pub fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}

	pub fn sum(&self, x: u16, y: u16) -> Self {
		Self { x: self.x + x, y: self.y + y }
	}

	pub fn sum_t(&self, rhs: (i16, i16)) -> Self {
		Self { x: (self.x as i16 + rhs.0) as u16, y: (self.y as i16 + rhs.1) as u16 }
	}

	pub fn sum_v(&self, rhs: UVec2) -> Self {
		Self { x: self.x + rhs.x, y: self.y + rhs.y }
	}
}


impl fmt::Display for UVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}

impl fmt::Debug for UVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}


pub fn build_identity_4x4() -> Vec<f32> {
	vec![
		1.0, 0.0, 0.0, 0.0,
		0.0, 1.0, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.0, 0.0, 0.0, 1.0,
	]
}

pub fn copy_mat4x4(vec_src: &[f32], vec_dst: &mut [f32]) {
	vec_dst.copy_from_slice(vec_src);
}

pub fn clip_space_to_screen_space(p: &Vec3, screen_width: u16, screen_height: u16) -> IVec2 {
	let screen_x = (p.x + 1.0) * 0.5 * screen_width  as f32;
	let screen_y = (p.y + 1.0) * 0.5 * screen_height as f32;

	IVec2::new(screen_x as Int, screen_y as Int)
}


pub fn normalize(v: &Vec3) -> Vec3 {
	let length = (v.x*v.x + v.y*v.y + v.z*v.z).sqrt();
	Vec3::new(v.x / length, v.y / length, v.z / length)
}

pub fn cross_product(a: &Vec3, b: &Vec3) -> Vec3 {
	Vec3::new(
		a.y * b.z - a.z * b.y,
		a.z * b.x - a.x * b.z,
		a.x * b.y - a.y * b.x,
	)
}

pub fn dot_product(a: &Vec3, b: &Vec3) -> f32 {
	a.x*b.x + a.y*b.y + a.z*b.z
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

	// counterclockwise rotation
	let rot_mat = [
		             cos_y*cos_z          ,              cos_y*sin_z         ,    -sin_y   , 0.0,
		sin_x*sin_y*cos_z +  cos_x*-sin_z , sin_x*sin_y*sin_z +  cos_x*cos_z , sin_x*cos_y , 0.0,
		cos_x*sin_y*cos_z + -sin_x*-sin_z , cos_x*sin_y*sin_z + -sin_x*cos_z , cos_x*cos_y , 0.0,
		     0.0,           0.0           ,               0.0                ,               1.0,
	];

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
	const SZ: usize = 4;
	mat[0 * SZ + 3] = pos_x;
	mat[1 * SZ + 3] = pos_y;
	mat[2 * SZ + 3] = pos_z;
}

pub fn multiply_4x4_matrices(dump: &mut [f32], mat: &[f32]) {
	const SZ: usize = 4;

	let x0_y0 = dump[0 * SZ + 0] * mat[0 * SZ + 0]  +  dump[0 * SZ + 1] * mat[1 * SZ + 0]  +  dump[0 * SZ + 2] * mat[2 * SZ + 0]  +  dump[0 * SZ + 3] * mat[3 * SZ + 0];
	let x0_y1 = dump[0 * SZ + 0] * mat[0 * SZ + 1]  +  dump[0 * SZ + 1] * mat[1 * SZ + 1]  +  dump[0 * SZ + 2] * mat[2 * SZ + 1]  +  dump[0 * SZ + 3] * mat[3 * SZ + 1];
	let x0_y2 = dump[0 * SZ + 0] * mat[0 * SZ + 2]  +  dump[0 * SZ + 1] * mat[1 * SZ + 2]  +  dump[0 * SZ + 2] * mat[2 * SZ + 2]  +  dump[0 * SZ + 3] * mat[3 * SZ + 2];
	let x0_y3 = dump[0 * SZ + 0] * mat[0 * SZ + 3]  +  dump[0 * SZ + 1] * mat[1 * SZ + 3]  +  dump[0 * SZ + 2] * mat[2 * SZ + 3]  +  dump[0 * SZ + 3] * mat[3 * SZ + 3];

	let x1_y0 = dump[1 * SZ + 0] * mat[0 * SZ + 0]  +  dump[1 * SZ + 1] * mat[1 * SZ + 0]  +  dump[1 * SZ + 2] * mat[2 * SZ + 0]  +  dump[1 * SZ + 3] * mat[3 * SZ + 0];
	let x1_y1 = dump[1 * SZ + 0] * mat[0 * SZ + 1]  +  dump[1 * SZ + 1] * mat[1 * SZ + 1]  +  dump[1 * SZ + 2] * mat[2 * SZ + 1]  +  dump[1 * SZ + 3] * mat[3 * SZ + 1];
	let x1_y2 = dump[1 * SZ + 0] * mat[0 * SZ + 2]  +  dump[1 * SZ + 1] * mat[1 * SZ + 2]  +  dump[1 * SZ + 2] * mat[2 * SZ + 2]  +  dump[1 * SZ + 3] * mat[3 * SZ + 2];
	let x1_y3 = dump[1 * SZ + 0] * mat[0 * SZ + 3]  +  dump[1 * SZ + 1] * mat[1 * SZ + 3]  +  dump[1 * SZ + 2] * mat[2 * SZ + 3]  +  dump[1 * SZ + 3] * mat[3 * SZ + 3];

	let x2_y0 = dump[2 * SZ + 0] * mat[0 * SZ + 0]  +  dump[2 * SZ + 1] * mat[1 * SZ + 0]  +  dump[2 * SZ + 2] * mat[2 * SZ + 0]  +  dump[2 * SZ + 3] * mat[3 * SZ + 0];
	let x2_y1 = dump[2 * SZ + 0] * mat[0 * SZ + 1]  +  dump[2 * SZ + 1] * mat[1 * SZ + 1]  +  dump[2 * SZ + 2] * mat[2 * SZ + 1]  +  dump[2 * SZ + 3] * mat[3 * SZ + 1];
	let x2_y2 = dump[2 * SZ + 0] * mat[0 * SZ + 2]  +  dump[2 * SZ + 1] * mat[1 * SZ + 2]  +  dump[2 * SZ + 2] * mat[2 * SZ + 2]  +  dump[2 * SZ + 3] * mat[3 * SZ + 2];
	let x2_y3 = dump[2 * SZ + 0] * mat[0 * SZ + 3]  +  dump[2 * SZ + 1] * mat[1 * SZ + 3]  +  dump[2 * SZ + 2] * mat[2 * SZ + 3]  +  dump[2 * SZ + 3] * mat[3 * SZ + 3];

	let x3_y0 = dump[3 * SZ + 0] * mat[0 * SZ + 0]  +  dump[3 * SZ + 1] * mat[1 * SZ + 0]  +  dump[3 * SZ + 2] * mat[2 * SZ + 0]  +  dump[3 * SZ + 3] * mat[3 * SZ + 0];
	let x3_y1 = dump[3 * SZ + 0] * mat[0 * SZ + 1]  +  dump[3 * SZ + 1] * mat[1 * SZ + 1]  +  dump[3 * SZ + 2] * mat[2 * SZ + 1]  +  dump[3 * SZ + 3] * mat[3 * SZ + 1];
	let x3_y2 = dump[3 * SZ + 0] * mat[0 * SZ + 2]  +  dump[3 * SZ + 1] * mat[1 * SZ + 2]  +  dump[3 * SZ + 2] * mat[2 * SZ + 2]  +  dump[3 * SZ + 3] * mat[3 * SZ + 2];
	let x3_y3 = dump[3 * SZ + 0] * mat[0 * SZ + 3]  +  dump[3 * SZ + 1] * mat[1 * SZ + 3]  +  dump[3 * SZ + 2] * mat[2 * SZ + 3]  +  dump[3 * SZ + 3] * mat[3 * SZ + 3];

	dump[0 * SZ + 0] = x0_y0;
	dump[0 * SZ + 1] = x0_y1;
	dump[0 * SZ + 2] = x0_y2;
	dump[0 * SZ + 3] = x0_y3;

	dump[1 * SZ + 0] = x1_y0;
	dump[1 * SZ + 1] = x1_y1;
	dump[1 * SZ + 2] = x1_y2;
	dump[1 * SZ + 3] = x1_y3;

	dump[2 * SZ + 0] = x2_y0;
	dump[2 * SZ + 1] = x2_y1;
	dump[2 * SZ + 2] = x2_y2;
	dump[2 * SZ + 3] = x2_y3;

	dump[3 * SZ + 0] = x3_y0;
	dump[3 * SZ + 1] = x3_y1;
	dump[3 * SZ + 2] = x3_y2;
	dump[3 * SZ + 3] = x3_y3;
}

pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
	(1.0 - t) * a + b * t
}

pub fn triangle_wave(t: f32) -> f32 {
	1.0 - ((t % 1.0) - 0.5).abs() * 2.0
}

// when smoothness is 1, it's a line, more than that smoothes it
pub fn smoothed_0_to_1(t: f32, sharpness: f32) -> f32 {
	let pow_t = t.powf(sharpness);
	pow_t / ( pow_t + (1.0 - t).powf(sharpness) )
}
