use crate::{camera::Camera, TerminalBuffer};

use crate::{rendering::*, maths::*};



pub fn cull_tri_into_screen_space(p0: &Vec3, p1: &Vec3, p2: &Vec3, camera: &Camera, buf: &mut TerminalBuffer) -> Option<ScreenTri> {

	// buf.write_debug("\n\n");
	let trs_p0 = p0.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
	let from_trs_p0_to_cam = camera.position - trs_p0;
	let trs_p1 = p1.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
	let from_trs_p1_to_cam = camera.position - trs_p1;
	let trs_p2 = p2.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
	let from_trs_p2_to_cam = camera.position - trs_p2;

	let are_all_far_away_from_cam =
		trs_p0.squared_dist_to(&camera.position) > SQUARED_ZF &&
		trs_p1.squared_dist_to(&camera.position) > SQUARED_ZF &&
		trs_p2.squared_dist_to(&camera.position) > SQUARED_ZF;

	if are_all_far_away_from_cam { return None }

	let is_completely_off_camera =
		(Vec3::dot_product(&from_trs_p0_to_cam.normalized(), &camera.forward) - 1.0).abs() > MAX_DOT_PROD_DIST_FROM_1 &&
		(Vec3::dot_product(&from_trs_p1_to_cam.normalized(), &camera.forward) - 1.0).abs() > MAX_DOT_PROD_DIST_FROM_1 &&
		(Vec3::dot_product(&from_trs_p2_to_cam.normalized(), &camera.forward) - 1.0).abs() > MAX_DOT_PROD_DIST_FROM_1;

	// buf.write_debug(&format!(" dot trs_p0 {:.6}\n", (Vec3::dot_product(&from_trs_p0_to_cam.normalized(), &camera.forward) - 1.0).abs()));
	// buf.write_debug(&format!(" dot trs_p1 {:.6}\n", (Vec3::dot_product(&from_trs_p1_to_cam.normalized(), &camera.forward) - 1.0).abs()));
	// buf.write_debug(&format!(" dot trs_p2 {:.6}\n", (Vec3::dot_product(&from_trs_p2_to_cam.normalized(), &camera.forward) - 1.0).abs()));
	// buf.write_debug(&format!(" completely off? {} \n", is_completely_off_camera));
	if is_completely_off_camera { return None }

	let p0_clip = p0.get_transformed_by_mat4x4_w(&buf.render_mat);
	let p1_clip = p1.get_transformed_by_mat4x4_w(&buf.render_mat);
	let p2_clip = p2.get_transformed_by_mat4x4_w(&buf.render_mat);

	// the triangle is inside the screen, don't cull
	// buf.write_debug(&format!("p0 {:?} in w ran {} \n", p0_clip, p0_clip.in_w_range()));
	if p0_clip.in_w_range() { return Some(ScreenTri::from_clip_space(p0_clip, p1_clip, p2_clip, buf.wid, buf.hei)) }
	// buf.write_debug(&format!("p1 {:?} in w ran {} \n", p1_clip, p1_clip.in_w_range()));
	if p1_clip.in_w_range() { return Some(ScreenTri::from_clip_space(p0_clip, p1_clip, p2_clip, buf.wid, buf.hei)) }
	// buf.write_debug(&format!("p2 {:?} in w ran {} \n\n", p2_clip, p2_clip.in_w_range()));
	if p2_clip.in_w_range() { return Some(ScreenTri::from_clip_space(p0_clip, p1_clip, p2_clip, buf.wid, buf.hei)) }

	let screen_p0 = clip_space_to_screen_space_f(&p0_clip.homogeneous(), buf.wid, buf.hei);
	let screen_p1 = clip_space_to_screen_space_f(&p1_clip.homogeneous(), buf.wid, buf.hei);
	let screen_p2 = clip_space_to_screen_space_f(&p2_clip.homogeneous(), buf.wid, buf.hei);

	let (width, height) = (buf.wid as f32, buf.hei as f32);

	// buf.write_debug(&format!("p0..p1 inters scr {}\n", line_intersect_screen(&screen_p0, &screen_p1, width, height)));
	if line_intersect_screen(&screen_p0, &screen_p1, width, height) { return Some(ScreenTri::from_screen_points(screen_p0, screen_p1, screen_p2)) }
	// buf.write_debug(&format!("p1..p2 inters scr {}\n", line_intersect_screen(&screen_p1, &screen_p2, width, height)));
	if line_intersect_screen(&screen_p1, &screen_p2, width, height) { return Some(ScreenTri::from_screen_points(screen_p0, screen_p1, screen_p2)) }
	// buf.write_debug(&format!("p2..p0 inters scr {}\n", line_intersect_screen(&screen_p2, &screen_p0, width, height)));
	if line_intersect_screen(&screen_p2, &screen_p0, width, height) { return Some(ScreenTri::from_screen_points(screen_p0, screen_p1, screen_p2)) }

	None	
}

pub fn cull_line_into_screen_space(p0: &Vec3, p1: &Vec3, camera: &Camera, buf: &mut TerminalBuffer) -> Option<Line> {

	buf.write_debug("\n\n");
	let trs_p0 = p0.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
	let from_trs_p0_to_cam = camera.position - trs_p0;
	let trs_p1 = p1.get_transformed_by_mat4x4_discard_w(&buf.transf_mat);
	let from_trs_p1_to_cam = camera.position - trs_p1;

	let are_all_far_away_from_cam =
		trs_p0.squared_dist_to(&camera.position) > SQUARED_ZF &&
		trs_p1.squared_dist_to(&camera.position) > SQUARED_ZF;

	if are_all_far_away_from_cam { return None }

	let is_completely_off_camera =
		(Vec3::dot_product(&from_trs_p0_to_cam.normalized(), &camera.forward) - 1.0).abs() > MAX_DOT_PROD_DIST_FROM_1 &&
		(Vec3::dot_product(&from_trs_p1_to_cam.normalized(), &camera.forward) - 1.0).abs() > MAX_DOT_PROD_DIST_FROM_1;

	// buf.write_debug(&format!(" trs_p0  {:?} dot {:.6}\n", trs_p0, (Vec3::dot_product(&from_trs_p0_to_cam.normalized(), &camera.forward) - 1.0).abs()));
	// buf.write_debug(&format!(" trs_p1  {:?} dot {:.6}\n", trs_p1, (Vec3::dot_product(&from_trs_p1_to_cam.normalized(), &camera.forward) - 1.0).abs()));
	// buf.write_debug(&format!(" cam pos {:?} completely off? {} \n", camera.position, is_completely_off_camera));
	if is_completely_off_camera { return None }

	let p0_clip = p0.get_transformed_by_mat4x4_w(&buf.render_mat);
	let p1_clip = p1.get_transformed_by_mat4x4_w(&buf.render_mat);

	// buf.write_debug(&format!(" homog {:?}, scr space {:?} \n", &p0_clip.homogeneous_cpy(), clip_space_to_screen_space(&p0_clip.homogeneous_cpy(), buf.wid, buf.hei)));
	// buf.write_debug(&format!(" homog {:?}, scr space {:?} \n", &p1_clip.homogeneous_cpy(), clip_space_to_screen_space(&p1_clip.homogeneous_cpy(), buf.wid, buf.hei)));
	// the triangle is inside the screen, don't cull
	// buf.write_debug(&format!("p0 {:?} in w ran {} \n", p0_clip, p0_clip.in_w_range()));
	if p0_clip.in_w_range() { return Some(Line::from_clip_space(p0_clip, p1_clip, buf.wid, buf.hei)) }
	// buf.write_debug(&format!("p1 {:?} in w ran {} \n", p1_clip, p1_clip.in_w_range()));
	if p1_clip.in_w_range() { return Some(Line::from_clip_space(p0_clip, p1_clip, buf.wid, buf.hei)) }

	let screen_p0 = clip_space_to_screen_space_f(&p0_clip.homogeneous(), buf.wid, buf.hei);
	let screen_p1 = clip_space_to_screen_space_f(&p1_clip.homogeneous(), buf.wid, buf.hei);

	let (width, height) = (buf.wid as f32, buf.hei as f32);

	// buf.write_debug(&format!("p0..p1 inters scr {}\n", line_intersect_screen(&screen_p0, &screen_p1, width, height)));
	if line_intersect_screen(&screen_p0, &screen_p1, width, height) { return Some(Line::from_screen_points(screen_p0, screen_p1)) }

	None
}

pub fn cull_circle(pos: &FVec2, x_rad: f32, buf: &mut TerminalBuffer) -> bool {
	let (wid, hei) = (buf.wid as f32, buf.hei as f32);
	
	let (last_x, last_y) = (wid - 1.0, hei - 1.0);

	let closest_point = if pos.x >= wid {
		// buf.write_debug(&format!("to the right"));
		FVec2::new(last_x, pos.y.clamp(0.0, last_y))
	} else if pos.x < 0.0 {
		// buf.write_debug(&format!("to the left"));
		FVec2::new(0.0, pos.y.clamp(0.0, last_y))
	} else if pos.y >= hei {
		// buf.write_debug(&format!("downwards"));
		FVec2::new(pos.x, last_y)
	} else if pos.y < 0.0 {
		// buf.write_debug(&format!("upwards"));
		FVec2::new(pos.x, 0.0)
	} else {
		return false;
	};


	// X scale is double Y scale
	let mut vec_pos_to_closest = pos - &closest_point;
	vec_pos_to_closest.scale_y(2.0);

	let sq_magnitude = vec_pos_to_closest.squared_magnitude();
	let sq_x_rad = x_rad * x_rad;

	// render_char('&', &(&closest_point).into(), buf);
	// buf.write_debug(&format!("\npos to closest {:?}\nsq mag {:.4}, sq rad {:.2}\n(rad {:.4})\nCULL? {} \n\n", vec_pos_to_closest, sq_magnitude, sq_x_rad, x_rad, sq_magnitude >= sq_x_rad));

	sq_magnitude >= sq_x_rad
}