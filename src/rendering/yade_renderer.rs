use crate::{camera::Camera, file_readers::yade_dem_reader::YadeDemData, render_bounding_box, render_yade, render_yade_sorted, renderer::Renderer, terminal::TerminalBuffer, timer::Timer, BoundingBox};

pub struct YadeRenderer {
	data: YadeDemData,
}

impl YadeRenderer {
	pub fn new(data: YadeDemData) -> Self {
		Self { data }
	}
}

impl Renderer for YadeRenderer {
	fn render(&self, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
		render_yade_sorted(&self.data, buf, timer, camera);

		// let bbox = BoundingBox::from_vec3_iter(self.data.get_verts_iter());
		// render_bounding_box(&bbox, buf, timer, camera);
	}
}