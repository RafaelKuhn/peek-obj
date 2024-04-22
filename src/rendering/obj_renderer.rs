use crate::{camera::Camera, mesh::{self, Mesh}, render_mesh, renderer::Renderer, terminal_wrapper::TerminalBuffer, timer::Timer};

pub struct ObjRenderer {
	mesh: Mesh,
	render: fn(mesh: &Mesh, &mut TerminalBuffer, &Timer, &Camera),
}

impl ObjRenderer {
	pub fn new(data: Mesh) -> Self {
		ObjRenderer {
			mesh: data,
			render: render_mesh,
		}
	}
}

impl Renderer for ObjRenderer {
	fn render(&self, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
		render_mesh(&self.mesh, buf, timer, camera);
	}
}
