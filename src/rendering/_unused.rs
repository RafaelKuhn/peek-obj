


#[deprecated]
fn bounding_square_clip_space(transformed_pos: &Vec3, rad_scaled: f32, render_mat_without_transform: &[f32], camera: &Camera) -> [Vec3; 4] {
	let trs_pos_up = transformed_pos.add_vec(&(camera.up * rad_scaled));
	let trs_pos_up_proj = trs_pos_up.get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform);

	let trs_pos_dw = transformed_pos.add_vec(&(camera.up * -rad_scaled));
	let trs_pos_dw_proj = trs_pos_dw.get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform);

	let trs_pos_right = transformed_pos.add_vec(&(camera.side * rad_scaled));
	let trs_pos_right_proj = trs_pos_right.get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform);

	let trs_pos_left = transformed_pos.add_vec(&(camera.side * -rad_scaled));
	let trs_pos_left_proj = trs_pos_left.get_transformed_by_mat4x4_homogeneous(&render_mat_without_transform);

	// TODO: I don't know exactly what's Z supposed to mean here but ok
	[
		Vec3::new(trs_pos_right_proj.x, trs_pos_up_proj.y, trs_pos_right_proj.z),
		Vec3::new(trs_pos_left_proj.x,  trs_pos_up_proj.y, trs_pos_left_proj.z),
		Vec3::new(trs_pos_right_proj.x, trs_pos_dw_proj.y, trs_pos_dw_proj.z),
		Vec3::new(trs_pos_left_proj.x,  trs_pos_dw_proj.y, trs_pos_dw_proj.z),
	]	
}




pub fn render_sphere(pos: &Vec3, rad: f32, ch: char, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {

	buf.copy_projection_to_render_matrix();
	apply_identity_to_mat_4x4(&mut buf.transf_mat);

	buf.clear_debug();

	// let mut rot_only_mat = create_identity_4x4();

	let mut proj_only_mat = create_identity_4x4();
	proj_only_mat.copy_from_slice(&buf.render_mat);


	let speed = 0.5;
	let mut t = timer.time_aggr.as_millis() as f32 * 0.001 * speed;
	// let mut t = TAU * 1./4.;
	// if buf.test { t = 0.0; }

	// let rad = 0.009926;
	// let rad = 0.01;
	// let pos = Vec3::new(-0.034109, -0.002092, -0.080507);
	// let pos = Vec3::new(0.02, 0., 0.);

	let pos = pos * YADE_SCALE_TEMP;
	let rad_sc = rad * YADE_SCALE_TEMP;

	buf.write_debug(&format!("{:?} original\n", pos));
	buf.write_debug(&format!("{:.6} RAD * SCALE\n", rad_sc));



	apply_rotation_to_mat_4x4_simple(&mut buf.transf_mat, 0.0, t, 0.0);

	multiply_4x4_matrices(&mut buf.render_mat, &camera.view_matrix);
	multiply_4x4_matrices(&mut buf.render_mat, &buf.transf_mat);


	multiply_4x4_matrices(&mut proj_only_mat, &camera.view_matrix);
	let rot_pos = pos.rotated_y(t);
	// buf.write_debug(&format!("{:?} rot only\n", rot_pos));
	let rot_pos_up = rot_pos.add_vec(&(camera.up * rad_sc));
	// buf.write_debug(&format!("{:?} rot up\n", rot_pos_up));
	let rot_pos_up_proj = rot_pos_up.get_transformed_by_mat4x4_uniform(&proj_only_mat);
	// buf.write_debug(&format!("{:?} rot up projected\n", rot_pos_up_proj));
	let rot_pos_up_proj_2d_f32 = clip_space_to_screen_space_f32(&rot_pos_up_proj, buf.wid, buf.hei);

	// let rot_pos_up_proj_2d = clip_space_to_screen_space(&rot_pos_up_proj, buf.wid, buf.hei);
	// buf.write_debug(&format!("{:?} rot_pos_up_proj_2d, BUT: {:?}\n", rot_pos_up_proj_2d, rot_pos_up_proj_2d_f32));

	let rot_pos_up_proj_2d_f = clip_space_to_screen_space_f32(&rot_pos_up_proj, buf.wid, buf.hei);
	// safe_render_char_signed('X', rot_pos_up_proj_2d.x, rot_pos_up_proj_2d.y, buf);



	let projected = pos.get_transformed_by_mat4x4_uniform(&buf.render_mat);
	buf.write_debug(&format!("\n\n{:?} projected\n", projected));
	// let projected_clip = normalize_clip_space(&projected);
	// buf.write_debug(&format!("{:?} normalized\n", projected_clip));
	let projected_2d = clip_space_to_screen_space(&projected, buf.wid, buf.hei);
	buf.write_debug(&format!("{:?} 2d of wh {:?}\n", projected_2d, IVec2::new(buf.wid.into(), buf.hei.into())));


	// let up_proj = rot_pos.add_vec(&(&camera.up * rad * YADE_SCALE_TEMP)).get_transformed_by_mat4x4_uniform(&buf.transf_mat);
	// // let up_proj = pos.add_vec(&(&camera.up * rad)).get_transformed_by_mat4x4_uniform(&proj_only_mat);
	// let scr = up_proj.clip_space_to_screen_space(buf.wid, buf.hei);
	// safe_render_char_signed('P', scr.x, scr.y, buf);

	
	let rot_pos_side = rot_pos.add_vec(&(&camera.side.inversed() * rad_sc));
	let rot_pos_side_proj = rot_pos_side.get_transformed_by_mat4x4_uniform(&proj_only_mat);
	let rot_pos_side_proj_2d = clip_space_to_screen_space(&rot_pos_side_proj, buf.wid, buf.hei);




	// safe_render_char_signed('c', projected_2d.x, projected_2d.y, buf);

	let ball_up = pos.add_vec(&(camera.up * rad * YADE_SCALE_TEMP)). get_transformed_by_mat4x4_uniform(&buf.render_mat);
	let ball_right = pos.add_vec(&(&camera.side.inversed() * rad * YADE_SCALE_TEMP)).get_transformed_by_mat4x4_uniform(&buf.render_mat);
	// let ball_up = ball_pos.add_vec(&(&camera.up * ball.rad * YADE_SCALE_TEMP));

	// buf.write_debug(&format!("{:}, r {:.6}, pos {:?} -> {:?} up {:?}\n", i, ball.rad, ball.pos, ball_pos, ball_up));

	let screen_circ = clip_space_to_screen_space(&projected, buf.wid, buf.hei);
	let screen_circ_f32 = clip_space_to_screen_space_f32(&projected, buf.wid, buf.hei);

	// safe_render_char_signed('U', screen_up.x, screen_up.y, buf);
	// safe_render_char_signed('S', screen_side.x, screen_side.y, buf);


	let dist = (rot_pos_up_proj_2d_f32.1 - screen_circ_f32.1).abs();
	// render_fill_bres_circle(&screen_circ, dist, '.', buf);
	render_fill_bres_circle(&screen_circ, dist, ch, buf);

	safe_render_char_signed('C', screen_circ.x, screen_circ.y, buf);

	safe_render_char_signed('U', rot_pos_up_proj_2d.x, rot_pos_up_proj_2d.y, buf);
	safe_render_char_signed('S', rot_pos_side_proj_2d.x, rot_pos_side_proj_2d.y, buf);
}



pub fn draw_mesh_wire_and_normals(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), timer: &Timer, matrices: (&mut [f32], &mut [f32]), _camera: &Camera) {
	let (screen_width, screen_height) = width_height;
	let (proj_mat, transform_mat) = matrices;

	// apply_identity_to_mat_4x4(proj_mat);
	// apply_projection_to_mat_4x4(proj_mat, screen_width, screen_height);

	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

	let t = timer.time_aggr.as_millis() as f32 * 0.001;
	let (_angle_x, _angle_y, _angle_z) = (0.0, 0.0, 0.0);
	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);

	let _speed = 0.2;
	// let tmod = ((t * speed % 1.0) - 0.5).abs() * 2.0;
	let tmod = 0.6;
	let (scale_x, scale_y, scale_z) = (0.2 + 0.2 * tmod, 0.2 + 0.2 * tmod, 0.2 + 0.2 * tmod);


	apply_identity_to_mat_4x4(transform_mat);

	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(proj_mat, transform_mat);

	let num_tris = mesh.tris_indices.len() / 3;
	for tri_i in 0..num_tris {

		let p0_i = tri_i * 3 + 0;
		let p1_i = tri_i * 3 + 1;
		let p2_i = tri_i * 3 + 2;


		let n0 = mesh.get_normal_at(p0_i);
		let trs_n0 = n0.get_transformed_by_mat4x4(proj_mat);

		let n1 = mesh.get_normal_at(p1_i);
		let trs_n1 = n1.get_transformed_by_mat4x4(proj_mat);

		let n2 = mesh.get_normal_at(p2_i);
		let trs_n2 = n2.get_transformed_by_mat4x4(proj_mat);


		let p0 = mesh.get_vert_at(p0_i);
		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);

		let p1 = mesh.get_vert_at(p1_i);
		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);

		let p2 = mesh.get_vert_at(p2_i);
		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);


		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

		let screen_n0 = clip_space_to_screen_space(&(&trs_p0 + &trs_n0), screen_width, screen_height);
		let screen_n1 = clip_space_to_screen_space(&(&trs_p1 + &trs_n1), screen_width, screen_height);
		let screen_n2 = clip_space_to_screen_space(&(&trs_p2 + &trs_n2), screen_width, screen_height);

		// draw_string(&format!("n0 {:.2},{:.2},{:.2}", n0.x, n0.y, n0.z), &UVec2::new(0, 0), buffer, screen_width);
		// draw_string(&format!("n1 {:.2},{:.2},{:.2}", n1.x, n1.y, n1.z), &UVec2::new(0, 1), buffer, screen_width);
		// draw_string(&format!("n2 {:.2},{:.2},{:.2}", n2.x, n2.y, n2.z), &UVec2::new(0, 2), buffer, screen_width);

		// draw_string(&format!("n0 {:.2},{:.2},{:.2}", trs_n0.x, trs_n0.y, trs_n0.z), &UVec2::new(0, 4), buffer, screen_width);
		// draw_string(&format!("n1 {:.2},{:.2},{:.2}", trs_n1.x, trs_n1.y, trs_n1.z), &UVec2::new(0, 5), buffer, screen_width);
		// draw_string(&format!("n2 {:.2},{:.2},{:.2}", trs_n2.x, trs_n2.y, trs_n2.z), &UVec2::new(0, 6), buffer, screen_width);

		// draw_string(&format!("s n0 {},{}", screen_n0.x, screen_n0.y), &UVec2::new(0, 8), buffer, screen_width);
		// draw_string(&format!("s n1 {},{}", screen_n1.x, screen_n1.y), &UVec2::new(0, 9), buffer, screen_width);
		// draw_string(&format!("s n2 {},{}", screen_n2.x, screen_n2.y), &UVec2::new(0, 10), buffer, screen_width);

		draw_bresenham_line(&clip_space_to_screen_space(&trs_p0, screen_width, screen_height), &screen_n0, buffer, screen_width, '.');
		draw_bresenham_line(&screen_p1, &screen_n1, buffer, screen_width, '.');
		draw_bresenham_line(&screen_p2, &screen_n2, buffer, screen_width, '.');

		draw_bresenham_line(&clip_space_to_screen_space(&trs_p0, screen_width, screen_height), &screen_p1, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p2, &clip_space_to_screen_space(&trs_p0, screen_width, screen_height), buffer, screen_width, FILL_CHAR);

		draw_point(&screen_n0, buffer, screen_width, '@');
		draw_point(&screen_n1, buffer, screen_width, '@');
		draw_point(&screen_n2, buffer, screen_width, '@');
	}
}

pub fn draw_mesh_wire(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), timer: &Timer, matrices: (&mut [f32], &mut [f32]), _camera: &Camera) {
	let (screen_width, screen_height) = width_height;
	let (proj_mat, transform_mat) = matrices;

	apply_identity_to_mat_4x4(proj_mat);
	apply_projection_to_mat_4x4(proj_mat, screen_width, screen_height);


	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

	let start_ms = 89_340;
	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);
	// let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);

	let speed = 0.3;
	let sharpness = 2.5;

	let tri_wave = triangle_wave(t * speed);
	let t_smooth_wave = smoothed_0_to_1(tri_wave, sharpness);
	let tmod = lerp_f32(0.2, 0.4, t_smooth_wave);
	// let tmod = 1.0;
	let (scale_x, scale_y, scale_z) = (tmod, tmod, tmod);

	draw_string(&format!("{:.2}", t), &UVec2::new(0, 0), buffer, screen_width);
	draw_string(&format!("{:.2}", t_smooth_wave), &UVec2::new(0, 1), buffer, screen_width);
	draw_string(&format!("{:.2}", tmod), &UVec2::new(0, 2), buffer, screen_width);

	apply_identity_to_mat_4x4(transform_mat);
	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

	multiply_4x4_matrices(proj_mat, transform_mat);

	let num_tris = mesh.tris_indices.len() / 3;
	for tri_i in 0..num_tris {

		let p0_i = tri_i * 3 + 0;
		let p1_i = tri_i * 3 + 1;
		let p2_i = tri_i * 3 + 2;

		let p0 = mesh.get_vert_at(p0_i);
		let p1 = mesh.get_vert_at(p1_i);
		let p2 = mesh.get_vert_at(p2_i);

		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);
		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);
		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);

		let screen_p0 = clip_space_to_screen_space(&trs_p0, screen_width, screen_height);
		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

		draw_bresenham_line(&screen_p0, &screen_p1, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
		draw_bresenham_line(&screen_p2, &screen_p0, buffer, screen_width, FILL_CHAR);
	}
}
