use std::{fs::{File, self}, io::Read, num::{ParseFloatError, ParseIntError}, fmt::Display};

use crate::rendering::mesh::Mesh;


pub enum ReaderError {
	BadFormat,
	FileNotFound,
	IOError,
}

impl Display for ReaderError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		
		let response = match self {
			ReaderError::IOError      => "IO error",
			ReaderError::BadFormat    => "Bad format error",
			ReaderError::FileNotFound => "Not found error",
		};

		f.write_str(response).unwrap();
		Ok(())
	}
}

impl From<std::io::Error> for ReaderError {
	fn from(io_error: std::io::Error) -> Self {
		match io_error.kind() {
			std::io::ErrorKind::NotFound => Self::FileNotFound,
			_ => Self::IOError,
		}
	}
}

impl From<ParseFloatError> for ReaderError {
	fn from(_: ParseFloatError) -> Self {
		Self::BadFormat
	}
}

impl From<ParseIntError> for ReaderError {
	fn from(_: ParseIntError) -> Self {
		Self::BadFormat
	}
}

// TODO: instead of propagating the shit all the way, I can make ReaderError have a string that expands with the error (shows lines that went bad)
pub fn read_mesh_from_obj(path: &str) -> Result<Mesh, ReaderError> {
	let file_content = fs::read_to_string(path)?;
	let mut verts = vec![];
	let mut tris  = vec![];
	
	for line in file_content.lines() {
		if line.starts_with('v') {
			let line_split_by_space = line.split(' ').skip(1);

			for vert_str in line_split_by_space {
				let vert = vert_str.parse::<f32>()?;
				verts.push(vert);
			}

			continue;
		}

		// TODO: handle Ngons
		//https://stackoverflow.com/questions/60660726/how-can-i-load-and-render-an-obj-file-that-may-include-triangles-quads-or-n-go
		if line.starts_with('f') {
			let line_split_by_space = line.split(' ').skip(1);
			
			for indices_group in line_split_by_space {
				// println!("{}", value);

				// TODO:
				// f v1/vt1 v2/vt2 v3/vt3
				// face references (vertex_index/texture_index/normal_index)
				let mut indices_of_tri = indices_group.split('/');
				let vertex_index = indices_of_tri.next().ok_or(ReaderError::BadFormat)?;
				let vertex_index = vertex_index.parse::<u16>()? - 1;
				tris.push(vertex_index);

			}
		}
	}

	let mesh = Mesh {
		verts,
		tris,
	};

	Ok(mesh)
}