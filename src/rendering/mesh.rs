
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

}


