use core::{fmt,};
use std::{fmt::Display, f32::consts::TAU};



// TODO: macro the shit out of this
// or use type system, require a type to be provided by each implementation of Vec3

pub struct UVec3 {
	pub x: u16,
	pub y: u16,
	pub z: u16,
}

impl UVec3 {
	pub fn new(x: u16, y: u16, z: u16,) -> Self {
		UVec3 { x, y, z }
	}
}


pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32,) -> Self {
		Vec3 { x, y, z }
	}

	pub fn transform_by_mat3x3(&mut self, mat: &Vec<f32>) {
		let x = self.x * mat[0*3 + 0] + self.y * mat[0*3 + 1] + self.z * mat[0*3 + 2];
		let y = self.x * mat[1*3 + 0] + self.y * mat[1*3 + 1] + self.z * mat[1*3 + 2];
		let z = self.x * mat[2*3 + 0] + self.y * mat[2*3 + 1] + self.z * mat[2*3 + 2];

		self.x = x;
		self.y = y;
		self.z = z;
	}

	pub fn get_translated_z(&self, v: f32) -> Self {
		Self { x: self.x, y: self.y, z: self.z + v }
	}

	pub fn get_transformed_by_mat3x3(&self, mat: &Vec<f32>) -> Self {
		let x = self.x * mat[0*3 + 0] + self.y * mat[0*3 + 1] + self.z * mat[0*3 + 2];
		let y = self.x * mat[1*3 + 0] + self.y * mat[1*3 + 1] + self.z * mat[1*3 + 2];
		let z = self.x * mat[2*3 + 0] + self.y * mat[2*3 + 1] + self.z * mat[2*3 + 2];

		Self { x, y, z }
	}

	pub fn get_transformed_by_mat4x4(&self, mat: &Vec<f32>) -> Self {
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

	pub fn transform_by_mat4x4(&mut self, mat: &Vec<f32>) {
		let mut x = self.x * mat[0*4 + 0] + self.y * mat[0*4 + 1] + self.z * mat[0*4 + 2] + 1.0 * mat[0*4 + 3];
		let mut y = self.x * mat[1*4 + 0] + self.y * mat[1*4 + 1] + self.z * mat[1*4 + 2] + 1.0 * mat[1*4 + 3];
		let mut z = self.x * mat[2*4 + 0] + self.y * mat[2*4 + 1] + self.z * mat[2*4 + 2] + 1.0 * mat[2*4 + 3];
		let w     = self.x * mat[3*4 + 0] + self.y * mat[3*4 + 1] + self.z * mat[3*4 + 2] + 1.0 * mat[3*4 + 3];

		if w != 0.0 {
			x /= w;
			y /= w;
			z /= w;
		}

		self.x = x;
		self.y = y;
		self.z = z;
	}
}


#[derive(Debug)]
pub struct IVec2 {
	pub x: i16,
	pub y: i16,
}


pub struct UVec2 {
	pub x: u16,
	pub y: u16,
}

impl Display for UVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
    }
}


impl UVec2 {
	pub fn new(x: u16, y: u16) -> Self {
		Self { x, y }
	}
}

// TODO: remove, check if I need to implement display or debug
impl fmt::Debug for UVec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl std::ops::Sub<&UVec2> for &UVec2 {
	type Output = IVec2;
	fn sub(self, rhs: &UVec2) -> Self::Output {
		return IVec2 {
			x: rhs.x as i16 - self.x as i16,
			y: rhs.y as i16 - self.y as i16,
		}
	}
}

impl std::ops::Sub<UVec2> for UVec2 {
	type Output = IVec2;
	fn sub(self, rhs: UVec2) -> Self::Output {
		return IVec2 {
			x: rhs.x as i16 - self.x as i16,
			y: rhs.y as i16 - self.y as i16,
		}
	}
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

fn transpose_3x3(vec: &mut Vec<f32>) {
	let mut temp: f32;
	
	let x = 1; let y = 0;
	temp = vec[y * 3 + x];
	vec[y * 3 + x] = vec[x * 3 + 0];
	vec[y * 3 + x] = temp;

	let x = 2; let y = 0;
	temp = vec[y * 3 + x];
	vec[y * 3 + x] = vec[x * 3 + 0];
	vec[y * 3 + x] = temp;

	let x = 2; let y = 1;
	temp = vec[y * 3 + x];
	vec[y * 3 + x] = vec[x * 3 + 0];
	vec[y * 3 + x] = temp;
}

fn transpose_3x3_area_in_4x4(vec: &mut Vec<f32>) {
	let mut temp: f32;
	
	let sz = 4;

	let x = 1; let y = 0;
	temp = vec[y * sz + x];
	vec[y * sz + x] = vec[x * sz + 0];
	vec[y * sz + x] = temp;

	let x = 2; let y = 0;
	temp = vec[y * sz + x];
	vec[y * sz + x] = vec[x * sz + 0];
	vec[y * sz + x] = temp;

	let x = 2; let y = 1;
	temp = vec[y * sz + x];
	vec[y * sz + x] = vec[x * sz + 0];
	vec[y * sz + x] = temp;
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

pub fn build_scale_mat_3x3(sc_x: f32, sc_y: f32, sc_z: f32) -> Vec<f32> {
	vec![
		sc_x,  0.0,  0.0,
		 0.0, sc_y,  0.0,
		 0.0,  0.0, sc_z,
	]
}

pub fn lerp(a: u32, b: u32, t: f32) -> u32 {
	(a as f32 * t + (b - a) as f32 * t) as u32
}

pub fn apply_identity_to_mat_4x4(mat: &mut Vec<f32>) {
	let sz = 4;
	
	mat[0 * sz + 0] = 1.0;
	mat[0 * sz + 1] = 0.0;
	mat[0 * sz + 2] = 0.0;
	mat[0 * sz + 3] = 0.0;

	mat[1 * sz + 0] = 0.0;
	mat[1 * sz + 1] = 1.0;
	mat[1 * sz + 2] = 0.0;
	mat[1 * sz + 3] = 0.0;

	mat[2 * sz + 0] = 0.0;
	mat[2 * sz + 1] = 0.0;
	mat[2 * sz + 2] = 1.0;
	mat[2 * sz + 3] = 0.0;

	mat[3 * sz + 0] = 0.0;
	mat[3 * sz + 1] = 0.0;
	mat[3 * sz + 2] = 0.0;
	mat[3 * sz + 3] = 1.0;
}

pub fn apply_projection_to_mat_4x4(mat: &mut Vec<f32>, width_height: (u16, u16)) {
	let (screen_width, screen_height) = width_height;

	let zn =   0.1;
	let zf = 100.0;
	
	let aspect_ratio = (screen_height as f32 * 2.0) / screen_width as f32;
    let fov = 0.25 * TAU;

    let inv_tan_half_fov = 1.0 / ((fov / 2.0).tan());
	let z_range = zf - zn;

	let fir = aspect_ratio * inv_tan_half_fov;
	let sec = inv_tan_half_fov;
	let thi = zf / (z_range);
	let fou = (-zf *zn) / (z_range);

	// let mut proj_mat = vec![
	// 	fir, 0.0, 0.0, 0.0,
	// 	0.0, sec, 0.0, 0.0,
	// 	0.0, 0.0, thi, 1.0,
	// 	0.0, 0.0, fou, 0.0,

	// 	// mirrored:
	// let mut proj_mat = vec![
	// 	fir, 0.0, 0.0, 0.0,
	// 	0.0, sec, 0.0, 0.0,
	// 	0.0, 0.0, thi, fou,
	// 	0.0, 0.0, 1.0, 0.0,
	// ];

	let sz = 4;
	mat[0 * sz + 0] = fir;
	mat[1 * sz + 1] = sec;
	mat[2 * sz + 2] = thi;
	mat[3 * sz + 3] = 0.0;
	
	mat[3 * sz + 2] = fou;
	mat[2 * sz + 3] = 1.0;
}

pub fn apply_scale_to_mat_4x4(mat: &mut Vec<f32>, scale_x: f32, scale_y: f32, scale_z: f32) {
	let sz = 4;

	//              y * w + x
	let x0_y0 = mat[0 * sz + 0] * scale_x; // [0, 0] * sc x
	let x1_y1 = mat[1 * sz + 1] * scale_y; // [1, 1] * sc y
	let x2_y2 = mat[2 * sz + 2] * scale_z; // [2, 2] * sc z
	
	mat[0 * sz + 0] = x0_y0; // x0_y0
	mat[0 * sz + 1] = 0.0;   // x0_y1
	mat[0 * sz + 2] = 0.0;   // x0_y2
	mat[0 * sz + 3] = 0.0;   // x0_y3

	mat[1 * sz + 0] = 0.0;   // x1_y0
	mat[1 * sz + 1] = x1_y1; // x1_y1
	mat[1 * sz + 2] = 0.0;   // x1_y2
	mat[1 * sz + 3] = 0.0;   // x1_y3

	mat[2 * sz + 0] = 0.0;   // x2_y0
	mat[2 * sz + 1] = 0.0;   // x2_y1
	mat[2 * sz + 2] = x2_y2; // x2_y2
	mat[2 * sz + 3] = 0.0;   // x2_y3
	
	mat[3 * sz + 0] = 0.0;   // x3_y0
	mat[3 * sz + 1] = 0.0;   // x3_y1
	mat[3 * sz + 2] = 0.0;   // x3_y2
}

pub fn apply_rotation_to_mat_4x4(mat: &mut Vec<f32>, angle_x: f32, angle_y: f32, angle_z: f32) {
	// easier: (allocates memory)

	// let mut mat2 = build_rot_mat_xyz_4x4(angle_x, angle_y, angle_z);
	// multiply_4x4_matrices(mat, &mat2);
	// return;
	
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


	let sz = 4;

	let x0_y0 = mat[0 * sz + 0] * x0_y0_rot  +  mat[0 * sz + 1] * x0_y1_rot  +  mat[0 * sz + 2] * x0_y2_rot;
	let x0_y1 = mat[0 * sz + 0] * x1_y0_rot  +  mat[0 * sz + 1] * x1_y1_rot  +  mat[0 * sz + 2] * x1_y2_rot;
	let x0_y2 = mat[0 * sz + 0] * x2_y0_rot  +  mat[0 * sz + 1] * x2_y1_rot  +  mat[0 * sz + 2] * x2_y2_rot;

	let x1_y0 = mat[1 * sz + 0] * x0_y0_rot  +  mat[1 * sz + 1] * x0_y1_rot  +  mat[1 * sz + 2] * x0_y2_rot;
	let x1_y1 = mat[1 * sz + 0] * x1_y0_rot  +  mat[1 * sz + 1] * x1_y1_rot  +  mat[1 * sz + 2] * x1_y2_rot;
	let x1_y2 = mat[1 * sz + 0] * x2_y0_rot  +  mat[1 * sz + 1] * x2_y1_rot  +  mat[1 * sz + 2] * x2_y2_rot;

	let x2_y0 = mat[2 * sz + 0] * x0_y0_rot  +  mat[2 * sz + 1] * x0_y1_rot  +  mat[2 * sz + 2] * x0_y2_rot;
	let x2_y1 = mat[2 * sz + 0] * x1_y0_rot  +  mat[2 * sz + 1] * x1_y1_rot  +  mat[2 * sz + 2] * x1_y2_rot;
	let x2_y2 = mat[2 * sz + 0] * x2_y0_rot  +  mat[2 * sz + 1] * x2_y1_rot  +  mat[2 * sz + 2] * x2_y2_rot;

	mat[0 * sz + 0] = x0_y0;
	mat[0 * sz + 1] = x0_y1;
	mat[0 * sz + 2] = x0_y2;

	mat[1 * sz + 0] = x1_y0;
	mat[1 * sz + 1] = x1_y1;
	mat[1 * sz + 2] = x1_y2;

	mat[2 * sz + 0] = x2_y0;
	mat[2 * sz + 1] = x2_y1;
	mat[2 * sz + 2] = x2_y2;
}

pub fn apply_pos_to_mat_4x4(mat: &mut Vec<f32>, pos_x: f32, pos_y: f32, pos_z: f32) {
	let sz = 4;
	mat[0 * sz + 3] = pos_x;
	mat[1 * sz + 3] = pos_y;
	mat[2 * sz + 3] = pos_z;
}


pub fn multiply_4x4_matrices(mat: &mut Vec<f32>, mat2: &Vec<f32>) {
	let sz = 4;

	let x0_y0 = mat[0 * sz + 0] * mat2[0 * sz + 0]  +  mat[0 * sz + 1] * mat2[1 * sz + 0]  +  mat[0 * sz + 2] * mat2[2 * sz + 0]  +  mat[0 * sz + 3] * mat2[3 * sz + 0];
	let x0_y1 = mat[0 * sz + 0] * mat2[0 * sz + 1]  +  mat[0 * sz + 1] * mat2[1 * sz + 1]  +  mat[0 * sz + 2] * mat2[2 * sz + 1]  +  mat[0 * sz + 3] * mat2[3 * sz + 1];
	let x0_y2 = mat[0 * sz + 0] * mat2[0 * sz + 2]  +  mat[0 * sz + 1] * mat2[1 * sz + 2]  +  mat[0 * sz + 2] * mat2[2 * sz + 2]  +  mat[0 * sz + 3] * mat2[3 * sz + 2];
	let x0_y3 = mat[0 * sz + 0] * mat2[0 * sz + 3]  +  mat[0 * sz + 1] * mat2[1 * sz + 3]  +  mat[0 * sz + 2] * mat2[2 * sz + 3]  +  mat[0 * sz + 3] * mat2[3 * sz + 3];

	let x1_y0 = mat[1 * sz + 0] * mat2[0 * sz + 0]  +  mat[1 * sz + 1] * mat2[1 * sz + 0]  +  mat[1 * sz + 2] * mat2[2 * sz + 0]  +  mat[1 * sz + 3] * mat2[3 * sz + 0];
	let x1_y1 = mat[1 * sz + 0] * mat2[0 * sz + 1]  +  mat[1 * sz + 1] * mat2[1 * sz + 1]  +  mat[1 * sz + 2] * mat2[2 * sz + 1]  +  mat[1 * sz + 3] * mat2[3 * sz + 1];
	let x1_y2 = mat[1 * sz + 0] * mat2[0 * sz + 2]  +  mat[1 * sz + 1] * mat2[1 * sz + 2]  +  mat[1 * sz + 2] * mat2[2 * sz + 2]  +  mat[1 * sz + 3] * mat2[3 * sz + 2];
	let x1_y3 = mat[1 * sz + 0] * mat2[0 * sz + 3]  +  mat[1 * sz + 1] * mat2[1 * sz + 3]  +  mat[1 * sz + 2] * mat2[2 * sz + 3]  +  mat[1 * sz + 3] * mat2[3 * sz + 3];

	let x2_y0 = mat[2 * sz + 0] * mat2[0 * sz + 0]  +  mat[2 * sz + 1] * mat2[1 * sz + 0]  +  mat[2 * sz + 2] * mat2[2 * sz + 0]  +  mat[2 * sz + 3] * mat2[3 * sz + 0];
	let x2_y1 = mat[2 * sz + 0] * mat2[0 * sz + 1]  +  mat[2 * sz + 1] * mat2[1 * sz + 1]  +  mat[2 * sz + 2] * mat2[2 * sz + 1]  +  mat[2 * sz + 3] * mat2[3 * sz + 1];
	let x2_y2 = mat[2 * sz + 0] * mat2[0 * sz + 2]  +  mat[2 * sz + 1] * mat2[1 * sz + 2]  +  mat[2 * sz + 2] * mat2[2 * sz + 2]  +  mat[2 * sz + 3] * mat2[3 * sz + 2];
	let x2_y3 = mat[2 * sz + 0] * mat2[0 * sz + 3]  +  mat[2 * sz + 1] * mat2[1 * sz + 3]  +  mat[2 * sz + 2] * mat2[2 * sz + 3]  +  mat[2 * sz + 3] * mat2[3 * sz + 3];

	let x3_y0 = mat[3 * sz + 0] * mat2[0 * sz + 0]  +  mat[3 * sz + 1] * mat2[1 * sz + 0]  +  mat[3 * sz + 2] * mat2[2 * sz + 0]  +  mat[3 * sz + 3] * mat2[3 * sz + 0];
	let x3_y1 = mat[3 * sz + 0] * mat2[0 * sz + 1]  +  mat[3 * sz + 1] * mat2[1 * sz + 1]  +  mat[3 * sz + 2] * mat2[2 * sz + 1]  +  mat[3 * sz + 3] * mat2[3 * sz + 1];
	let x3_y2 = mat[3 * sz + 0] * mat2[0 * sz + 2]  +  mat[3 * sz + 1] * mat2[1 * sz + 2]  +  mat[3 * sz + 2] * mat2[2 * sz + 2]  +  mat[3 * sz + 3] * mat2[3 * sz + 2];
	let x3_y3 = mat[3 * sz + 0] * mat2[0 * sz + 3]  +  mat[3 * sz + 1] * mat2[1 * sz + 3]  +  mat[3 * sz + 2] * mat2[2 * sz + 3]  +  mat[3 * sz + 3] * mat2[3 * sz + 3];

	mat[0 * sz + 0] = x0_y0;
	mat[0 * sz + 1] = x0_y1;
	mat[0 * sz + 2] = x0_y2;
	mat[0 * sz + 3] = x0_y3;

	mat[1 * sz + 0] = x1_y0;
	mat[1 * sz + 1] = x1_y1;
	mat[1 * sz + 2] = x1_y2;
	mat[1 * sz + 3] = x1_y3;

	mat[2 * sz + 0] = x2_y0;
	mat[2 * sz + 1] = x2_y1;
	mat[2 * sz + 2] = x2_y2;
	mat[2 * sz + 3] = x2_y3;

	mat[3 * sz + 0] = x3_y0;
	mat[3 * sz + 1] = x3_y1;
	mat[3 * sz + 2] = x3_y2;
	mat[3 * sz + 3] = x3_y3;

}

pub fn multiply_4x4_matrices_alloc(mat: &mut Vec<f32>, mat2: &Vec<f32>) {
	let mut result = [0.0; 16]; // Create a new matrix to store the result

	let sz = 4;

	for i in 0..sz {
		for j in 0..sz {
			for k in 0..sz {
				result[i * sz + j] += mat[i * sz + k] * mat2[k * sz + j];
			}
		}
	}
	
	// Copy the elements from the result matrix back to mat
	for i in 0..sz*sz {
		mat[i] = result[i];
	}
}

pub fn apply_mat_generic(mat: &mut Vec<f32>) {
	
	// build any mat_4x4 here to test
	let mat2 = Vec::<f32>::new();
	
	let sz = 4;

	let x0_y0 = mat[0 * sz + 0] * mat2[0 * sz + 0]  +  mat[0 * sz + 1] * mat2[1 * sz + 0]  +  mat[0 * sz + 2] * mat2[2 * sz + 0]  +  mat[0 * sz + 3] * mat2[3 * sz + 0];
	let x0_y1 = mat[0 * sz + 0] * mat2[0 * sz + 1]  +  mat[0 * sz + 1] * mat2[1 * sz + 1]  +  mat[0 * sz + 2] * mat2[2 * sz + 1]  +  mat[0 * sz + 3] * mat2[3 * sz + 1];
	let x0_y2 = mat[0 * sz + 0] * mat2[0 * sz + 2]  +  mat[0 * sz + 1] * mat2[1 * sz + 2]  +  mat[0 * sz + 2] * mat2[2 * sz + 2]  +  mat[0 * sz + 3] * mat2[3 * sz + 2];
	let x0_y3 = mat[0 * sz + 0] * mat2[0 * sz + 3]  +  mat[0 * sz + 1] * mat2[1 * sz + 3]  +  mat[0 * sz + 2] * mat2[2 * sz + 3]  +  mat[0 * sz + 3] * mat2[3 * sz + 3];

	let x1_y0 = mat[1 * sz + 0] * mat2[0 * sz + 0]  +  mat[1 * sz + 1] * mat2[1 * sz + 0]  +  mat[1 * sz + 2] * mat2[2 * sz + 0]  +  mat[1 * sz + 3] * mat2[3 * sz + 0];
	let x1_y1 = mat[1 * sz + 0] * mat2[0 * sz + 1]  +  mat[1 * sz + 1] * mat2[1 * sz + 1]  +  mat[1 * sz + 2] * mat2[2 * sz + 1]  +  mat[1 * sz + 3] * mat2[3 * sz + 1];
	let x1_y2 = mat[1 * sz + 0] * mat2[0 * sz + 2]  +  mat[1 * sz + 1] * mat2[1 * sz + 2]  +  mat[1 * sz + 2] * mat2[2 * sz + 2]  +  mat[1 * sz + 3] * mat2[3 * sz + 2];
	let x1_y3 = mat[1 * sz + 0] * mat2[0 * sz + 3]  +  mat[1 * sz + 1] * mat2[1 * sz + 3]  +  mat[1 * sz + 2] * mat2[2 * sz + 3]  +  mat[1 * sz + 3] * mat2[3 * sz + 3];

	let x2_y0 = mat[2 * sz + 0] * mat2[0 * sz + 0]  +  mat[2 * sz + 1] * mat2[1 * sz + 0]  +  mat[2 * sz + 2] * mat2[2 * sz + 0]  +  mat[2 * sz + 3] * mat2[3 * sz + 0];
	let x2_y1 = mat[2 * sz + 0] * mat2[0 * sz + 1]  +  mat[2 * sz + 1] * mat2[1 * sz + 1]  +  mat[2 * sz + 2] * mat2[2 * sz + 1]  +  mat[2 * sz + 3] * mat2[3 * sz + 1];
	let x2_y2 = mat[2 * sz + 0] * mat2[0 * sz + 2]  +  mat[2 * sz + 1] * mat2[1 * sz + 2]  +  mat[2 * sz + 2] * mat2[2 * sz + 2]  +  mat[2 * sz + 3] * mat2[3 * sz + 2];
	let x2_y3 = mat[2 * sz + 0] * mat2[0 * sz + 3]  +  mat[2 * sz + 1] * mat2[1 * sz + 3]  +  mat[2 * sz + 2] * mat2[2 * sz + 3]  +  mat[2 * sz + 3] * mat2[3 * sz + 3];

	let x3_y0 = mat[3 * sz + 0] * mat2[0 * sz + 0]  +  mat[3 * sz + 1] * mat2[1 * sz + 0]  +  mat[3 * sz + 2] * mat2[2 * sz + 0]  +  mat[3 * sz + 3] * mat2[3 * sz + 0];
	let x3_y1 = mat[3 * sz + 0] * mat2[0 * sz + 1]  +  mat[3 * sz + 1] * mat2[1 * sz + 1]  +  mat[3 * sz + 2] * mat2[2 * sz + 1]  +  mat[3 * sz + 3] * mat2[3 * sz + 1];
	let x3_y2 = mat[3 * sz + 0] * mat2[0 * sz + 2]  +  mat[3 * sz + 1] * mat2[1 * sz + 2]  +  mat[3 * sz + 2] * mat2[2 * sz + 2]  +  mat[3 * sz + 3] * mat2[3 * sz + 2];
	let x3_y3 = mat[3 * sz + 0] * mat2[0 * sz + 3]  +  mat[3 * sz + 1] * mat2[1 * sz + 3]  +  mat[3 * sz + 2] * mat2[2 * sz + 3]  +  mat[3 * sz + 3] * mat2[3 * sz + 3];

	mat[0 * sz + 0] = x0_y0;
	mat[0 * sz + 1] = x0_y1;
	mat[0 * sz + 2] = x0_y2;
	mat[0 * sz + 3] = x0_y3;

	mat[1 * sz + 0] = x1_y0;
	mat[1 * sz + 1] = x1_y1;
	mat[1 * sz + 2] = x1_y2;
	mat[1 * sz + 3] = x1_y3;

	mat[2 * sz + 0] = x2_y0;
	mat[2 * sz + 1] = x2_y1;
	mat[2 * sz + 2] = x2_y2;
	mat[2 * sz + 3] = x2_y3;

	mat[3 * sz + 0] = x3_y0;
	mat[3 * sz + 1] = x3_y1;
	mat[3 * sz + 2] = x3_y2;
	mat[3 * sz + 3] = x3_y3;

	mat[0 * sz + 0] = x0_y0;
	mat[0 * sz + 1] = x0_y1;
	mat[0 * sz + 2] = x0_y2;
	mat[0 * sz + 3] = x0_y3;

	mat[1 * sz + 0] = x1_y0;
	mat[1 * sz + 1] = x1_y1;
	mat[1 * sz + 2] = x1_y2;
	mat[1 * sz + 3] = x1_y3;

	mat[2 * sz + 0] = x2_y0;
	mat[2 * sz + 1] = x2_y1;
	mat[2 * sz + 2] = x2_y2;
	mat[2 * sz + 3] = x2_y3;

	mat[3 * sz + 0] = x3_y0;
	mat[3 * sz + 1] = x3_y1;
	mat[3 * sz + 2] = x3_y2;
	mat[3 * sz + 3] = x3_y3;
}
