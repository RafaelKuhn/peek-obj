use crate::{maths::*, terminal_wrapper::TerminalBuffer};


pub struct Camera {
	pub position: Vec3,
	pub rotation: Vec3,
	pub view_matrix: Vec<f32>,

	initial_position: Vec3,
	initial_rotation: Vec3,
}

impl Camera {
	pub fn new() -> Camera {
		Self {
			position: Vec3::zero(),
			rotation: Vec3::zero(),
			view_matrix: build_identity_4x4(),
			initial_position: Vec3::zero(),
			initial_rotation: Vec3::zero(),
		}
	}

	pub fn set_initial_pos(&mut self, x: f32, y: f32, z: f32) {
		self.initial_position = Vec3::new(x, y, z);
		self.position = self.initial_position;
	}

	pub fn set_initial_rot(&mut self, x: f32, y: f32, z: f32) {
		self.initial_rotation = Vec3::new(x, y, z);
		self.rotation = self.initial_rotation;
	}

	pub fn restore_initial_pos_and_rot(&mut self) {
		self.position = self.initial_position;
		self.rotation = self.initial_rotation;
	}

	// TODO: implement
	pub fn look_at(&mut self, _dest: &Vec3) {
		panic!("not implemented");
	}

	pub fn update_view_matrix(&mut self, buf: &mut TerminalBuffer) {
		self.view_matrix = create_view_matrix(&self.position, &self.rotation, buf);
	}

}
