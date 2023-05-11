

// TODO: verts: Option<&'a [f32]>,
pub struct Mesh {
	pub verts: Vec<f32>,
	pub tris:  Vec<u16>,
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
    		tris: vec![
				// TODO: remember .obj store shit starting in 1
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
		}
	}

}


