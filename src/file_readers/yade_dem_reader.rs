use std::{fmt::Display, fs, process};

use crate::maths::*;

const YADE_SCALE: f32 = 15.0;


pub struct YadeDemData {
	pub tris:  Vec<Tri>,
	pub balls: Vec<Ball>,
}


#[derive(Debug, Clone)]
pub struct Ball {
	pub pos: Vec3,

	pub rad: Float,
}

impl Display for Ball {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({:+.6}) at [{:+.6}, {:+.6}, {:+.6}]", self.rad, self.pos.x, self.pos.y, self.pos.z)
	}
}

#[derive(Debug)]
pub struct Tri {
	pub p0: Vec3,
	pub p1: Vec3,
	pub p2: Vec3,
}

impl Tri {
	fn with_pos(p0: Vec3, p1: Vec3, p2: Vec3) -> Tri {
		Tri { p0, p1, p2 }
	}
}

impl YadeDemData {

	pub fn debug() -> YadeDemData {
		let mut balls = vec![];

		let mut tris = Vec::<Tri>::with_capacity(12);

		let v0 = Vec3::new( 0.06, -0.06, -0.06);
		let v1 = Vec3::new( 0.06, -0.06,  0.06);
		let v2 = Vec3::new(-0.06, -0.06,  0.06);
		let v3 = Vec3::new(-0.06, -0.06, -0.06);
		let v4 = Vec3::new( 0.06,  0.06, -0.06);
		let v5 = Vec3::new( 0.06,  0.06,  0.06);
		let v6 = Vec3::new(-0.06,  0.06,  0.06);
		let v7 = Vec3::new(-0.06,  0.06, -0.06);

		tris.push(Tri::with_pos(v1, v2, v3));
		tris.push(Tri::with_pos(v7, v6, v5));
		tris.push(Tri::with_pos(v4, v5, v1));
		tris.push(Tri::with_pos(v5, v6, v2));
		tris.push(Tri::with_pos(v2, v6, v7));
		tris.push(Tri::with_pos(v0, v3, v7));
		tris.push(Tri::with_pos(v0, v1, v3));
		tris.push(Tri::with_pos(v4, v7, v5));
		tris.push(Tri::with_pos(v0, v4, v1));
		tris.push(Tri::with_pos(v1, v5, v2));
		tris.push(Tri::with_pos(v3, v2, v7));
		tris.push(Tri::with_pos(v4, v0, v7));

		balls.push(Ball { pos: Vec3 { x:  0.00, y:  0.00, z:  0.00 }, rad: 0.01 });
		balls.push(Ball { pos: Vec3 { x:  0.05, y:  0.00, z:  0.00 }, rad: 0.01 });
		balls.push(Ball { pos: Vec3 { x:  0.00, y:  0.05, z:  0.00 }, rad: 0.01 });
		balls.push(Ball { pos: Vec3 { x:  0.00, y:  0.00, z:  0.05 }, rad: 0.01 });
		balls.push(Ball { pos: Vec3 { x: -0.05, y:  0.00, z:  0.00 }, rad: 0.01 });
		balls.push(Ball { pos: Vec3 { x:  0.00, y: -0.05, z:  0.00 }, rad: 0.01 });
		balls.push(Ball { pos: Vec3 { x:  0.00, y:  0.00, z: -0.05 }, rad: 0.01 });

		Self {
			balls,
			tris,
		}
	}

	pub fn read_from_file_or_quit(path: &str) -> YadeDemData {
		// println!("Reading file '{}'", path);

		// let file_content = fs::read_to_string(path).map_err(|err| err.to_string())?;
		let file_content = match fs::read_to_string(path) {
			Ok(content) => content,
			Err(error) => {
				eprintln!("IO error: {}", error);
				process::exit(1)
			}
		};

		let mut balls = vec![];
		let mut tris  = vec![];

		for (line_index, line) in file_content.lines().enumerate() {
			let line_num = line_index + 1;
			// println!(" {}: '{}'", i, line);

			let line = line.trim();
			let mut line_split = line.split(',').skip(1);

			let is_sphere = line.starts_with('0');
			if is_sphere {

				// TODO: port to juicier Rust
				// let [ x, y, z, rad ] = line_split.next_chunk().unwrap();

				let   x = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let   y = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let   z = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let rad = get_next_float_in_line_or_quit(&mut line_split, path, line_num);

				// input coordinate system is XYZ, converts to XZY
				let ball = Ball {
					pos: Vec3 { x: x, y: z, z: y }.scale(YADE_SCALE),
					rad: rad * YADE_SCALE
				};
				// println!(" got 0: {:?} ", ball);

				balls.push(ball);
				continue;
			}

			let is_triangle = line.starts_with('1');
			if is_triangle {

				// TODO: port to juicier Rust
				// let [ x, y, z, p0x, p0y, p0z, p1x, p1y, p1z, p2x, p2y, p2z ] = line_split.next_chunk().unwrap();

				let   x = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let   y = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let   z = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let p0x = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let p0y = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let p0z = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let p1x = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let p1y = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let p1z = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let p2x = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let p2y = get_next_float_in_line_or_quit(&mut line_split, path, line_num);
				let p2z = get_next_float_in_line_or_quit(&mut line_split, path, line_num);

				// input coordinate system is XYZ, converts to XZY
				let pos = Vec3 { x: x, y: z, z: y }.scale(YADE_SCALE);
				let tri = Tri {
					p0: Vec3 { x: p0x, y: p0z, z: p0y }.scale(YADE_SCALE).add_vec(&pos),
					p1: Vec3 { x: p1x, y: p1z, z: p1y }.scale(YADE_SCALE).add_vec(&pos),
					p2: Vec3 { x: p2x, y: p2z, z: p2y }.scale(YADE_SCALE).add_vec(&pos),
				};
				// println!(" got 1: {:?} ", tri);

				// input coordinate system is XYZ, converts to XZY
				tris.push(tri);

				continue;
			}

			if line.is_empty() { continue; }

			let is_comment = line.starts_with('#') || line.starts_with("//");
			if is_comment { continue; }

			eprintln!("line should ")
		}

		// #if VERBOSE
		// println!();
		// for (i, tri) in read_yade_dem.tris.into_iter().enumerate() {
		// 	println!("TRIANGLE {:3}: {:+.4} {:+.4} {:+.4}  ({:+.3} {:+.3} {:+.3})  ({:+.3} {:+.3} {:+.3})  ({:+.3} {:+.3} {:+.3})", i, tri.x, tri.y, tri.z,
		// 	tri.p0x, tri.p0y, tri.p0z, tri.p1x, tri.p1y, tri.p1z, tri.p2x, tri.p2y, tri.p2z );
		// }

		// println!();
		// for (i, circ) in read_yade_dem.circs.into_iter().enumerate() {
		// 	println!("CIRCLE {:3}: {:+.4} {:+.4} {:+.4} rad {:+.4}", i, circ.x, circ.y, circ.z, circ.rad);
		// }
		// #endif

		Self {
			balls,
			tris,
		}
	}

	pub fn print_mock() {
		let v0 = Vec3::new( 0.06, -0.06, -0.06);
		let v1 = Vec3::new( 0.06, -0.06,  0.06);
		let v2 = Vec3::new(-0.06, -0.06,  0.06);
		let v3 = Vec3::new(-0.06, -0.06, -0.06);
		let v4 = Vec3::new( 0.06,  0.06, -0.06);
		let v5 = Vec3::new( 0.06,  0.06,  0.06);
		let v6 = Vec3::new(-0.06,  0.06,  0.06);
		let v7 = Vec3::new(-0.06,  0.06, -0.06);

		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v1, v2, v3);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v7, v6, v5);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v4, v5, v1);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v5, v6, v2);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v2, v6, v7);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v0, v3, v7);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v0, v1, v3);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v4, v7, v5);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v0, v4, v1);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v1, v5, v2);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v3, v2, v7);
		println!("1, {:?}, {:?}, {:?}, {:?}", Vec3::zero(), v4, v0, v7);

		println!("0, {:?}, {}", Vec3 { x:  0.00, y:  0.00, z:  0.00 }, 0.01 );
		println!("0, {:?}, {}", Vec3 { x:  0.05, y:  0.00, z:  0.00 }, 0.01 );
		println!("0, {:?}, {}", Vec3 { x:  0.00, y:  0.05, z:  0.00 }, 0.01 );
		println!("0, {:?}, {}", Vec3 { x:  0.00, y:  0.00, z:  0.05 }, 0.01 );
		println!("0, {:?}, {}", Vec3 { x: -0.05, y:  0.00, z:  0.00 }, 0.01 );
		println!("0, {:?}, {}", Vec3 { x:  0.00, y: -0.05, z:  0.00 }, 0.01 );
		println!("0, {:?}, {}", Vec3 { x:  0.00, y:  0.00, z: -0.05 }, 0.01 );
	}

}

fn get_next_float_in_line_or_quit<'a>(line_iter: &mut impl Iterator<Item = &'a str>, path: &str, line_num: usize) -> Float {

	let next_str = match line_iter.next() {
		None => quit_with(&format!("Not enough coordinates at line {line_num}"), path),
		Some(slice) => slice,
	};

	let trimmed_str = next_str.trim();
	if trimmed_str.is_empty() {
		quit_with(&format!("Empty string slice, should have a value, line: {line_num}"), path);
	}

	match trimmed_str.parse() {
		Err(err) => quit_with(&format!("Could not parse a float from string slice: '{trimmed_str}', line: {line_num}\nerr: {err}"), path),
		Ok(float) => float,
	}
}

// OR with generics:
// fn next_float_or_quit<'a, T>(it: &mut T, line_num: usize) -> Float where T: Iterator<Item = &'a str> { 0.0 }

fn quit_with(message: &str, path: &str) -> ! {
	eprintln!("Error reading '{path}'");
	eprintln!("{}", message);
	std::process::exit(1);
}