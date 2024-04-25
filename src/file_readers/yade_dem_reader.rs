use std::{fmt::Display, fs, process};

use crate::maths::*;


pub struct YadeDemData {
	pub tris:  Vec<Tri>,
	pub balls: Vec<Circ>,
}

type Float = f32;

#[derive(Clone)]
pub struct Circ {
	pub pos: Vec3,

	pub rad: Float,
}

impl Display for Circ {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({:+.6}) at [{:+.6}, {:+.6}, {:+.6}]", self.rad, self.pos.x, self.pos.y, self.pos.z)
	}
}

pub struct Tri {
	pub pos: Vec3,

	pub p0: Vec3,
	pub p1: Vec3,
	pub p2: Vec3,
}

impl YadeDemData {
	// TODO: read_from_file -> Result

	pub fn read_from_file_or_quit(path: &str) -> Self {
		// println!("Reading file '{}'", path);

		// let file_content = fs::read_to_string(path).map_err(|err| err.to_string())?;
		let file_content = match fs::read_to_string(path) {
			Ok(content) => content,
			Err(error) => {
				println!("IO error: {}", error);
				process::exit(1)
			}
		};

		let mut circs = vec![];
		let mut tris  = vec![];

		for (line_index, line) in file_content.lines().enumerate() {
			// println!(" {}: '{}'", i, line);
			let mut line_split = line.split(", ").skip(1);

			if line.starts_with('0') {

				// TODO: port to juicier Rust
				// let [ x, y, z, rad ] = line_split.next_chunk().unwrap();

				// TODO: function that maps iter into x y z rad
				let   x = line_split.next().unwrap().parse::<Float>().unwrap();
				let   y = line_split.next().unwrap().parse::<Float>().unwrap();
				let   z = line_split.next().unwrap().parse::<Float>().unwrap();
				let rad = line_split.next().unwrap().parse::<Float>().unwrap();

				circs.push(Circ { pos: Vec3 { x: x, y: z, z: y }, rad });

				continue;
			}

			if line.starts_with('1') {

				// TODO: port to juicier Rust
				// let [ x, y, z, p0x, p0y, p0z, p1x, p1y, p1z, p2x, p2y, p2z ] = line_split.next_chunk().unwrap();

				let   x = line_split.next().unwrap().parse::<Float>().unwrap();
				let   y = line_split.next().unwrap().parse::<Float>().unwrap();
				let   z = line_split.next().unwrap().parse::<Float>().unwrap();
				let p0x = line_split.next().unwrap().parse::<Float>().unwrap();
				let p0y = line_split.next().unwrap().parse::<Float>().unwrap();
				let p0z = line_split.next().unwrap().parse::<Float>().unwrap();
				let p1x = line_split.next().unwrap().parse::<Float>().unwrap();
				let p1y = line_split.next().unwrap().parse::<Float>().unwrap();
				let p1z = line_split.next().unwrap().parse::<Float>().unwrap();
				let p2x = line_split.next().unwrap().parse::<Float>().unwrap();
				let p2y = line_split.next().unwrap().parse::<Float>().unwrap();
				let p2z = line_split.next().unwrap().parse::<Float>().unwrap();

				// input coordinate system is XYZ, converts to XZY
				tris.push(Tri {
					pos: Vec3 { x:   x, y:   z, z:   y },
					p0:  Vec3 { x: p0x, y: p0z, z: p0y },
					p1:  Vec3 { x: p1x, y: p1z, z: p1y },
					p2:  Vec3 { x: p2x, y: p2z, z: p2y }
				});

				continue;
			}
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
			balls: circs,
			tris,
		}

	}

}



