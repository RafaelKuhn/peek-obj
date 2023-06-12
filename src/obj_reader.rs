use std::{num::{ParseFloatError, ParseIntError}, fmt::Display, fs};

use crate::{rendering::mesh::Mesh, maths::Vec3};


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

	let mut verts   = vec![];
	let mut tris    = vec![];
	let mut normals = vec![];
	let mut normal_indices = vec![];

	let mut has_found_normals = false;

	for line in file_content.lines() {
		let line_split_by_space = line.split(' ').skip(1);
		
		if line.starts_with("v ") {
			for vert_str in line_split_by_space {
				let vert = vert_str.parse::<f32>()?;
				verts.push(vert);
			}

			continue;
		}

		if line.starts_with("vn ") {
			for normal_str in line_split_by_space {
				let normal = normal_str.parse::<f32>()?;
				normals.push(normal);
			}

			has_found_normals = true;
			continue;
		}

		// TODO: handle Ngons
		// https://stackoverflow.com/questions/60660726/how-can-i-load-and-render-an-obj-file-that-may-include-triangles-quads-or-n-go
		if line.starts_with("f ") {
			for indices_group in line_split_by_space {
				// println!("{}", value);

				// f v1/vt1 v2/vt2 v3/vt3
				// face references (vertex_index/texture_index/normal_index)
				let mut indices = indices_group.split('/');

				let vertex_index = indices.next().ok_or(ReaderError::BadFormat)?;
				let vertex_index = vertex_index.parse::<u16>()? - 1;
				tris.push(vertex_index);

				// skip texture coordinates
				indices.next();

				if has_found_normals {
					let normal_index = indices.next().ok_or(ReaderError::BadFormat)?;
					let normal_index = normal_index.parse::<u16>()? - 1;
					normal_indices.push(normal_index);		
				}
			}

			continue;
		}

		if !has_found_normals {
			// TODO: calculate normals manually
		}
	}

	let mesh = Mesh {
		verts: verts,
		tris_indices: tris,
		normals: normals,
		normal_indices: normal_indices,
	};

	Ok(mesh)
}

pub fn translate_mesh(mesh_res: Result<Mesh, ReaderError>, translation: &Vec3) -> Result<Mesh, ReaderError> {
	let mut mesh = mesh_res?;
	let mesh_mut = &mut mesh;

	let mut i = 0;
	while i < mesh_mut.verts.len() {
		
		let vert_x = mesh_mut.verts[i];
		mesh_mut.verts[i] = vert_x + translation.x;
		i += 1;

		let vert_y = mesh_mut.verts[i];
		mesh_mut.verts[i] = -(vert_y + translation.y);
		i += 1;
		
		let vert_z = mesh_mut.verts[i];
		mesh_mut.verts[i] = vert_z + translation.z;
		i += 1;
	}

	Ok(mesh)
}

