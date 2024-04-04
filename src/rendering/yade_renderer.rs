use crate::{camera::Camera, file_readers::yade_dem_reader::YadeDemData, renderer::Renderer, terminal_wrapper::TerminalBuffer, timer::Timer};

pub struct YadeRenderer {
	data: YadeDemData,
}

impl YadeRenderer {
	pub fn new(data: YadeDemData) -> Self {
		Self { data }
	}
}

impl Renderer for &YadeRenderer {
	fn render(&self, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
		todo!()
	}
}