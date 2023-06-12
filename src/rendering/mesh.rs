use crate::maths::Vec3;


pub struct Mesh {
	pub verts: Vec<f32>,
	pub tris_indices: Vec<u16>,
	pub normals: Vec<f32>,
	pub normal_indices: Vec<u16>,
}


impl Mesh {
	pub fn cube() -> Self {
		Self {
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
	
	pub fn get_vert_at(&self, index: usize) -> Vec3 {
		let tri_index = self.tris_indices[index];
		
		let sz = 3;
		Vec3::new(
			 self.verts[(tri_index * sz) as usize + 0],
			-self.verts[(tri_index * sz) as usize + 1],
			 self.verts[(tri_index * sz) as usize + 2],
		)
	}

	pub fn get_normal_at(&self, index: usize) -> Vec3 {
		let normal_index = self.normal_indices[index];
		
		let sz = 3;
		Vec3::new(
			 self.normals[(normal_index * sz) as usize + 0],
			-self.normals[(normal_index * sz) as usize + 1],
			 self.normals[(normal_index * sz) as usize + 2],
		)
	}
}


