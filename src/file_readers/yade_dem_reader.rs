use std::{fs, process};

use crate::{maths::Vec3};


pub struct YadeDemData {
	pub tris:  Vec<Tri>,
	pub circs: Vec<Circ>,
}

type Float = f32;

pub struct Circ {
	pub pos: Vec3,

	pub rad: Float,
}

pub struct Tri {
	pub pos: Vec3,
	
	pub p0: Vec3,
	pub p1: Vec3,
	pub p2: Vec3,
}

impl YadeDemData {
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
	
		for (_, line) in file_content.lines().enumerate() {
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
	
				circs.push(Circ { pos: Vec3 { x, y, z }, rad });
	
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
	
				tris.push(Tri {
					pos: Vec3 { x, y, z },
					p0:  Vec3 { x: p0x, y: p0y, z: p0z },
					p1:  Vec3 { x: p1x, y: p1y, z: p1z },
					p2:  Vec3 { x: p2x, y: p2y, z: p2z }
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
			circs,
			tris,
		}
	
	}

}



