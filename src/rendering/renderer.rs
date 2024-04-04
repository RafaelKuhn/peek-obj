use crate::{camera::Camera, file_readers::yade_dem_reader::YadeDemData, mesh::Mesh, obj_renderer::ObjRenderer, terminal_wrapper::TerminalBuffer, timer::Timer, yade_renderer::YadeRenderer, FileType};


pub trait Renderer {
	fn render(&self, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera);
}


// pub fn make_renderer(data: FileType) -> Box<dyn Renderer> {
// 	return match data {
// 		// FileType::Mesh(data) => &ObjRenderer::new(data),
// 		FileType::Mesh(data) => Box::new(&ObjRenderer::new(data)),
// 		FileType::YadeData(data) => Box::new(&YadeRenderer::new(data)),
// 	}
// }

// pub trait DataWeCanDraw { }
