use crate::{camera::Camera, mesh::Mesh, render_mesh, renderer::{Renderer}, terminal_wrapper::TerminalBuffer, timer::Timer};

pub struct ObjRenderer {
	data: Mesh,
	render: fn(mesh: &Mesh, &mut TerminalBuffer, &Timer, &Camera),
}

impl ObjRenderer {
	pub fn new(data: Mesh) -> Self {
		ObjRenderer {
			data,
			render: render_mesh,
		}
	}
}

// impl DataWeCanDraw for ObjRenderer { }

impl Renderer for &ObjRenderer {
	fn render(&self, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
		todo!()
	}
}
