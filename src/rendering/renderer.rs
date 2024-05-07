use crate::{camera::Camera, terminal::TerminalBuffer, timer::Timer};


pub trait Renderer {
	fn render(&self, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera);
}
