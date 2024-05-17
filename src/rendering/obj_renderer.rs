use crate::{camera::Camera, mesh::Mesh, renderer::Renderer, terminal::TerminalBuffer, timer::Timer};

use crate::{maths::*, rendering::*};

const FILL_CHAR: char = '@';

pub struct ObjRenderer {
	mesh: Mesh,
	render: fn(mesh: &Mesh, &mut TerminalBuffer, &Timer, &Camera),
}

impl ObjRenderer {
	pub fn new(data: Mesh) -> Self {
		ObjRenderer {
			mesh: data,
			render: render_mesh,
		}
	}
}

impl Renderer for ObjRenderer {
	fn render(&self, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
		render_mesh(&self.mesh, buf, timer, camera);
	}
}

pub fn render_mesh(mesh: &Mesh, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);

	let start_ms = 89_340;
	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
	// teapot weirded out thing
	let start_ms = 95_374;
	let t = (start_ms) as f32 * 0.001;
	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);
	let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);

	let speed = 0.3;
	let sharpness = 2.5;

	let tri_wave = triangle_wave(t * speed);
	let t_smooth_wave = smoothed_0_to_1_s(tri_wave, sharpness);
	let tmod = lerp_f32(0.5, 1.0, t_smooth_wave);
	let (scale_x, scale_y, scale_z) = (tmod, tmod, tmod);
	let (scale_x, scale_y, scale_z) = (1.0, 1.0, 1.0);


	// this tweens the scale and rotation of the thing
	// let (pos_x, pos_y, pos_z) = (0.0, 0.0, 0.0);

	// let speed = 0.5;
	// let mut t = timer.time_aggr.as_millis() as f32 * 0.001 * speed;
	// if buf.test { t = 0.0; }

	// let (angle_x, angle_y, angle_z) = (t * 0.3, t, t * 2.1);

	// let speed = 0.6;
	// let tmod = ((t * speed % 1.0) - 0.5).abs() * 2.0;
	// render_string(&format!("{}", tmod), &UVec2::new(0, 7), buf);
	// let tmod_smooth = smoothed_0_to_1(tmod);
	// let animation_curve = tmod_smooth * 0.5 + 0.25;
	// let (scale_x, scale_y, scale_z) = (animation_curve, animation_curve, animation_curve);


	buf.copy_projection_to_render_matrix();

	apply_identity_to_mat_4x4(&mut buf.transf_mat);
	apply_scale_to_mat_4x4(&mut buf.transf_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(&mut buf.transf_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(&mut buf.transf_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);

	// buf.clear_debug();
	let num_tris = mesh.tris_indices.len() / 3;
	for tri_i in 0..num_tris {

		let p0_i = tri_i * 3 + 0;
		let p1_i = tri_i * 3 + 1;
		let p2_i = tri_i * 3 + 2;

		// buf.write_debug(&format!("gets p0 p1 p2   {} {} {}\n", p0_i, p1_i, p2_i));
		let p0 = mesh.get_vert_at(p0_i);
		let p1 = mesh.get_vert_at(p1_i);
		let p2 = mesh.get_vert_at(p2_i);

		let trs_p0 = p0.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);
		let trs_p1 = p1.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);
		let trs_p2 = p2.get_transformed_by_mat4x4_homogeneous(&buf.render_mat);

		let screen_p0 = clip_space_to_screen_space(&trs_p0, buf.wid, buf.hei);
		let screen_p1 = clip_space_to_screen_space(&trs_p1, buf.wid, buf.hei);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, buf.wid, buf.hei);

		render_bresenham_line(&screen_p0, &screen_p1, buf, FILL_CHAR);
		render_bresenham_line(&screen_p1, &screen_p2, buf, FILL_CHAR);
		render_bresenham_line(&screen_p2, &screen_p0, buf, FILL_CHAR);
	}
}