use crate::maths::*;


pub struct Camera {
	pub position: Vec3,
	pub rotation: Vec3,
	pub view_matrix: Vec<f32>,
}

impl Camera {
	pub fn new() -> Camera {
		Self {
			position: Vec3::zero(),
			rotation: Vec3::zero(),
			view_matrix: build_identity_4x4(),
		}
	}

	pub fn get_pos(&self) -> Vec3 {
		Vec3 {
			x: self.position.x,
			y: self.position.y,
			z: self.position.z,
		}
	}

	pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
		self.position.x = x;
		self.position.y = y;
		self.position.z = z;
	}

	pub fn set_rot(&mut self, x: f32, y: f32, z: f32) {
		self.rotation.x = x;
		self.rotation.y = y;
		self.rotation.z = z;
	}

	// TODO: implement
	pub fn look_at(&mut self, _dest: &Vec3) {
		panic!("not implemented");
	}

	pub fn update_view_matrix(&mut self) {

		self.view_matrix = create_view_matrix(&self.position, &self.rotation);

		// apply_pos_vec_to_mat_4x4(&mut self.view_matrix, &self.position.inversed());
		// apply_identity_to_mat_4x4(&mut self.view_matrix);

		// apply_scale_to_mat_4x4(&mut self.view_matrix, 1.0, 1.0, 1.0);
		// apply_rotation_to_mat_4x4(&mut self.view_matrix, self.rotation.x, self.rotation.y, self.rotation.z);
		// apply_pos_to_mat_4x4(&mut self.view_matrix, self.position.x, self.position.y, self.position.z);
	}

}
