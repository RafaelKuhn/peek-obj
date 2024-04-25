

// TODO: format this more decently
pub fn fmt_mat4x4(mat: &Vec<f32>) -> String {
	format!("\n[{} {} {} {}]\n[{} {} {} {}]\n[{} {} {} {}]\n[{} {} {} {}]", 
		fmt_f(mat[ 0]), fmt_f(mat[ 1]), fmt_f(mat[ 2]), fmt_f(mat[ 3]),
		fmt_f(mat[ 4]), fmt_f(mat[ 5]), fmt_f(mat[ 6]), fmt_f(mat[ 7]),
		fmt_f(mat[ 8]), fmt_f(mat[ 9]), fmt_f(mat[10]), fmt_f(mat[11]),
		fmt_f(mat[12]), fmt_f(mat[13]), fmt_f(mat[14]), fmt_f(mat[15]),
	)
}

pub fn fmt_mat4_line(x: f32, y: f32, z: f32, w: f32) -> String {
	format!("[{} {} {} {}]", fmt_f(x), fmt_f(y), fmt_f(z), fmt_f(w))
}

pub fn fmt_f(f: f32) -> String {
	// ' >6' means pad the string with spaces ' ' until its length is 6
	// '+.2' means pad the float with two decimals and force + sign on positive
	format!("{:>6}", format!("{:+.2}", f))
}
