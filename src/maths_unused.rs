
fn slope_of_line(p0: &UVec2, p1: &UVec2) -> f32 {
	(p1.y as f32 - p0.y as f32) / (p1.x as f32 - p0.x as f32)
}


fn transpose_3x3(vec: &mut [f32]) {
	let mut temp: f32;
	
	const SZ: usize = 3;

	// TODO: xy_to_i
	let x = 1; let y = 0;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;

	let x = 2; let y = 0;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;

	let x = 2; let y = 1;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;
}

fn transpose_3x3_area_in_4x4(vec: &mut [f32]) {
	let mut temp: f32;
	
	const SZ: usize = 4;

	// TODO: xy_to_i
	let x = 1; let y = 0;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;

	let x = 2; let y = 0;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;

	let x = 2; let y = 1;
	temp = vec[y * SZ + x];
	vec[y * SZ + x] = vec[x * SZ + 0];
	vec[y * SZ + x] = temp;
}

pub fn create_view_matrix(position: &Vec3, rotation: &Vec3) -> Vec<f32> {

	let cos_x = rotation.x.cos();
	let sin_x = rotation.x.sin();

	let cos_y = rotation.y.cos();
	let sin_y = rotation.y.sin();

	let cos_z = rotation.z.cos();
	let sin_z = rotation.z.sin();
	
	let cam_up = Vec3::new(
		cos_x * sin_z + sin_x * sin_y * cos_z,
		cos_x * cos_z - sin_x * sin_y * sin_z,
		-sin_x * cos_y
	);

	let cam_forward = Vec3::new(
		sin_x * sin_z - cos_x * sin_y * cos_z,
		sin_x * cos_z + cos_x * sin_y * sin_z,
		cos_x * cos_y,
	);

	let cam_side = normalize(&cross_product(&cam_forward, &cam_up));
	let cam_up = cross_product(&cam_side, &cam_forward);

	let mut view_matrix = build_identity_4x4();

	const SZ: u16 = 4;

	view_matrix[xy_to_it(0, 0, SZ)] = cam_side.x;
	view_matrix[xy_to_it(1, 0, SZ)] = cam_side.y;
	view_matrix[xy_to_it(2, 0, SZ)] = cam_side.z;

	view_matrix[xy_to_it(0, 1, SZ)] = cam_up.x;
	view_matrix[xy_to_it(1, 1, SZ)] = cam_up.y;
	view_matrix[xy_to_it(2, 1, SZ)] = cam_up.z;

	view_matrix[xy_to_it(0, 2, SZ)] = -cam_forward.x;
	view_matrix[xy_to_it(1, 2, SZ)] = -cam_forward.y;
	view_matrix[xy_to_it(2, 2, SZ)] = -cam_forward.z;

	view_matrix[xy_to_it(3, 0, SZ)] = -dot_product(&cam_side, &position);
	view_matrix[xy_to_it(3, 1, SZ)] = -dot_product(&cam_up, &position);
	view_matrix[xy_to_it(3, 2, SZ)] = dot_product(&cam_forward, &position);
	view_matrix[xy_to_it(3, 3, SZ)] = 1.0;

	view_matrix
}

fn apply_rotation_to_mat_4x4_alloc(mat: &mut [f32], angle_x: f32, angle_y: f32, angle_z: f32) {
	let mat2 = build_rot_mat_xyz_4x4(angle_x, angle_y, angle_z);
	multiply_4x4_matrices(mat, &mat2);
}

pub fn build_rot_mat_x_3x3(angle: f32) -> Vec<f32> {
	let cos = angle.cos();
	let sin = angle.sin();

	vec![
		1.0,  0.0,  0.0,
		0.0,  cos, -sin,
		0.0,  sin,  cos,
	]
}

pub fn build_rot_mat_y_3x3(angle: f32) -> Vec<f32> {
	let cos = angle.cos();
	let sin = angle.sin();

	vec![
		 cos,  0.0,  sin,
		 0.0,  1.0,  0.0,
		-sin,  0.0,  cos,
	]
}

pub fn build_rot_mat_z_3x3(angle: f32) -> Vec<f32> {
	let cos = angle.cos();
	let sin = angle.sin();

	vec![
		cos, -sin,  0.0,
		sin,  cos,  0.0,
		0.0,  0.0,  1.0,
	]
}

pub fn build_rot_mat_xyz_3x3(angle_x: f32, angle_y: f32, angle_z: f32) -> Vec<f32> {
	let cos_x = angle_x.cos();
	let sin_x = angle_x.sin();

	let cos_y = angle_y.cos();
	let sin_y = angle_y.sin();

	let cos_z = angle_z.cos();
	let sin_z = angle_z.sin();

	let sin_y_cos_z = sin_y * cos_z;
	
	vec![
		cos_y * cos_z,                       -cos_y * sin_z,                          sin_y,
		cos_x * sin_z + sin_x * sin_y_cos_z,  cos_x * cos_z - sin_x * sin_y * sin_z, -sin_x * cos_y,
		sin_x * sin_z - cos_x * sin_y_cos_z,  sin_x * cos_z + cos_x * sin_y * sin_z,  cos_x * cos_y,
	]
}

pub fn build_scale_mat_3x3(sc_x: f32, sc_y: f32, sc_z: f32) -> Vec<f32> {
	vec![
		sc_x,  0.0,  0.0,
		 0.0, sc_y,  0.0,
		 0.0,  0.0, sc_z,
	]
}

pub fn build_pos_mat4x4(pos_x: f32, pos_y: f32, pos_z: f32) -> Vec<f32> {
	vec![
		1.0, 0.0, 0.0, pos_x,
		0.0, 1.0, 0.0, pos_y,
		0.0, 0.0, 1.0, pos_z,
		0.0, 0.0, 0.0, 1.0,
		
		// transposed
		// 1.0, 0.0, 0.0, 0.0,
		// 0.0, 1.0, 0.0, 0.0,
		// 0.0, 0.0, 1.0, 0.0,
		// pos_x, pos_y, pos_z, 1.0,
	]
}

fn build_rot_mat_xyz_4x4(angle_x: f32, angle_y: f32, angle_z: f32) -> Vec<f32> {
	let cos_x = angle_x.cos();
	let sin_x = angle_x.sin();

	let cos_y = angle_y.cos();
	let sin_y = angle_y.sin();

	let cos_z = angle_z.cos();
	let sin_z = angle_z.sin();

	vec![
		cos_y * cos_z,
		-cos_y * sin_z,
		sin_y,
		0.0,

		cos_x * sin_z + sin_x * sin_y * cos_z,
		cos_x * cos_z - sin_x * sin_y * sin_z,
		-sin_x * cos_y,
		0.0,

		sin_x * sin_z - cos_x * sin_y * cos_z,
		sin_x * cos_z + cos_x * sin_y * sin_z,
		cos_x * cos_y,
		0.0,

		0.0,
		0.0,
		0.0,
		1.0
	]

	// or
	// vec![
	// 	cos_y * cos_z,                      -cos_y * sin_z,                          sin_y,         0.0,
	// 	cos_x * sin_z + sin_x * sin_y_cos_z,  cos_x * cos_z - sin_x * sin_y * sin_z, -sin_x * cos_y, 0.0,
	// 	sin_x * sin_z - cos_x * sin_y_cos_z,  sin_x * cos_z + cos_x * sin_y * sin_z,  cos_x * cos_y, 0.0,
	// 	0.0,                                  0.0,                                    0.0,           1.0
	// ]
}


// only allocates an array on the stack
fn multiply_4x4_matrices_alloc(mat: &mut [f32], mat2: &[f32]) {
	let mut result = [0.0; 16];

	const SZ: usize = 4;

	for i in 0..SZ {
		for j in 0..SZ {
			for k in 0..SZ {
				result[i * SZ + j] += mat[i * SZ + k] * mat2[k * SZ + j];
			}
		}
	}

	const SZ_SQ: usize = SZ * SZ;
	mat[..SZ_SQ].copy_from_slice(&result[..SZ_SQ]);

	// same as
	// for i in 0..sz*sz {
	// 	mat[i] = result[i];
	// }
}

pub fn apply_pos_vec_to_mat_4x4(mat: &mut [f32], vec: &Vec3) {
	const SZ: usize = 4;
	mat[0 * SZ + 3] = vec.x;
	mat[1 * SZ + 3] = vec.y;
	mat[2 * SZ + 3] = vec.z;
}

fn apply_mat_generic(mat: &mut [f32]) {
	
	// build any mat_4x4 here to test
	let mat2 = Vec::<f32>::new();
	
	const SZ: usize = 4;

	// TODO: xy_to_i
	let x0_y0 = mat[0 * SZ + 0] * mat2[0 * SZ + 0]  +  mat[0 * SZ + 1] * mat2[1 * SZ + 0]  +  mat[0 * SZ + 2] * mat2[2 * SZ + 0]  +  mat[0 * SZ + 3] * mat2[3 * SZ + 0];
	let x0_y1 = mat[0 * SZ + 0] * mat2[0 * SZ + 1]  +  mat[0 * SZ + 1] * mat2[1 * SZ + 1]  +  mat[0 * SZ + 2] * mat2[2 * SZ + 1]  +  mat[0 * SZ + 3] * mat2[3 * SZ + 1];
	let x0_y2 = mat[0 * SZ + 0] * mat2[0 * SZ + 2]  +  mat[0 * SZ + 1] * mat2[1 * SZ + 2]  +  mat[0 * SZ + 2] * mat2[2 * SZ + 2]  +  mat[0 * SZ + 3] * mat2[3 * SZ + 2];
	let x0_y3 = mat[0 * SZ + 0] * mat2[0 * SZ + 3]  +  mat[0 * SZ + 1] * mat2[1 * SZ + 3]  +  mat[0 * SZ + 2] * mat2[2 * SZ + 3]  +  mat[0 * SZ + 3] * mat2[3 * SZ + 3];

	let x1_y0 = mat[1 * SZ + 0] * mat2[0 * SZ + 0]  +  mat[1 * SZ + 1] * mat2[1 * SZ + 0]  +  mat[1 * SZ + 2] * mat2[2 * SZ + 0]  +  mat[1 * SZ + 3] * mat2[3 * SZ + 0];
	let x1_y1 = mat[1 * SZ + 0] * mat2[0 * SZ + 1]  +  mat[1 * SZ + 1] * mat2[1 * SZ + 1]  +  mat[1 * SZ + 2] * mat2[2 * SZ + 1]  +  mat[1 * SZ + 3] * mat2[3 * SZ + 1];
	let x1_y2 = mat[1 * SZ + 0] * mat2[0 * SZ + 2]  +  mat[1 * SZ + 1] * mat2[1 * SZ + 2]  +  mat[1 * SZ + 2] * mat2[2 * SZ + 2]  +  mat[1 * SZ + 3] * mat2[3 * SZ + 2];
	let x1_y3 = mat[1 * SZ + 0] * mat2[0 * SZ + 3]  +  mat[1 * SZ + 1] * mat2[1 * SZ + 3]  +  mat[1 * SZ + 2] * mat2[2 * SZ + 3]  +  mat[1 * SZ + 3] * mat2[3 * SZ + 3];

	let x2_y0 = mat[2 * SZ + 0] * mat2[0 * SZ + 0]  +  mat[2 * SZ + 1] * mat2[1 * SZ + 0]  +  mat[2 * SZ + 2] * mat2[2 * SZ + 0]  +  mat[2 * SZ + 3] * mat2[3 * SZ + 0];
	let x2_y1 = mat[2 * SZ + 0] * mat2[0 * SZ + 1]  +  mat[2 * SZ + 1] * mat2[1 * SZ + 1]  +  mat[2 * SZ + 2] * mat2[2 * SZ + 1]  +  mat[2 * SZ + 3] * mat2[3 * SZ + 1];
	let x2_y2 = mat[2 * SZ + 0] * mat2[0 * SZ + 2]  +  mat[2 * SZ + 1] * mat2[1 * SZ + 2]  +  mat[2 * SZ + 2] * mat2[2 * SZ + 2]  +  mat[2 * SZ + 3] * mat2[3 * SZ + 2];
	let x2_y3 = mat[2 * SZ + 0] * mat2[0 * SZ + 3]  +  mat[2 * SZ + 1] * mat2[1 * SZ + 3]  +  mat[2 * SZ + 2] * mat2[2 * SZ + 3]  +  mat[2 * SZ + 3] * mat2[3 * SZ + 3];

	let x3_y0 = mat[3 * SZ + 0] * mat2[0 * SZ + 0]  +  mat[3 * SZ + 1] * mat2[1 * SZ + 0]  +  mat[3 * SZ + 2] * mat2[2 * SZ + 0]  +  mat[3 * SZ + 3] * mat2[3 * SZ + 0];
	let x3_y1 = mat[3 * SZ + 0] * mat2[0 * SZ + 1]  +  mat[3 * SZ + 1] * mat2[1 * SZ + 1]  +  mat[3 * SZ + 2] * mat2[2 * SZ + 1]  +  mat[3 * SZ + 3] * mat2[3 * SZ + 1];
	let x3_y2 = mat[3 * SZ + 0] * mat2[0 * SZ + 2]  +  mat[3 * SZ + 1] * mat2[1 * SZ + 2]  +  mat[3 * SZ + 2] * mat2[2 * SZ + 2]  +  mat[3 * SZ + 3] * mat2[3 * SZ + 2];
	let x3_y3 = mat[3 * SZ + 0] * mat2[0 * SZ + 3]  +  mat[3 * SZ + 1] * mat2[1 * SZ + 3]  +  mat[3 * SZ + 2] * mat2[2 * SZ + 3]  +  mat[3 * SZ + 3] * mat2[3 * SZ + 3];

	mat[0 * SZ + 0] = x0_y0;
	mat[0 * SZ + 1] = x0_y1;
	mat[0 * SZ + 2] = x0_y2;
	mat[0 * SZ + 3] = x0_y3;

	mat[1 * SZ + 0] = x1_y0;
	mat[1 * SZ + 1] = x1_y1;
	mat[1 * SZ + 2] = x1_y2;
	mat[1 * SZ + 3] = x1_y3;

	mat[2 * SZ + 0] = x2_y0;
	mat[2 * SZ + 1] = x2_y1;
	mat[2 * SZ + 2] = x2_y2;
	mat[2 * SZ + 3] = x2_y3;

	mat[3 * SZ + 0] = x3_y0;
	mat[3 * SZ + 1] = x3_y1;
	mat[3 * SZ + 2] = x3_y2;
	mat[3 * SZ + 3] = x3_y3;

	mat[0 * SZ + 0] = x0_y0;
	mat[0 * SZ + 1] = x0_y1;
	mat[0 * SZ + 2] = x0_y2;
	mat[0 * SZ + 3] = x0_y3;

	mat[1 * SZ + 0] = x1_y0;
	mat[1 * SZ + 1] = x1_y1;
	mat[1 * SZ + 2] = x1_y2;
	mat[1 * SZ + 3] = x1_y3;

	mat[2 * SZ + 0] = x2_y0;
	mat[2 * SZ + 1] = x2_y1;
	mat[2 * SZ + 2] = x2_y2;
	mat[2 * SZ + 3] = x2_y3;

	mat[3 * SZ + 0] = x3_y0;
	mat[3 * SZ + 1] = x3_y1;
	mat[3 * SZ + 2] = x3_y2;
	mat[3 * SZ + 3] = x3_y3;
}
