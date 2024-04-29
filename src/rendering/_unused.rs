

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


// pub fn draw_string(string: &str, pos: &UVec2, buffer: &mut [char], screen_width: u16) {
// 	let mut index = pos.y as usize * screen_width as usize + pos.x as usize;
// 	for ch in string.chars() {
// 		// TODO: bounds check
// 		if index > buffer.len() { continue; }
// 		buffer[index] = ch;
// 		index += 1;
// 	}
// }

// pub fn draw_char(ch: char, pos: &UVec2, buffer: &mut [char], screen_width: u16) {
// 	let index = pos.y as usize * screen_width as usize + pos.x as usize;
// 	if index > buffer.len() { return }
// 	buffer[index] = ch;
// }

// pub fn draw_mat4x4(mat: &[f32], pos: &UVec2, buffer: &mut [char], screen_width: u16) {
// 	let r0 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[ 0], mat[ 1], mat[ 2], mat[ 3]);
// 	draw_string(&r0, pos, buffer, screen_width);

// 	let r1 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[ 4], mat[ 5], mat[ 6], mat[ 7]);
// 	draw_string(&r1, &UVec2::new(pos.x, pos.y+1), buffer, screen_width);

// 	let r2 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[ 8], mat[ 9], mat[10], mat[11]);
// 	draw_string(&r2, &UVec2::new(pos.x, pos.y+2), buffer, screen_width);

// 	let r3 = format!("{:.2} {:.2} {:.2} {:.2}\n", mat[12], mat[13], mat[14], mat[15]);
// 	draw_string(&r3, &UVec2::new(pos.x, pos.y+3), buffer, screen_width);
// }

// pub fn draw_mesh_wire_and_normals(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), timer: &Timer, matrices: (&mut [f32], &mut [f32]), _camera: &Camera) {
// 	let (screen_width, screen_height) = width_height;
// 	let (proj_mat, transform_mat) = matrices;

// 	// apply_identity_to_mat_4x4(proj_mat);
// 	// apply_projection_to_mat_4x4(proj_mat, screen_width, screen_height);

// 	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

// 	let t = timer.time_aggr.as_millis() as f32 * 0.001;
// 	let (_angle_x, _angle_y, _angle_z) = (0.0, 0.0, 0.0);
// 	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);

// 	let _speed = 0.2;
// 	// let tmod = ((t * speed % 1.0) - 0.5).abs() * 2.0;
// 	let tmod = 0.6;
// 	let (scale_x, scale_y, scale_z) = (0.2 + 0.2 * tmod, 0.2 + 0.2 * tmod, 0.2 + 0.2 * tmod);


// 	apply_identity_to_mat_4x4(transform_mat);

// 	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
// 	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
// 	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

// 	multiply_4x4_matrices(proj_mat, transform_mat);

// 	let num_tris = mesh.tris_indices.len() / 3;
// 	for tri_i in 0..num_tris {

// 		let p0_i = tri_i * 3 + 0;
// 		let p1_i = tri_i * 3 + 1;
// 		let p2_i = tri_i * 3 + 2;


// 		let n0 = mesh.get_normal_at(p0_i);
// 		let trs_n0 = n0.get_transformed_by_mat4x4(proj_mat);

// 		let n1 = mesh.get_normal_at(p1_i);
// 		let trs_n1 = n1.get_transformed_by_mat4x4(proj_mat);

// 		let n2 = mesh.get_normal_at(p2_i);
// 		let trs_n2 = n2.get_transformed_by_mat4x4(proj_mat);


// 		let p0 = mesh.get_vert_at(p0_i);
// 		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);

// 		let p1 = mesh.get_vert_at(p1_i);
// 		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);

// 		let p2 = mesh.get_vert_at(p2_i);
// 		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);


// 		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
// 		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

// 		let screen_n0 = clip_space_to_screen_space(&(&trs_p0 + &trs_n0), screen_width, screen_height);
// 		let screen_n1 = clip_space_to_screen_space(&(&trs_p1 + &trs_n1), screen_width, screen_height);
// 		let screen_n2 = clip_space_to_screen_space(&(&trs_p2 + &trs_n2), screen_width, screen_height);

// 		// draw_string(&format!("n0 {:.2},{:.2},{:.2}", n0.x, n0.y, n0.z), &UVec2::new(0, 0), buffer, screen_width);
// 		// draw_string(&format!("n1 {:.2},{:.2},{:.2}", n1.x, n1.y, n1.z), &UVec2::new(0, 1), buffer, screen_width);
// 		// draw_string(&format!("n2 {:.2},{:.2},{:.2}", n2.x, n2.y, n2.z), &UVec2::new(0, 2), buffer, screen_width);

// 		// draw_string(&format!("n0 {:.2},{:.2},{:.2}", trs_n0.x, trs_n0.y, trs_n0.z), &UVec2::new(0, 4), buffer, screen_width);
// 		// draw_string(&format!("n1 {:.2},{:.2},{:.2}", trs_n1.x, trs_n1.y, trs_n1.z), &UVec2::new(0, 5), buffer, screen_width);
// 		// draw_string(&format!("n2 {:.2},{:.2},{:.2}", trs_n2.x, trs_n2.y, trs_n2.z), &UVec2::new(0, 6), buffer, screen_width);

// 		// draw_string(&format!("s n0 {},{}", screen_n0.x, screen_n0.y), &UVec2::new(0, 8), buffer, screen_width);
// 		// draw_string(&format!("s n1 {},{}", screen_n1.x, screen_n1.y), &UVec2::new(0, 9), buffer, screen_width);
// 		// draw_string(&format!("s n2 {},{}", screen_n2.x, screen_n2.y), &UVec2::new(0, 10), buffer, screen_width);

// 		draw_bresenham_line(&clip_space_to_screen_space(&trs_p0, screen_width, screen_height), &screen_n0, buffer, screen_width, '.');
// 		draw_bresenham_line(&screen_p1, &screen_n1, buffer, screen_width, '.');
// 		draw_bresenham_line(&screen_p2, &screen_n2, buffer, screen_width, '.');

// 		draw_bresenham_line(&clip_space_to_screen_space(&trs_p0, screen_width, screen_height), &screen_p1, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p2, &clip_space_to_screen_space(&trs_p0, screen_width, screen_height), buffer, screen_width, FILL_CHAR);

// 		draw_point(&screen_n0, buffer, screen_width, '@');
// 		draw_point(&screen_n1, buffer, screen_width, '@');
// 		draw_point(&screen_n2, buffer, screen_width, '@');
// 	}
// }

// pub fn draw_mesh_wire(mesh: &Mesh, buffer: &mut [char], width_height: (u16, u16), timer: &Timer, matrices: (&mut [f32], &mut [f32]), _camera: &Camera) {
// 	let (screen_width, screen_height) = width_height;
// 	let (proj_mat, transform_mat) = matrices;

// 	apply_identity_to_mat_4x4(proj_mat);
// 	apply_projection_to_mat_4x4(proj_mat, screen_width, screen_height);


// 	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

// 	let start_ms = 89_340;
// 	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
// 	let (angle_x, angle_y, angle_z) = (t * 0.1, t * 0.83, t * 1.2);
// 	// let (angle_x, angle_y, angle_z) = (0.0, 0.0, 0.0);

// 	let speed = 0.3;
// 	let sharpness = 2.5;

// 	let tri_wave = triangle_wave(t * speed);
// 	let t_smooth_wave = smoothed_0_to_1(tri_wave, sharpness);
// 	let tmod = lerp_f32(0.2, 0.4, t_smooth_wave);
// 	// let tmod = 1.0;
// 	let (scale_x, scale_y, scale_z) = (tmod, tmod, tmod);

// 	draw_string(&format!("{:.2}", t), &UVec2::new(0, 0), buffer, screen_width);
// 	draw_string(&format!("{:.2}", t_smooth_wave), &UVec2::new(0, 1), buffer, screen_width);
// 	draw_string(&format!("{:.2}", tmod), &UVec2::new(0, 2), buffer, screen_width);

// 	apply_identity_to_mat_4x4(transform_mat);
// 	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
// 	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
// 	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

// 	multiply_4x4_matrices(proj_mat, transform_mat);

// 	let num_tris = mesh.tris_indices.len() / 3;
// 	for tri_i in 0..num_tris {

// 		let p0_i = tri_i * 3 + 0;
// 		let p1_i = tri_i * 3 + 1;
// 		let p2_i = tri_i * 3 + 2;

// 		let p0 = mesh.get_vert_at(p0_i);
// 		let p1 = mesh.get_vert_at(p1_i);
// 		let p2 = mesh.get_vert_at(p2_i);

// 		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);
// 		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);
// 		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);

// 		let screen_p0 = clip_space_to_screen_space(&trs_p0, screen_width, screen_height);
// 		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
// 		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

// 		draw_bresenham_line(&screen_p0, &screen_p1, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p2, &screen_p0, buffer, screen_width, FILL_CHAR);
// 	}
// }


// pub fn draw_yade(yade_data: &YadeDemData, buffer: &mut [char], width_height: (u16, u16), timer: &Timer, matrices: (&mut [f32], &mut [f32]), _camera: &Camera) {
// 	let (screen_width, screen_height) = width_height;
// 	let (proj_mat, transform_mat) = matrices;

// 	apply_identity_to_mat_4x4(proj_mat);
// 	apply_projection_to_mat_4x4(proj_mat, screen_width, screen_height);


// 	let (pos_x, pos_y, pos_z) = (0.0, 0.0, 12.0);

// 	let start_ms = 89_340;
// 	let t = (timer.time_aggr.as_millis() + start_ms) as f32 * 0.001;
// 	let (angle_x, angle_y, angle_z) = (TAU * 0.25, t, 0.0);

// 	let speed = 0.3;
// 	let sharpness = 2.5;

// 	let tri_wave = triangle_wave(t * speed);
// 	let t_smooth_wave = smoothed_0_to_1(tri_wave, sharpness);
// 	let tmod = lerp_f32(0.2, 0.4, t_smooth_wave) * 15.0;
// 	// let tmod = 1.0;
// 	// let (scale_x, scale_y, scale_z) = (tmod, tmod, tmod);

// 	let scale = 12.0;
// 	let (scale_x, scale_y, scale_z) = (scale, scale, scale);

// 	draw_string(&format!("{:.2}", t), &UVec2::new(0, 0), buffer, screen_width);
// 	draw_string(&format!("{:.2}", t_smooth_wave), &UVec2::new(0, 1), buffer, screen_width);
// 	draw_string(&format!("{:.2}", tmod), &UVec2::new(0, 2), buffer, screen_width);

// 	apply_identity_to_mat_4x4(transform_mat);
// 	apply_scale_to_mat_4x4(transform_mat, scale_x, scale_y, scale_z);
// 	apply_rotation_to_mat_4x4(transform_mat, angle_x, angle_y, angle_z);
// 	apply_pos_to_mat_4x4(transform_mat, pos_x, pos_y, pos_z);

// 	multiply_4x4_matrices(proj_mat, transform_mat);

// 	for tri in yade_data.tris.iter() {

// 		let p0 = &tri.p0 + &tri.pos;
// 		let p1 = &tri.p1 + &tri.pos;
// 		let p2 = &tri.p2 + &tri.pos;

// 		let trs_p0 = p0.get_transformed_by_mat4x4(proj_mat);
// 		let trs_p1 = p1.get_transformed_by_mat4x4(proj_mat);
// 		let trs_p2 = p2.get_transformed_by_mat4x4(proj_mat);

// 		let screen_p0 = clip_space_to_screen_space(&trs_p0, screen_width, screen_height);
// 		let screen_p1 = clip_space_to_screen_space(&trs_p1, screen_width, screen_height);
// 		let screen_p2 = clip_space_to_screen_space(&trs_p2, screen_width, screen_height);

// 		draw_bresenham_line(&screen_p0, &screen_p1, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p1, &screen_p2, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&screen_p2, &screen_p0, buffer, screen_width, FILL_CHAR);
// 	}

// 	for circ in yade_data.circs.iter() {

// 		let circ_pos = circ.pos.get_transformed_by_mat4x4(proj_mat);
// 		let screen_circ = clip_space_to_screen_space(&circ_pos, screen_width, screen_height);

// 		draw_char('R', &screen_circ, buffer, screen_width);
// 	}
// }


// fn draw_bresenham_line(p0: &UVec2, p1: &UVec2, buffer: &mut [char], screen_width: u16, fill_char: char) {
// 	let x0 = p0.x as i32;
// 	let y0 = p0.y as i32;
// 	let x1 = p1.x as i32;
// 	let y1 = p1.y as i32;

// 	let mut x = x0;
// 	let mut y = y0;
// 	let dx = (x1 - x0).abs();
// 	let dy = (y1 - y0).abs();
// 	let sx = if x0 < x1 { 1 } else { -1 };
// 	let sy = if y0 < y1 { 1 } else { -1 };
// 	let mut deriv_diff = dx - dy;

// 	let i_screen_width = screen_width as i32;
// 	let mut index: usize;
// 	loop {
// 		index = (y * i_screen_width + x) as usize;

// 		// handle out of bounds
// 		if index < buffer.len() {
// 			buffer[index] = fill_char;
// 		}

// 		if x == x1 && y == y1 {
// 			break;
// 		}

// 		let double_deriv_diff = deriv_diff * 2;
// 		if double_deriv_diff > -dy {
// 			deriv_diff -= dy;
// 			x += sx;
// 		}
// 		if double_deriv_diff < dx {
// 			deriv_diff += dx;
// 			y += sy;
// 		}
// 	}

// }

// ////////////
// // TESTS: //
// ////////////

// pub fn draw_triangles_wire(screen_space_tris: &[ScreenTriangle], buffer: &mut [char], screen_width: u16) {
// 	for (index, tri) in screen_space_tris.iter().enumerate() {
// 		let i = index as u16;
// 		draw_bresenham_line(&tri.p0, &tri.p1, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&tri.p1, &tri.p2, buffer, screen_width, FILL_CHAR);
// 		draw_bresenham_line(&tri.p2, &tri.p0, buffer, screen_width, FILL_CHAR);


// 		draw_string(&format!("p0 {:?}", &tri.p0), &UVec2 { x: &tri.p0.x + 3, y: &tri.p0.y - 2 + i }, buffer, screen_width);
// 		draw_string(&format!("p1 {:?}", &tri.p1), &UVec2 { x: &tri.p1.x - 3, y: &tri.p1.y + 2 + i }, buffer, screen_width);
// 		draw_string(&format!("p2 {:?}", &tri.p2), &UVec2 { x: &tri.p2.x + 3, y: &tri.p2.y - 1 + i }, buffer, screen_width);

// 		let (topmost, secmost, trimost) = sort_by_y_prefer_left(&tri.p0, &tri.p1, &tri.p2);

// 		// TODO: learn the vector shit behind this
// 		let shortside_in_left =
// 			(secmost.y as f32 - topmost.y as f32) * (trimost.x as f32 - topmost.x as f32) >
// 			(secmost.x as f32 - topmost.x as f32) * (trimost.y as f32 - topmost.y as f32);
// 		if shortside_in_left {
// 			draw_string("the bend is on left",  &UVec2 { x: 4, y: 4+i }, buffer, screen_width);
// 		} else {
// 			draw_string("the bend is on right", &UVec2 { x: 4, y: 4+i }, buffer, screen_width);
// 		}

// 		// TODO: bounds check
// 		let index = (tri.p0.y * screen_width + tri.p0.x) as usize;
// 		buffer[index] = '@';

// 		let index = (tri.p1.y * screen_width + tri.p1.x) as usize;
// 		buffer[index] = '@';

// 		let index = (tri.p2.y * screen_width + tri.p2.x) as usize;
// 		buffer[index] = '@';
// 	}
// }

// // TODO: decent test
// pub fn test_bresenham(buffer: &mut [char], screen_width: u16, screen_height: u16, time_seed: i32) {
// 	draw_string(&format!("w:{}, h:{}", screen_width, screen_height), &UVec2::new(0, 0), buffer, screen_width);

// 	let middle = UVec2::new(screen_width / 2, screen_height / 2);

// 	let len = 20.0;
// 	let modulus = time_seed / 2 % 1000;
// 	let t = modulus as f32 / 1000.0;
// 	// let t_2 = ((t-0.5)).abs() * 2.0;

// 	let angle = t * TAU;

// 	let x = (angle.cos() * len * 2.0) as i16;
// 	let y = (angle.sin() * len) as i16;

// 	let up = UVec2::new((middle.x as i16 + x) as u16, (middle.y as i16 + y) as u16);

// 	// let up = UVec2::new((middle.x as i16) as u16, (middle.y as i16 + 15) as u16);
// 	// let up = UVec2::new((middle.x + 15), middle.y + 7);


// 	let direction: char;
// 	if angle < (TAU * 1.0/8.0) {
// 		direction = '↘';
// 	} else if angle < (TAU * 2.0/8.0) {
// 		direction = '↓';
// 	} else if angle < (TAU * 3.0/8.0) {
// 		direction = '↙';
// 	} else if angle < (TAU * 4.0/8.0) {
// 		direction = '←';
// 	} else if angle < (TAU * 5.0/8.0) {
// 		direction = '↖';
// 	} else if angle < (TAU * 6.0/8.0) {
// 		direction = '↑';
// 	} else if angle < (TAU * 7.0/8.0) {
// 		direction = '↗';
// 	} else {
// 		direction = '→';
// 	}

// 	draw_bresenham_line(&middle, &up, buffer, screen_width, direction);

// 	draw_point(&up, buffer, screen_width, '@');

// 	draw_string(&format!("{}", angle), &UVec2::new(0, 1), buffer, screen_width);
// 	draw_string(&format!("{}", up),    &UVec2::new(up.x+2, up.y), buffer, screen_width);

// 	// let right = &UVec2::new(middle.x + len, middle.y);
// 	// let left  = &UVec2::new(middle.x - len, middle.y);
// 	// let up    = &UVec2::new(middle.x, middle.y - len);
// 	// let down  = &UVec2::new(middle.x, middle.y + len/4);

// 	// let up_r  = &UVec2::new(right.x, up.y);
// 	// let up_l  = &UVec2::new(left.x, up.y);
// 	// ↖ ↑ ↗
// 	// ← · →
// 	// ↙ ↓ ↘

// 	// draw_bresenham_line(&middle, right, buffer, screen_width, '→');
// 	// draw_bresenham_line(&middle, left,  buffer, screen_width, '←');
// 	// draw_bresenham_line(&middle, up,    buffer, screen_width, '↑');
// 	// draw_bresenham_line(&middle, down,  buffer, screen_width, '↓');

// 	// draw_bresenham_line(&middle, up_l,  buffer, screen_width, '↖');
// 	// draw_bresenham_line(&middle, up_r,  buffer, screen_width, '↗');

// 	// draw_point(&middle, buffer, screen_width, '·');
// }



// fn sort_by_x<'a>(first: &'a UVec2, sec: &'a UVec2) -> (&'a UVec2, &'a UVec2) {
// 	if first.x > sec.x { (first, sec) } else { (sec, first) }
// }

// fn sort_by_y<'a>(first: &'a UVec2, sec: &'a UVec2) -> (&'a UVec2, &'a UVec2) {
// 	if first.y > sec.y { (first, sec) } else { (sec, first) }
// }
