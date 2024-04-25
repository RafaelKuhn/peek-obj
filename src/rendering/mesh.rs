use crate::maths::Vec3;


pub struct Mesh {
	pub pos: Vec3,
	pub verts: Vec<f32>,
	pub tris_indices: Vec<u16>,
	pub normals: Vec<f32>,
	pub normal_indices: Vec<u16>,
}

impl Mesh {
	pub fn cube() -> Self {
		Self {
			pos: Vec3::zero(),
			verts: vec![
				 1.0, -1.0, -1.0,
				 1.0, -1.0,  1.0,
				-1.0, -1.0,  1.0,
				-1.0, -1.0, -1.0,
				 1.0,  1.0, -1.0,
				 1.0,  1.0,  1.0,
				-1.0,  1.0,  1.0,
				-1.0,  1.0, -1.0,
			],
    		tris_indices: vec![
				1, 2, 3,
				7, 6, 5,
				4, 5, 1,
				5, 6, 2,
				2, 6, 7,
				0, 3, 7,
				0, 1, 3,
				4, 7, 5,
				0, 4, 1,
				1, 5, 2,
				3, 2, 7,
				4, 0, 7,
			],
			normals: vec! [],
			normal_indices: vec! [],
		}
	}

	pub fn pillars() -> Self {
		Self {
			pos: Vec3::zero(),
			verts: vec![
				 0.5, 0.0,  0.5,
				 0.6, 0.0,  0.5,
				 0.5, 1.0,  0.5,

				-0.5, 0.0,  0.5,
				-0.5, 0.0,  0.6,
				-0.5, 1.0,  0.5,

				-0.5, 0.0, -0.5,
				-0.6, 0.0, -0.5,
				-0.5, 1.0, -0.5,

				 0.5, 0.0, -0.5,
				 0.5, 0.0, -0.6,
				 0.5, 1.0, -0.5,

				//  0.5,  2.1,  0.5,
				//  0.6,  2.1,  0.5,
				//  0.5,  1.1,  0.5,

				// -0.5,  2.1,  0.5,
				// -0.5,  2.1,  0.6,
				// -0.5,  1.1,  0.5,

				// -0.5,  2.1, -0.5,
				// -0.6,  2.1, -0.5,
				// -0.5,  1.1, -0.5,

				//  0.5,  2.1, -0.5,
				//  0.5,  2.1, -0.6,
				//  0.5,  1.1, -0.5,
			],
    		tris_indices: vec![
				0, 1, 2,
				3, 4, 5,
				6, 7, 8,
				9, 10, 11,

				0, 3, 6,
				6, 9, 0,
			],
			normals: vec! [],
			normal_indices: vec! [],
		}
	}

	pub fn quad1x1() -> Self {
		Self {
			pos: Vec3::zero(),
			verts: vec![
				 1.0,  0.0,  1.0,
				-1.0,  0.0,  1.0,
				 1.0,  0.0, -1.0,
				-1.0,  0.0, -1.0,
			],
			tris_indices: vec![
				0, 1, 2,
				2, 3, 0,
			],
			normals: vec! [],
			normal_indices: vec! [],
		}
	}

	pub fn get_vert_at(&self, index: usize) -> Vec3 {
		let tri_index = self.tris_indices[index];

		const SZ: usize = 3;
		Vec3::new(
			 self.verts[tri_index as usize * SZ + 0],
			 self.verts[tri_index as usize * SZ + 1],
			 self.verts[tri_index as usize * SZ + 2],
		)
	}

	pub fn get_normal_at(&self, index: usize) -> Vec3 {
		let normal_index = self.normal_indices[index];

		const SZ: usize = 3;
		Vec3::new(
			 self.normals[normal_index as usize * SZ + 0],
			 self.normals[normal_index as usize * SZ + 1],
			 self.normals[normal_index as usize * SZ + 2],
		)
	}

	pub fn invert_mesh_yz(&mut self) {
		let mut i = 0;
		while i < self.verts.len() {

			// let vert_x = self.verts[i];
			i += 1;

			let vert_y = self.verts[i];
			i += 1;
			let vert_z = self.verts[i];

			// y
			self.verts[i-1] = vert_z;
			// z
			self.verts[i] = vert_y;

			i += 1;
		}
	}
}
