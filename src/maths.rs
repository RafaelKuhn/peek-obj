use core::fmt;
use std::{fmt::Display, f32::consts::TAU};


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


pub struct UVec2 {
	pub x: u16,
	pub y: u16,
}

impl UVec2 {
	pub fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}
}

impl Display for UVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}

// TODO: check if I need to implement display or debug
impl fmt::Debug for UVec2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}


pub fn build_rot_mat_x_3x3(angle: f32) -> Vec<f32> {
	let cos = angle.cos();
	let sin = angle.sin();

	vec![
		1.0,  0.0,  0.0,
		0.0,  cos, -sin,
		0.0,  sin,  cos,
	]
}

pub fn build_rot_mat_y_3x3(angle: f32) -> Vec<f32> {
	let cos = angle.cos();
	let sin = angle.sin();

	vec![
		 cos,  0.0,  sin,
		 0.0,  1.0,  0.0,
		-sin,  0.0,  cos,
	]
}

pub fn build_rot_mat_z_3x3(angle: f32) -> Vec<f32> {
	let cos = angle.cos();
	let sin = angle.sin();

	vec![
		cos, -sin,  0.0,
		sin,  cos,  0.0,
		0.0,  0.0,  1.0,
	]
}

pub fn build_rot_mat_xyz_3x3(angle_x: f32, angle_y: f32, angle_z: f32) -> Vec<f32> {
	let cos_x = angle_x.cos();
	let sin_x = angle_x.sin();

	let cos_y = angle_y.cos();
	let sin_y = angle_y.sin();

	let cos_z = angle_z.cos();
	let sin_z = angle_z.sin();

	let sin_y_cos_z = sin_y * cos_z;
	
	vec![
		cos_y * cos_z,                       -cos_y * sin_z,                          sin_y,
		cos_x * sin_z + sin_x * sin_y_cos_z,  cos_x * cos_z - sin_x * sin_y * sin_z, -sin_x * cos_y,
		sin_x * sin_z - cos_x * sin_y_cos_z,  sin_x * cos_z + cos_x * sin_y * sin_z,  cos_x * cos_y,
	]
}

pub fn build_scale_mat_3x3(sc_x: f32, sc_y: f32, sc_z: f32) -> Vec<f32> {
	vec![
		sc_x,  0.0,  0.0,
		 0.0, sc_y,  0.0,
		 0.0,  0.0, sc_z,
	]
}

pub fn build_pos_mat4x4(pos_x: f32, pos_y: f32, pos_z: f32) -> Vec<f32> {
	vec![
		1.0, 0.0, 0.0, pos_x,
		0.0, 1.0, 0.0, pos_y,
		0.0, 0.0, 1.0, pos_z,
		0.0, 0.0, 0.0, 1.0,
		
		// transposed
		// 1.0, 0.0, 0.0, 0.0,
		// 0.0, 1.0, 0.0, 0.0,
		// 0.0, 0.0, 1.0, 0.0,
		// pos_x, pos_y, pos_z, 1.0,
	]
}

pub fn build_rot_mat_xyz_4x4(angle_x: f32, angle_y: f32, angle_z: f32) -> Vec<f32> {
	let cos_x = angle_x.cos();
	let sin_x = angle_x.sin();

	let cos_y = angle_y.cos();
	let sin_y = angle_y.sin();

	let cos_z = angle_z.cos();
	let sin_z = angle_z.sin();

	vec![
		cos_y * cos_z,
		-cos_y * sin_z,
		sin_y,
		0.0,

		cos_x * sin_z + sin_x * sin_y * cos_z,
		cos_x * cos_z - sin_x * sin_y * sin_z,
		-sin_x * cos_y,
		0.0,

		sin_x * sin_z - cos_x * sin_y * cos_z,
		sin_x * cos_z + cos_x * sin_y * sin_z,
		cos_x * cos_y,
		0.0,

		0.0,
		0.0,
		0.0,
		1.0
	]

	// or
	// vec![
	// 	cos_y * cos_z,                      -cos_y * sin_z,                          sin_y,         0.0,
	// 	cos_x * sin_z + sin_x * sin_y_cos_z,  cos_x * cos_z - sin_x * sin_y * sin_z, -sin_x * cos_y, 0.0,
	// 	sin_x * sin_z - cos_x * sin_y_cos_z,  sin_x * cos_z + cos_x * sin_y * sin_z,  cos_x * cos_y, 0.0,
	// 	0.0,                                  0.0,                                    0.0,           1.0
	// ]
}

pub fn build_identity_4x4() -> Vec<f32> {
	vec![
		1.0, 0.0, 0.0, 0.0,
		0.0, 1.0, 0.0, 0.0,
		0.0, 0.0, 1.0, 0.0,
		0.0, 0.0, 0.0, 1.0,
	]
}

pub fn clip_space_to_screen_space(p: &Vec3, screen_width: u16, screen_height: u16) -> UVec2 {
	let screen_x = (p.x + 1.0) * 0.5 * screen_width  as f32;
	let screen_y = (p.y + 1.0) * 0.5 * screen_height as f32;

	UVec2::new(screen_x as u16, screen_y as u16)
}

fn transpose_3x3(vec: &mut [f32]) {
	let mut temp: f32;
	
	const SZ: usize = 3;

	let x = 1; let y = 0;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;

	let x = 2; let y = 0;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;

	let x = 2; let y = 1;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;
}

fn transpose_3x3_area_in_4x4(vec: &mut [f32]) {
	let mut temp: f32;
	
	const SZ: usize = 4;

	let x = 1; let y = 0;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;

	let x = 2; let y = 0;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;

	let x = 2; let y = 1;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;
}

pub fn apply_identity_to_mat_4x4(mat: &mut [f32]) {
	const SZ: usize = 4;
	
	mat[0 * SZ + 0] = 1.0;
	mat[0 * SZ + 1] = 0.0;
	mat[0 * SZ + 2] = 0.0;
	mat[0 * SZ + 3] = 0.0;

	mat[1 * SZ + 0] = 0.0;
	mat[1 * SZ + 1] = 1.0;
	mat[1 * SZ + 2] = 0.0;
	mat[1 * SZ + 3] = 0.0;

	mat[2 * SZ + 0] = 0.0;
	mat[2 * SZ + 1] = 0.0;
	mat[2 * SZ + 2] = 1.0;
	mat[2 * SZ + 3] = 0.0;

	mat[3 * SZ + 0] = 0.0;
	mat[3 * SZ + 1] = 0.0;
	mat[3 * SZ + 2] = 0.0;
	mat[3 * SZ + 3] = 1.0;
}

pub fn apply_projection_to_mat_4x4(mat: &mut [f32], width: u16, height: u16) {

	const ZN: f32 =   0.1;
	const ZF: f32 = 100.0;

	// height of the characters is double the width of the characters
	let aspect_ratio = (height as f32 * 2.0) / width as f32;
	const FOV: f32 = 0.25 * TAU;

	let inv_tan_half_fov = 1.0 / ((FOV / 2.0).tan());
	let z_range = ZF - ZN;

	let fir = aspect_ratio * inv_tan_half_fov;
	let sec = inv_tan_half_fov;
	let thi = ZF / (z_range);
	let fou = (-ZF *ZN) / (z_range);

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

	const SZ: usize = 4;
	mat[0 * SZ + 0] = fir;
	mat[1 * SZ + 1] = sec;
	mat[2 * SZ + 2] = thi;
	mat[3 * SZ + 3] = 0.0;
	
	mat[3 * SZ + 2] = fou;
	mat[2 * SZ + 3] = 1.0;
}

pub fn apply_scale_to_mat_4x4(mat: &mut [f32], scale_x: f32, scale_y: f32, scale_z: f32) {
	const SZ: usize = 4;

	//              y * w + x
	let x0_y0 = mat[0 * SZ + 0] * scale_x;
	let x1_y1 = mat[1 * SZ + 1] * scale_y;
	let x2_y2 = mat[2 * SZ + 2] * scale_z;
	
	mat[0 * SZ + 0] = x0_y0;
	mat[0 * SZ + 1] = 0.0;
	mat[0 * SZ + 2] = 0.0;
	mat[0 * SZ + 3] = 0.0;

	mat[1 * SZ + 0] = 0.0;
	mat[1 * SZ + 1] = x1_y1;
	mat[1 * SZ + 2] = 0.0;
	mat[1 * SZ + 3] = 0.0;

	mat[2 * SZ + 0] = 0.0;
	mat[2 * SZ + 1] = 0.0;
	mat[2 * SZ + 2] = x2_y2;
	mat[2 * SZ + 3] = 0.0;
	
	mat[3 * SZ + 0] = 0.0;
	mat[3 * SZ + 1] = 0.0;
	mat[3 * SZ + 2] = 0.0;
}

pub fn apply_rotation_to_mat_4x4_alloc(mat: &mut [f32], angle_x: f32, angle_y: f32, angle_z: f32) {
	let mat2 = build_rot_mat_xyz_4x4(angle_x, angle_y, angle_z);
	multiply_4x4_matrices(mat, &mat2);
}

pub fn apply_rotation_to_mat_4x4(mat: &mut [f32], angle_x: f32, angle_y: f32, angle_z: f32) {	
	let cos_x = angle_x.cos();
	let sin_x = angle_x.sin();

	let cos_y = angle_y.cos();
	let sin_y = angle_y.sin();

	let cos_z = angle_z.cos();
	let sin_z = angle_z.sin();

	let x0_y0_rot = cos_y * cos_z;
	let x0_y1_rot = -cos_y * sin_z;
	let x0_y2_rot = sin_y;

	let x1_y0_rot = cos_x * sin_z + sin_x * sin_y * cos_z;
	let x1_y1_rot = cos_x * cos_z - sin_x * sin_y * sin_z;
	let x1_y2_rot = -sin_x * cos_y;

	let x2_y0_rot = sin_x * sin_z - cos_x * sin_y * cos_z;
	let x2_y1_rot = sin_x * cos_z + cos_x * sin_y * sin_z;
	let x2_y2_rot = cos_x * cos_y;

	const SZ: usize = 4;

	let x0_y0 = mat[0 * SZ + 0] * x0_y0_rot  +  mat[0 * SZ + 1] * x0_y1_rot  +  mat[0 * SZ + 2] * x0_y2_rot;
	let x0_y1 = mat[0 * SZ + 0] * x1_y0_rot  +  mat[0 * SZ + 1] * x1_y1_rot  +  mat[0 * SZ + 2] * x1_y2_rot;
	let x0_y2 = mat[0 * SZ + 0] * x2_y0_rot  +  mat[0 * SZ + 1] * x2_y1_rot  +  mat[0 * SZ + 2] * x2_y2_rot;

	let x1_y0 = mat[1 * SZ + 0] * x0_y0_rot  +  mat[1 * SZ + 1] * x0_y1_rot  +  mat[1 * SZ + 2] * x0_y2_rot;
	let x1_y1 = mat[1 * SZ + 0] * x1_y0_rot  +  mat[1 * SZ + 1] * x1_y1_rot  +  mat[1 * SZ + 2] * x1_y2_rot;
	let x1_y2 = mat[1 * SZ + 0] * x2_y0_rot  +  mat[1 * SZ + 1] * x2_y1_rot  +  mat[1 * SZ + 2] * x2_y2_rot;

	let x2_y0 = mat[2 * SZ + 0] * x0_y0_rot  +  mat[2 * SZ + 1] * x0_y1_rot  +  mat[2 * SZ + 2] * x0_y2_rot;
	let x2_y1 = mat[2 * SZ + 0] * x1_y0_rot  +  mat[2 * SZ + 1] * x1_y1_rot  +  mat[2 * SZ + 2] * x1_y2_rot;
	let x2_y2 = mat[2 * SZ + 0] * x2_y0_rot  +  mat[2 * SZ + 1] * x2_y1_rot  +  mat[2 * SZ + 2] * x2_y2_rot;

	mat[0 * SZ + 0] = x0_y0;
	mat[0 * SZ + 1] = x0_y1;
	mat[0 * SZ + 2] = x0_y2;

	mat[1 * SZ + 0] = x1_y0;
	mat[1 * SZ + 1] = x1_y1;
	mat[1 * SZ + 2] = x1_y2;

	mat[2 * SZ + 0] = x2_y0;
	mat[2 * SZ + 1] = x2_y1;
	mat[2 * SZ + 2] = x2_y2;
}

pub fn apply_pos_vec_to_mat_4x4(mat: &mut [f32], vec: &Vec3) {
	const SZ: usize = 4;
	mat[0 * SZ + 3] = vec.x;
	mat[1 * SZ + 3] = vec.y;
	mat[2 * SZ + 3] = vec.z;
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

pub fn multiply_4x4_matrices_alloc(mat: &mut [f32], mat2: &[f32]) {
	let mut result = [0.0; 16];

	const SZ: usize = 4;

	for i in 0..SZ {
		for j in 0..SZ {
			for k in 0..SZ {
				result[i * SZ + j] += mat[i * SZ + k] * mat2[k * SZ + j];
			}
		}
	}

	const SZ_SQ: usize = SZ * SZ;
	mat[..SZ_SQ].copy_from_slice(&result[..SZ_SQ]);

	// same as
	// for i in 0..sz*sz {
	// 	mat[i] = result[i];
	// }
}

pub fn apply_mat_generic(mat: &mut [f32]) {
	
	// build any mat_4x4 here to test
	let mat2 = Vec::<f32>::new();
	
	const SZ: usize = 4;

	let x0_y0 = mat[0 * SZ + 0] * mat2[0 * SZ + 0]  +  mat[0 * SZ + 1] * mat2[1 * SZ + 0]  +  mat[0 * SZ + 2] * mat2[2 * SZ + 0]  +  mat[0 * SZ + 3] * mat2[3 * SZ + 0];
	let x0_y1 = mat[0 * SZ + 0] * mat2[0 * SZ + 1]  +  mat[0 * SZ + 1] * mat2[1 * SZ + 1]  +  mat[0 * SZ + 2] * mat2[2 * SZ + 1]  +  mat[0 * SZ + 3] * mat2[3 * SZ + 1];
	let x0_y2 = mat[0 * SZ + 0] * mat2[0 * SZ + 2]  +  mat[0 * SZ + 1] * mat2[1 * SZ + 2]  +  mat[0 * SZ + 2] * mat2[2 * SZ + 2]  +  mat[0 * SZ + 3] * mat2[3 * SZ + 2];
	let x0_y3 = mat[0 * SZ + 0] * mat2[0 * SZ + 3]  +  mat[0 * SZ + 1] * mat2[1 * SZ + 3]  +  mat[0 * SZ + 2] * mat2[2 * SZ + 3]  +  mat[0 * SZ + 3] * mat2[3 * SZ + 3];

	let x1_y0 = mat[1 * SZ + 0] * mat2[0 * SZ + 0]  +  mat[1 * SZ + 1] * mat2[1 * SZ + 0]  +  mat[1 * SZ + 2] * mat2[2 * SZ + 0]  +  mat[1 * SZ + 3] * mat2[3 * SZ + 0];
	let x1_y1 = mat[1 * SZ + 0] * mat2[0 * SZ + 1]  +  mat[1 * SZ + 1] * mat2[1 * SZ + 1]  +  mat[1 * SZ + 2] * mat2[2 * SZ + 1]  +  mat[1 * SZ + 3] * mat2[3 * SZ + 1];
	let x1_y2 = mat[1 * SZ + 0] * mat2[0 * SZ + 2]  +  mat[1 * SZ + 1] * mat2[1 * SZ + 2]  +  mat[1 * SZ + 2] * mat2[2 * SZ + 2]  +  mat[1 * SZ + 3] * mat2[3 * SZ + 2];
	let x1_y3 = mat[1 * SZ + 0] * mat2[0 * SZ + 3]  +  mat[1 * SZ + 1] * mat2[1 * SZ + 3]  +  mat[1 * SZ + 2] * mat2[2 * SZ + 3]  +  mat[1 * SZ + 3] * mat2[3 * SZ + 3];

	let x2_y0 = mat[2 * SZ + 0] * mat2[0 * SZ + 0]  +  mat[2 * SZ + 1] * mat2[1 * SZ + 0]  +  mat[2 * SZ + 2] * mat2[2 * SZ + 0]  +  mat[2 * SZ + 3] * mat2[3 * SZ + 0];
	let x2_y1 = mat[2 * SZ + 0] * mat2[0 * SZ + 1]  +  mat[2 * SZ + 1] * mat2[1 * SZ + 1]  +  mat[2 * SZ + 2] * mat2[2 * SZ + 1]  +  mat[2 * SZ + 3] * mat2[3 * SZ + 1];
	let x2_y2 = mat[2 * SZ + 0] * mat2[0 * SZ + 2]  +  mat[2 * SZ + 1] * mat2[1 * SZ + 2]  +  mat[2 * SZ + 2] * mat2[2 * SZ + 2]  +  mat[2 * SZ + 3] * mat2[3 * SZ + 2];
	let x2_y3 = mat[2 * SZ + 0] * mat2[0 * SZ + 3]  +  mat[2 * SZ + 1] * mat2[1 * SZ + 3]  +  mat[2 * SZ + 2] * mat2[2 * SZ + 3]  +  mat[2 * SZ + 3] * mat2[3 * SZ + 3];

	let x3_y0 = mat[3 * SZ + 0] * mat2[0 * SZ + 0]  +  mat[3 * SZ + 1] * mat2[1 * SZ + 0]  +  mat[3 * SZ + 2] * mat2[2 * SZ + 0]  +  mat[3 * SZ + 3] * mat2[3 * SZ + 0];
	let x3_y1 = mat[3 * SZ + 0] * mat2[0 * SZ + 1]  +  mat[3 * SZ + 1] * mat2[1 * SZ + 1]  +  mat[3 * SZ + 2] * mat2[2 * SZ + 1]  +  mat[3 * SZ + 3] * mat2[3 * SZ + 1];
	let x3_y2 = mat[3 * SZ + 0] * mat2[0 * SZ + 2]  +  mat[3 * SZ + 1] * mat2[1 * SZ + 2]  +  mat[3 * SZ + 2] * mat2[2 * SZ + 2]  +  mat[3 * SZ + 3] * mat2[3 * SZ + 2];
	let x3_y3 = mat[3 * SZ + 0] * mat2[0 * SZ + 3]  +  mat[3 * SZ + 1] * mat2[1 * SZ + 3]  +  mat[3 * SZ + 2] * mat2[2 * SZ + 3]  +  mat[3 * SZ + 3] * mat2[3 * SZ + 3];

	mat[0 * SZ + 0] = x0_y0;
	mat[0 * SZ + 1] = x0_y1;
	mat[0 * SZ + 2] = x0_y2;
	mat[0 * SZ + 3] = x0_y3;

	mat[1 * SZ + 0] = x1_y0;
	mat[1 * SZ + 1] = x1_y1;
	mat[1 * SZ + 2] = x1_y2;
	mat[1 * SZ + 3] = x1_y3;

	mat[2 * SZ + 0] = x2_y0;
	mat[2 * SZ + 1] = x2_y1;
	mat[2 * SZ + 2] = x2_y2;
	mat[2 * SZ + 3] = x2_y3;

	mat[3 * SZ + 0] = x3_y0;
	mat[3 * SZ + 1] = x3_y1;
	mat[3 * SZ + 2] = x3_y2;
	mat[3 * SZ + 3] = x3_y3;

	mat[0 * SZ + 0] = x0_y0;
	mat[0 * SZ + 1] = x0_y1;
	mat[0 * SZ + 2] = x0_y2;
	mat[0 * SZ + 3] = x0_y3;

	mat[1 * SZ + 0] = x1_y0;
	mat[1 * SZ + 1] = x1_y1;
	mat[1 * SZ + 2] = x1_y2;
	mat[1 * SZ + 3] = x1_y3;

	mat[2 * SZ + 0] = x2_y0;
	mat[2 * SZ + 1] = x2_y1;
	mat[2 * SZ + 2] = x2_y2;
	mat[2 * SZ + 3] = x2_y3;

	mat[3 * SZ + 0] = x3_y0;
	mat[3 * SZ + 1] = x3_y1;
	mat[3 * SZ + 2] = x3_y2;
	mat[3 * SZ + 3] = x3_y3;
}