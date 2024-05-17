use std::f32::consts::TAU;

use crate::{app::App, maths::*, render_string, timer::Timer, TerminalBuffer};


pub struct Camera {
	pub position: Vec3,
	pub rotation: Vec3,
	pub view_matrix: Vec<f32>,

	pub forward: Vec3,
	pub up: Vec3,
	pub side: Vec3,

	initial_position: Vec3,
	initial_rotation: Vec3,

	pub cache_rot_x: f32,
	pub cache_rot_y: f32,

	pub cache_dist: f32,
}

impl Camera {
	pub fn new() -> Camera {
		Self {
			position: Vec3::zero(),
			rotation: Vec3::zero(),

			forward: Vec3::new(0.0, 0.0, 1.0),
			up:      Vec3::new(0.0, 1.0, 0.0),
			side:    Vec3::new(1.0, 0.0, 0.0),

			view_matrix: create_identity_4x4(),
			initial_position: Vec3::zero(),
			initial_rotation: Vec3::zero(),

			cache_rot_x: 0.,
			cache_rot_y: 0.,

			cache_dist: 0.,
		}
	}

	pub fn consume_user_data(&mut self, app: &mut App) {

		let camera = self;

		if app.called_reset_camera {
			app.called_reset_camera = false;

			camera.restore_initial_pos_and_rot();
			if !app.is_free_mov() {
				camera.clear_cache();
			}

			camera.update_view_matrix();
		}

		if app.called_set_camera_default_orientation {
			app.called_set_camera_default_orientation = false;

			if app.is_free_mov() {
				camera.set_initial_pos(camera.position.x, camera.position.y, camera.position.z);
				camera.set_initial_rot(camera.rotation.x, camera.rotation.y, camera.rotation.z);
			}
		}

		if app.called_toggle_free_mov {
			app.called_toggle_free_mov = false;

			app.toggle_free_mov(camera);
		}

		if app.is_free_mov() {
			let dir_vec = camera.forward * app.user_dir.z + camera.side * app.user_dir.x + camera.up * app.user_dir.y;
			camera.position = camera.position + dir_vec;
			camera.rotation = camera.rotation + app.user_rot;
			camera.update_view_matrix();
			return;
		}

		Self::solve_orbital_movement(camera, app);
	}

	fn solve_orbital_movement(camera: &mut Camera, app: &mut App) {

		let ang_increment_x = app.user_dir.y;
		let ang_increment_y = app.user_dir.x;

		camera.cache_rot_x += ang_increment_x;
		camera.cache_rot_y += ang_increment_y;

		camera.cache_dist += app.user_dir.z;

		// TODO: this will not work with any initial position that does not go in the direction (0.0, 0.0, 1.0)
		// let initial_pos_dir = camera.initial_position.normalized();
		// let base_pos = camera.initial_position + initial_pos_dir * camera.cache_dist;

		let base_pos = Vec3::new(0.0, 0.0, 16.0) + Vec3::new(0.0, 0.0, camera.cache_dist);

		camera.position = base_pos
		.rotated_x(camera.cache_rot_x)
		.rotated_y(camera.cache_rot_y)
		;

		camera.rotation.x = -camera.cache_rot_x;
		camera.rotation.y = -camera.cache_rot_y;

		// (won't work) this makes the camera obey arrow keys to rotate instead of orbiting
		// camera.rotation = camera.rotation + app.user_rot;

		camera.update_view_matrix();
	}

	#[deprecated(since="0.0", note="this shit does not work")]
	pub fn look_at(&mut self, target: &Vec3) {
		let direction = (target - &self.position).normalized();

		// Calculate the rotation angles
		let pitch = direction.y.atan2((direction.x.powi(2) + direction.z.powi(2)).sqrt());
		let yaw = direction.x.atan2(direction.z);

		// self.rotation.x = pitch;
		// self.rotation.y = yaw;

		self.rotation.x = -pitch;
		self.rotation.y = yaw + TAU /4.0;
	}

	pub fn reset_cached_dist(&mut self) {
		self.cache_dist = 0.0;
	}

	pub fn configure_defaults(&mut self, app: &mut App) {

		// DEFAULT (production)
		self.set_initial_pos(0.0, 0.0, 16.0);
		self.set_initial_rot(0.0, 0.0, 0.0);

		#[cfg(debug_assertions)] {

			// if !app.is_free_mov() { app.toggle_free_mov(self) }

			// // TODO: use this to debug (AXIS_SZ_WORLD == 20.0) go forward
			// self.set_initial_pos(0.589387, 0.680431, 2.741510);
			// self.set_initial_rot(0.220893, -0.196350, 0.000000);

			// // use this to debug frustum clipping with the blender blade
			// self.set_initial_pos(1.448884, -0.630452, -0.223440);
			// self.set_initial_rot(0.098175, -1.693514, 0.000000);

			// from above
			// self.set_initial_pos(0.000000, 13.021900, 4.108858);
			// self.set_initial_rot(1.251728, 0.000000, 0.000000);
	
			// self in front
			// self.set_initial_pos(0.0, 0.0, 5.0);
			// self.set_initial_rot(0.00, 0.00, 0.00);
	
			// self from above
			// self.set_initial_pos(0.0, 7.7989, 0.1271);
			// self.set_initial_rot(1.52, 0.00, 0.00);
	
			// self from side
			// self.set_initial_pos(8.585467,  3.822423, 0.048875);
			// self.set_initial_rot(0.392699, -1.570797, 0.000000);
	
			// can see the 3 axes
			// self.set_initial_pos(6.560868, 3.081584, 5.002097);
			// self.set_initial_rot(0.343612, -0.932661, 0.000000);
	
			// can see the 3 axes a little far
			// self.set_initial_pos(16.997_18, 7.730669, 12.742184);
			// self.set_initial_rot(0.343612, -0.932661, 0.000000);
	
			// a little bit up and to the right (good for bounding boxes)
			// self.set_initial_pos(2.398537, 2.217667, 11.542053);
			// self.set_initial_rot(0.147262, -0.147262, 0.000000);
	
			// up close to debug balls clipping (yade debug)
			// self.set_initial_pos(-0.035866, 0.622454, 2.083412);
			// self.set_initial_rot(0.343612, 0.245437, 0.000000);
		}

		self.update_view_matrix();
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

	pub fn clear_cache(&mut self) {
		self.cache_dist  = 0.0;
		self.cache_rot_x = 0.0;
		self.cache_rot_y = 0.0;
	}


	#[deprecated]
	fn find_up_and_forward(&self) -> (Vec3, Vec3) {
		let mut mat = create_identity_4x4_arr();
		apply_rotation_to_mat_4x4(&mut mat, self.rotation.x, self.rotation.y, self.rotation.z);

		let cam_up = Vec3::new(0.0, 1.0, 0.0);
		let cam_up = cam_up.get_transformed_by_mat4x4_homogeneous(&mat);

		let cam_forward = Vec3::new(0.0, 0.0, 1.0);
		let cam_forward = cam_forward.get_transformed_by_mat4x4_homogeneous(&mat);

		(cam_up, cam_forward)
	}

	pub fn update_view_matrix(&mut self) {

		let cos_x = self.rotation.x.cos();
		let sin_x = self.rotation.x.sin();

		let cos_y = self.rotation.y.cos();
		let sin_y = self.rotation.y.sin();

		// let cos_z = self.rotation.z.cos();
		// let sin_z = self.rotation.z.sin();

		// accounts for Z
		// let cam_up = Vec3::new(cos_x * sin_z + sin_x * sin_y * cos_z, cos_x * cos_z - sin_x * sin_y * sin_z, -sin_x * cos_y);

		self.up = Vec3::new(
			sin_x * sin_y,
			cos_x,
			-sin_x * cos_y,
		);

		// accounts for Z
		// let cam_forward = Vec3::new(sin_x * sin_z - cos_x * sin_y * cos_z, sin_x * cos_z + cos_x * sin_y * sin_z, cos_x * cos_y,);

		self.forward = Vec3::new(
			-cos_x * sin_y,
			sin_x,
			cos_x * cos_y,
		);

		self.side = Vec3::cross_product(&self.forward, &self.up);

		// buf.clear_debug();
		// buf.write_debug(&format!("pos     {:}\n", position));
		// buf.write_debug(&format!("rot     {:}\n", rotation));
		// buf.write_debug("- - - -\n");
		// buf.write_debug(&format!("c fward {:}\n", cam_forward));
		// buf.write_debug(&format!("c side  {:}\n", cam_side));
		// buf.write_debug(&format!("c up    {:}\n", cam_up));

		const SZ: u16 = 4;

		self.view_matrix[xy_to_it(0, 0, SZ)] = self.side.x;
		self.view_matrix[xy_to_it(1, 0, SZ)] = self.side.y;
		self.view_matrix[xy_to_it(2, 0, SZ)] = self.side.z;

		self.view_matrix[xy_to_it(0, 1, SZ)] = self.up.x;
		self.view_matrix[xy_to_it(1, 1, SZ)] = self.up.y;
		self.view_matrix[xy_to_it(2, 1, SZ)] = self.up.z;

		self.view_matrix[xy_to_it(0, 2, SZ)] = -self.forward.x;
		self.view_matrix[xy_to_it(1, 2, SZ)] = -self.forward.y;
		self.view_matrix[xy_to_it(2, 2, SZ)] = -self.forward.z;

		self.view_matrix[xy_to_it(3, 0, SZ)] = -Vec3::dot_product(&self.side, &self.position);
		self.view_matrix[xy_to_it(3, 1, SZ)] = -Vec3::dot_product(&self.up, &self.position);
		self.view_matrix[xy_to_it(3, 2, SZ)] =  Vec3::dot_product(&self.forward, &self.position);
	}

}