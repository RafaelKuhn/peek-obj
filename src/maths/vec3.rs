use std::{f32::consts::TAU, fmt};

use crate::{clip_space_to_screen_space, ivec2::IVec2, maths::*};


#[derive(Clone, Copy)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
		Self { x, y, z }
	}

	pub fn zero() -> Vec3 {
		Self { x: 0.0, y: 0.0, z: 0.0 }
	}

	pub fn side() -> Vec3 {
		Self { x: 0.0, y: 0.0, z: 1.0 }
	}
	pub fn up() -> Vec3 {
		Self { x: 0.0, y: 0.0, z: 1.0 }
	}
	pub fn forward() -> Vec3 {
		Self { x: 0.0, y: 0.0, z: 1.0 }
	}

	pub fn xy(&self) -> (f32, f32) {
		(self.x, self.y)
	}

	pub fn yz(&self) -> (f32, f32) {
		(self.y, self.z)
	}

	pub fn xz(&self) -> (f32, f32) {
		(self.x, self.z)
	}
		
	pub fn squared_magnitude(&self) -> f32 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn squared_dist_to(&self, other: &Vec3) -> f32 {
		let vec = Self::new(other.x - self.x, other.y - self.y, other.z - self.z);
		vec.squared_magnitude()
	}

	pub fn magnitude(&self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	pub fn dist_to(&self, other: &Vec3) -> f32 {
		let vec = Self::new(other.x - self.x, other.y - self.y, other.z - self.z);
		vec.magnitude()
	}

	pub fn normalized(&self) -> Vec3 {
		let magnitude = self.magnitude();
		Self { x: self.x / magnitude, y: self.y / magnitude, z: self.z / magnitude }
	}

	#[deprecated(since = "0.0", note = "This does not seem to work for some reason")]
	pub fn get_rotation(&self) -> Vec3 {

		// I don't know why but it look like it works less crappy like this
		let rotation_x = -f32::atan2(self.y, self.z);
		let rotation_y = f32::atan2(self.x, self.z);
		let rotation_z = f32::atan2(self.y, self.x);

		Vec3::new(rotation_x, rotation_y, rotation_z)
	}

	pub fn get_rotation_wo_orientation(&self) -> Vec3 {
		let magnitude = self.magnitude();

		let rotation_x = ( Vec3::dot_product( &self, &Vec3::new(1., 0., 0.) ) / magnitude ).acos();
		let rotation_y = ( Vec3::dot_product( &self, &Vec3::new(0., 1., 0.) ) / magnitude ).acos();
		let rotation_z = ( Vec3::dot_product( &self, &Vec3::new(0., 0., 1.) ) / magnitude ).acos();

		Vec3::new(rotation_x, rotation_y, rotation_z)
	}

	pub fn rad_to_deg(&self) -> Vec3 {
		Vec3::new(rad_to_deg(self.x), rad_to_deg(self.y), rad_to_deg(self.z))
	}

	pub fn rad_to_deg_pretty(&self) -> Vec3 {
		Vec3::new(rad_to_deg(self.x) % 360.0, rad_to_deg(self.y) % 360.0, rad_to_deg(self.z) % 360.0)
	}

	// heavy, use only for debugging
	pub fn rotated_x(&self, x_rot: f32) -> Vec3 {
		let sin_x = x_rot.sin();
		let cos_x = x_rot.cos();

		Vec3::new(
			self.x,
			cos_x * self.y + -sin_x * self.z,
			sin_x * self.y +  cos_x * self.z,
		)
	}

	// heavy, use only for debugging
	pub fn rotated_y(&self, y_rot: f32) -> Vec3 {
		let sin_y = y_rot.sin();
		let cos_y = y_rot.cos();

		Vec3::new(
			 cos_y * self.x + sin_y * self.z,
			 self.y,
			-sin_y * self.x + cos_y * self.z,
		)
	}

	// heavy, use only for debugging
	pub fn rotated_xy(&self, x_rot: f32, y_rot: f32) -> Vec3 {
		let sin_x = x_rot.sin();
		let cos_x = x_rot.cos();

		let sin_y = y_rot.sin();
		let cos_y = y_rot.cos();

		Vec3::new(
				   cos_y *self.x + sin_y*self.z                      ,
			-sin_x*-sin_y*self.x + cos_x*self.y + -sin_x*cos_y*self.z,
			 cos_x*-sin_y*self.x + sin_x*self.y +  cos_x*cos_y*self.z,
		)
	}

	// heavy, use only for debugging
	pub fn rotated_z(&self, z_rot: f32) -> Vec3 {
		let sin_z = z_rot.sin();
		let cos_z = z_rot.cos();

		Vec3::new(
			cos_z * self.x + -sin_z * self.y,
			sin_z * self.x +  cos_z * self.y,
			self.z,
		)
	}

	// very heavy to call repeatedly, use only for debugging purposes!
	pub fn get_rotated_xyz(&self, x_rot: f32, y_rot: f32, z_rot: f32) -> Vec3 {
		let sin_x = x_rot.sin();
		let cos_x = x_rot.cos();
		
		let sin_y = y_rot.sin();
		let cos_y = y_rot.cos();

		let sin_z = z_rot.sin();
		let cos_z = z_rot.cos();

		Vec3::new(
			( cos_y * cos_z) * self.x                          + ( cos_y * -sin_z)                          * self.y + (         sin_y) * self.z                                  ,
			(-sin_x * -sin_y * cos_z + cos_x * sin_z) * self.x + (-sin_x * -sin_y * -sin_z + cos_x * cos_z) * self.y + (-sin_x * cos_y) * self.z,
			( cos_x * -sin_y * cos_z + sin_x * sin_z) * self.x + ( cos_x * -sin_y * -sin_z + sin_x * cos_z) * self.y + ( cos_x * cos_y) * self.z,
		)
	}

	#[must_use]
	pub fn with_y_inverted(&self) -> Vec3 {
		Self { x: self.x, y: -self.y, z: self.z }
	}

	pub fn invert_y(mut self) -> Vec3 {
		let self_mut = &mut self;
		self_mut.y = -self_mut.y;
		self
	}

	#[must_use]
	pub fn clip_space_to_screen_space(&self, screen_width: u16, screen_height: u16) -> IVec2 {
		clip_space_to_screen_space(self, screen_width, screen_height)
	}

	pub fn transform_by_mat3x3(&mut self, mat: &[f32]) {
		const SZ: u16 = 3;
		let x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)];
		let y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)];
		let z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)];

		self.x = x;
		self.y = y;
		self.z = z;
	}

	#[must_use]
	pub fn get_transformed_by_mat3x3(&self, mat: &[f32]) -> Vec3 {
		const SZ: u16 = 3;
		let x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)];
		let y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)];
		let z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)];

		Self { x, y, z }
	}

	#[must_use]
	pub fn get_transformed_by_mat4x4_discard_w(&self, mat: &[f32]) -> Vec3 {
		const SZ: u16 = 4;
		let x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)] + 1.0 * mat[xy_to_it(3, 0, SZ)];
		let y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)] + 1.0 * mat[xy_to_it(3, 1, SZ)];
		let z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)] + 1.0 * mat[xy_to_it(3, 2, SZ)];

		Self { x, y, z }
	}

	#[must_use]
	pub fn get_transformed_by_mat4x4_discard_w_and_translation(&self, mat: &[f32]) -> Vec3 {
		const SZ: u16 = 4;
		let x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)];
		let y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)];
		let z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)];

		Self { x, y, z }
	}

	#[must_use]
	pub fn get_transformed_by_mat4x4_homogeneous(&self, mat: &[f32]) -> Vec3 {
		const SZ: u16 = 4;
		let mut x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)] + 1.0 * mat[xy_to_it(3, 0, SZ)];
		let mut y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)] + 1.0 * mat[xy_to_it(3, 1, SZ)];
		let mut z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)] + 1.0 * mat[xy_to_it(3, 2, SZ)];
		let w     = self.x * mat[xy_to_it(0, 3, SZ)] + self.y * mat[xy_to_it(1, 3, SZ)] + self.z * mat[xy_to_it(2, 3, SZ)] + 1.0 * mat[xy_to_it(3, 3, SZ)];

		if w != 0.0 {
			x /= w;
			y /= w;
			z /= w;
		}

		Self { x, y, z }
	}

	pub fn get_transformed_by_mat4x4_w(&self, mat: &[f32]) -> Vec4 {
		const SZ: u16 = 4;
		let x = self.x * mat[xy_to_it(0, 0, SZ)] + self.y * mat[xy_to_it(1, 0, SZ)] + self.z * mat[xy_to_it(2, 0, SZ)] + 1.0 * mat[xy_to_it(3, 0, SZ)];
		let y = self.x * mat[xy_to_it(0, 1, SZ)] + self.y * mat[xy_to_it(1, 1, SZ)] + self.z * mat[xy_to_it(2, 1, SZ)] + 1.0 * mat[xy_to_it(3, 1, SZ)];
		let z = self.x * mat[xy_to_it(0, 2, SZ)] + self.y * mat[xy_to_it(1, 2, SZ)] + self.z * mat[xy_to_it(2, 2, SZ)] + 1.0 * mat[xy_to_it(3, 2, SZ)];
		let w = self.x * mat[xy_to_it(0, 3, SZ)] + self.y * mat[xy_to_it(1, 3, SZ)] + self.z * mat[xy_to_it(2, 3, SZ)] + 1.0 * mat[xy_to_it(3, 3, SZ)];

		Vec4 {
			xyz: Vec3 { x, y, z },
			w
		}
	}

	#[must_use]
	pub fn added(&self, x: f32, y: f32, z: f32) -> Vec3 {
		Vec3 {
			x: self.x + x,
			y: self.y + y,
			z: self.z + z,
		}
	}

	#[must_use]
	pub fn added_vec(&self, rhs: &Vec3) -> Vec3 {
		Vec3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}

	#[must_use]
	pub fn sub_vec(&self, rhs: &Vec3) -> Vec3 {
		Vec3 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}

	pub fn add_vec(mut self, rhs: &Vec3) -> Vec3 {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
		self
	}

	pub fn scale(mut self, scale: f32) -> Vec3 {
		self.x *= scale;
		self.y *= scale;
		self.z *= scale;
		self
	}

	pub fn mul_vec(&self, rhs: f32) -> Vec3 {
		Vec3 {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}

	pub fn inversed(&self) -> Vec3 {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

impl Vec3 {
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

	pub fn mid_point_of_tri(p0: &Vec3, p1: &Vec3, p2: &Vec3) -> Vec3 {
		Vec3::new(
			(p0.x + p1.x + p2.x) * 1./3.,
			(p0.y + p1.y + p2.y) * 1./3.,
			(p0.z + p1.z + p2.z) * 1./3.,
		)
	}
}

impl std::ops::Add<&Vec3> for &Vec3 {
	type Output = Vec3;

	fn add(self, rhs: &Vec3) -> Self::Output {
		self.added_vec(rhs)
	}
}

impl std::ops::Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Vec3) -> Self::Output {
		self.added_vec(&rhs)
	}
}

impl std::ops::Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: Vec3) -> Self::Output {
		self.sub_vec(&rhs)
	}
}

impl std::ops::Sub<&Vec3> for &Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: &Vec3) -> Self::Output {
		self.sub_vec(&rhs)
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

impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{:+.4}, {:+.4}, {:+.4}]", self.x, self.y, self.z)
	}
}

impl fmt::Debug for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:.6}, {:.6}, {:.6}", self.x, self.y, self.z)
	}
}