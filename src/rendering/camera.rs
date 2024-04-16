use crate::{maths::*, render_bresenham_line, render_char, render_string, screen_project, terminal_wrapper::TerminalBuffer};


pub struct Camera {
	pub position: Vec3,
	pub rotation: Vec3,
	pub view_matrix: Vec<f32>,

	pub forward: Vec3,
	pub up: Vec3,
	pub side: Vec3,

	initial_position: Vec3,
	initial_rotation: Vec3,
}

impl Camera {
	pub fn new() -> Camera {
		Self {
			position: Vec3::zero(),
			rotation: Vec3::zero(),

			forward: Vec3::new(0.0, 0.0, 1.0),
			up:      Vec3::new(0.0, 1.0, 0.0),
			side:    Vec3::new(1.0, 0.0, 0.0),

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

	// fn find_up_and_side() {
	// 	let mut mat = build_identity_4x4();
	// 	apply_rotation_to_mat_4x4(&mut mat, rotation.x, rotation.y, rotation.z);

	// 	let cam_up = Vec3::new(0.0, 1.0, 0.0);
	// 	let cam_up = cam_up.get_transformed_by_mat4x4(&mat);

	// 	let cam_forward = Vec3::new(0.0, 0.0, 1.0);
	// 	let cam_forward = cam_forward.get_transformed_by_mat4x4(&mat);
	// }

	pub fn update_view_matrix(&mut self, buf: &mut TerminalBuffer) {

		let cos_x = self.rotation.x.cos();
		let sin_x = self.rotation.x.sin();

		let cos_y = self.rotation.y.cos();
		let sin_y = self.rotation.y.sin();

		// let cos_z = self.rotation.z.cos();
		// let sin_z = self.rotation.z.sin();

		// accounts for Z
		// let cam_up = Vec3::new(cos_x * sin_z + sin_x * sin_y * cos_z, cos_x * cos_z - sin_x * sin_y * sin_z, -sin_x * cos_y);

		self.up = normalize(&Vec3::new(
			sin_x * sin_y,
			cos_x,
			-sin_x * cos_y,
		));

		// accounts for Z
		// let cam_forward = Vec3::new(sin_x * sin_z - cos_x * sin_y * cos_z, sin_x * cos_z + cos_x * sin_y * sin_z, cos_x * cos_y,);

		self.forward = normalize(&Vec3::new(
			- cos_x * sin_y,
			sin_x,
			cos_x * cos_y,
		));

		self.side = normalize(&cross_product(&self.forward, &self.up));
		// let cam_up = cross_product(&cam_side, &cam_forward);


		// TODO: move it correctly to fit the right top corner or smth
		// see how much space does this occupy

		buf.copy_projection_to_render_matrix();
		// in world space, 5 units back (camera is irrelevant for these calculations)
		// TODO: draw gizmos logic, in rendering.rs

		const GIZMO_SIZE_WORLD: f32 = 0.15;
	
		let base_world_space = Vec3::new(0.0, 0.0, -8.0);
		let origin = screen_project(&base_world_space, &buf.render_mat, buf.wid, buf.hei);
		let close_to_base_world = base_world_space.add_vec(&Vec3::new(GIZMO_SIZE_WORLD, 0.0, 0.0));
		let right_size = screen_project(&close_to_base_world, &buf.render_mat, buf.wid, buf.hei);

		let side_offset = (right_size.x - origin.x) as i16;
		let screen_offset = (
			    buf.wid as i16 / 2 -   side_offset       - 1,
			- ( buf.hei as i16 / 2 - ( side_offset / 2 ) - 1 )
		);

		let origin = origin.sum_t(screen_offset);

		let dbg_forward = self.forward.invert_y();
		let dbg_side = self.side.inversed().invert_y();
		let dbg_up = self.up.invert_y();

		let ptr = screen_project(&(base_world_space + (dbg_up * GIZMO_SIZE_WORLD)), &buf.render_mat, buf.wid, buf.hei).sum_t(screen_offset);
		let ch = if dot_product(&Vec3::new(0.0, 0.0, 1.0), &dbg_up) > 0.0 { 'y' } else { 'Y' };
		render_bresenham_line(&origin, &ptr, buf, ch);
		render_char('O', &ptr, buf);
		
		let ptr = screen_project(&(base_world_space + (&dbg_side * GIZMO_SIZE_WORLD)), &buf.render_mat, buf.wid, buf.hei).sum_t(screen_offset);
		let ch = if dot_product(&Vec3::new(0.0, 0.0, 1.0), &dbg_side) > 0.0 { 'x' } else { 'X' };
		render_bresenham_line(&origin, &ptr, buf, ch);
		render_char('O', &ptr, buf);

		let ptr = screen_project(&(base_world_space + (&dbg_forward * GIZMO_SIZE_WORLD)), &buf.render_mat, buf.wid, buf.hei).sum_t(screen_offset);
		let ch = if dot_product(&Vec3::new(0.0, 0.0, 1.0), &dbg_forward) > 0.0 { 'z' } else { 'Z' };
		render_bresenham_line(&origin, &ptr, buf, ch);
		render_char('O', &ptr, buf);

		render_char('O', &origin, buf);
		// END TODO: draw gizmos logic, in rendering.rs


		// TODO: dump to file logic
		// buf.clear_debug();
		// buf.write_debug(&format!("pos     {:}\n", position));
		// buf.write_debug(&format!("rot     {:}\n", rotation));
		// buf.write_debug("- - - -\n");
		// buf.write_debug(&format!("c fward {:}\n", cam_forward));
		// buf.write_debug(&format!("c side  {:}\n", cam_side));
		// buf.write_debug(&format!("c up    {:}\n", cam_up));
		
		const SZ: u16 = 4;

		self.view_matrix[xy_to_i(0, 0, SZ)] = self.side.x;
		self.view_matrix[xy_to_i(1, 0, SZ)] = self.side.y;
		self.view_matrix[xy_to_i(2, 0, SZ)] = self.side.z;

		self.view_matrix[xy_to_i(0, 1, SZ)] = self.up.x;
		self.view_matrix[xy_to_i(1, 1, SZ)] = self.up.y;
		self.view_matrix[xy_to_i(2, 1, SZ)] = self.up.z;

		self.view_matrix[xy_to_i(0, 2, SZ)] = -self.forward.x;
		self.view_matrix[xy_to_i(1, 2, SZ)] = -self.forward.y;
		self.view_matrix[xy_to_i(2, 2, SZ)] = -self.forward.z;

		self.view_matrix[xy_to_i(3, 0, SZ)] = -dot_product(&self.side, &self.position);
		self.view_matrix[xy_to_i(3, 1, SZ)] = -dot_product(&self.up, &self.position);
		self.view_matrix[xy_to_i(3, 2, SZ)] = dot_product(&self.forward, &self.position);
		// self.view_matrix[xy_to_i(3, 3, SZ)] = 1.0;
	}

}
