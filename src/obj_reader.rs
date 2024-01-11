use std::{num::{ParseFloatError, ParseIntError}, fmt::Display, fs, io::Error};

use crate::{rendering::mesh::Mesh, maths::Vec3};


// pub enum ReaderError<'a> {
pub enum ReaderError {
	// BadFormat(&'a str),
	BadFormat(String),
	// FileNotFound(&'a str),
	FileNotFound(String),
	IOError(Error),
}

impl ReaderError {
	fn from_io_error(err: Error, path: &str) -> ReaderError {
		match err.kind() {
			std::io::ErrorKind::NotFound => ReaderError::FileNotFound(path.to_owned()),
			_ => ReaderError::IOError(err),
		}
	}

	fn as_string(&self) -> String {
		match self {
			ReaderError::IOError(err) => format!("IO error {}", err),
			ReaderError::BadFormat(st) => st.to_owned(),
			ReaderError::FileNotFound(path) => format!("File '{}' Not found!", path),
		}
	}
}

impl Display for ReaderError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let response = self.as_string();
		f.write_str(&response).unwrap();

		Ok(())
	}
}

impl From<ParseFloatError> for ReaderError {
	fn from(err: ParseFloatError) -> Self {
		Self::BadFormat(format!("Parse float error: '{}'", err))
	}
}

impl From<ParseIntError> for ReaderError {
	fn from(err: ParseIntError) -> Self {
		Self::BadFormat(format!("Parse int error: '{}'", err))
	}
}

// TODO: instead of propagating the shit all the way, I can make ReaderError have a string that expands with the error (shows lines that went bad)
pub fn read_mesh_from_obj(path: &str) -> Result<Mesh, ReaderError> {

	let file_content = fs::read_to_string(path).map_err(|err| ReaderError::from_io_error(err, path))?;

	let mut verts   = vec![];
	let mut tris    = vec![];
	let mut normals = vec![];
	let mut normal_indices = vec![];

	let mut has_found_normals = false;

	for (i, line) in file_content.lines().enumerate() {
		let line_split_by_space = line.split(' ').skip(1);
		
		if line.starts_with("v ") {
			for vert_str in line_split_by_space {
				if vert_str.is_empty() { continue }

				let vert = vert_str.parse::<f32>().or(
					Err(
						ReaderError::BadFormat(format!("cant parse float '{}'\nline {}: '{}'", vert_str, i+1, line))
					)
				)?;
				verts.push(vert);
			}

			continue;
		}

		if line.starts_with("vn ") {
			for normal_str in line_split_by_space {
				if normal_str.is_empty() { continue; }

				let normal = normal_str.parse::<f32>().or(
					Err(
						ReaderError::BadFormat(format!("cant parse float '{}'\nline {}: '{}'", normal_str, i+1, line))
					)
				)?;
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

				let vertex_index_str = indices.next().ok_or(ReaderError::BadFormat(
					format!("cant skip indices iterator\nline {}: '{}'", i, line)
				))?;
				let vertex_index = vertex_index_str.parse::<u16>().or(
					Err(
						ReaderError::BadFormat(format!("cant parse int 16 '{}'\nline {}: '{}'", vertex_index_str, i+1, line))
					)
				)? - 1;
				tris.push(vertex_index);

				// skip texture coordinates
				indices.next();

				if has_found_normals {
					let normal_index = indices.next().ok_or(ReaderError::BadFormat(
						format!("cant parse normal iterator\nline {}: '{}'", i+1, line)
					))?;
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

