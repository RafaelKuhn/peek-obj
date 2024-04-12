#![allow(dead_code)]
#![allow(unreachable_code)]

#![allow(clippy::identity_op)]
#![allow(clippy::erasing_op)]


mod rendering;
mod maths;
mod terminal_wrapper;
mod timer;
mod benchmark;
mod file_readers;
mod settings;


use std::{env, io::{stdout, Write}};

use crossterm::{cursor::*, queue, style::Print, terminal::*};

use file_readers::yade_dem_reader::YadeDemData;
use rendering::{renderer::Renderer, yade_renderer::YadeRenderer, *};
use settings::Settings;
use terminal_wrapper::CrosstermTerminal;
use timer::Timer;
use benchmark::Benchmark;

use crate::{file_readers::obj_reader::read_mesh_from_obj, maths::*, obj_renderer::ObjRenderer, rendering::{camera::Camera, mesh::Mesh}, terminal_wrapper::{configure_terminal, poll_events, queue_draw_to_terminal_and_flush, restore_terminal, TerminalBuffer}};

// TODO: figure out how to do it more functionally if I wanted to
type RenderMeshFn = fn(&Mesh, &mut TerminalBuffer, &Timer, &Camera);
type RenderYadeFn = fn(&YadeDemData, &mut TerminalBuffer, &Timer, &Camera);

enum FileDataType {
	Mesh(Mesh),
	YadeData(YadeDemData),
}


fn main() {

	let args = env::args().skip(1);
	let settings = Settings::from_args(args);

	if !settings.has_custom_path {
		eprintln!("Provide a path");
		std::process::exit(1);
	}

	let data_to_draw = if settings.custom_path.ends_with(".obj") {
		FileDataType::Mesh(read_mesh_from_obj(&settings.custom_path).unwrap())
	} else {
		FileDataType::YadeData(YadeDemData::read_from_file_or_quit(&settings.custom_path))
	};

	let terminal_mut = &mut configure_terminal();

	let mut app = App::init_with_screen();
	// let mut app = App::init(32, 32);

	let mut timer = Timer::new();

	const BENCHMARK_REFRESH_RATE: f32 = 0.5;
	let mut benchmark = Benchmark::new(BENCHMARK_REFRESH_RATE);

	let mut camera = Camera::new();

	// TODO: why does setting the camera like this here puts it forward? should be ... Z?
	camera.set_pos(20., 1.0, 0.0);
	// camera.set_rot(6.28318530 * 0.1,  0.0, 0.0);
	// camera.set_rot(0.0, 6.2831 * 0.01, 0.0);

	camera.update_view_matrix();

	// #if MESH
	// let draw_mesh: DrawMeshFunction = if settings.draw_wireframe {
	// 	if settings.draw_normals { draw_mesh_wire_and_normals } else { draw_mesh_wire }
	// } else {
	// 	if settings.draw_normals { panic!("Can't draw normals + filled yet") } else { draw_mesh_filled }
	// };
	// #endif

	// TODO: try doing it with this:
	// https://refactoring.guru/design-patterns/abstract-factory/rust/example
	
	// TODO: try this less blurry crap
	// https://stackoverflow.com/questions/25445761/returning-a-closure-from-a-function


	// BUNNY config
	// mesh.invert_mesh_yz();
	// translate_mesh(&mut mesh, &Vec3::new(0.0, 0.0, -0.125));

	let renderer: Box<dyn Renderer> = match data_to_draw {
		FileDataType::YadeData(yade_data) => Box::new(YadeRenderer::new(yade_data)),
		FileDataType::Mesh(mesh) => Box::new(ObjRenderer::new(mesh)),
	};

	let mesh = Mesh::pillars();

	loop {
		just_poll_while_paused(&mut app, terminal_mut, &mut timer);
		render_clear(&mut app.buf);

		poll_events(terminal_mut, &mut app, &mut timer);

		app.buf.update_proj_matrix();

		// camera.position = Vec3::new(camera.position.x + app.pos.x, camera.position.y + app.pos.y, )
		camera.position = &camera.position + &app.pos;
		camera.rotation = &camera.rotation + &app.rot;
		camera.update_view_matrix();

		// TODO: render other crap
		render_axes(&mut app.buf, &timer, &camera);

		// let rad = 0.01 * YADE_SCALE_TEMP;
		// render_sphere(&Vec3::new(0.0, 0.0, 12.0), rad, &mut app.buf, &timer);

		// render_circle(&UVec2::new(app.buf.wid / 2, app.buf.hei / 2), app.buf.hei as f32 / 4.0, &mut app.buf);

		// render_mesh(&mesh, &mut app.buf, &timer, &camera);


		renderer.render(&mut app.buf, &timer, &camera);

		benchmark.profile_frame(&timer);
		render_benchmark(&benchmark, &mut app.buf);

		timer.run_frame();

		queue_draw_to_terminal_and_flush(&app.buf, terminal_mut);
	}

	restore_terminal(terminal_mut);
}

// TODO: try doing this with static dispatch (maybe make a RenderLoop function that accepts a generic shit like this)

// fn render<T: Renderer>(renderer: &T, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
// fn render(renderer: &dyn Renderer, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
fn render(renderer: Box<dyn Renderer>, buf: &mut TerminalBuffer, timer: &Timer, camera: &Camera) {
	renderer.render(buf, timer, camera);
}

pub fn just_poll_while_paused(app: &mut App, terminal_mut: &mut CrosstermTerminal, timer: &mut Timer) {
	
	if !app.has_paused_rendering { return; }

	let paused_str = "PAUSED!";
	render_string(paused_str, &UVec2::new(app.buf.wid - paused_str.len() as u16, app.buf.hei - 1), &mut app.buf);
	queue_draw_to_terminal_and_flush(&app.buf, terminal_mut);

	while app.has_paused_rendering {
		poll_events(terminal_mut, app, timer);
		timer.run();
	};
}

fn test_shit2() {

	let mut buf = [
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,

		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
		0, 0, 0, 0,
	];

	let mut stdout = stdout();
	
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[0..16]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[4..16]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[8..16]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[12..16]);
	
	// BACKGROUND_FILL_CHAR.encode_utf8(&mut slice[1..4]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[16..32]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[20..32]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[24..32]);
	BACKGROUND_FILL_CHAR.encode_utf8(&mut buf[28..32]);
	// BACKGROUND_FILL_CHAR.encode_utf8(&mut slice[5..8]);

	
	// let buf_str = unsafe { std::str::from_utf8_unchecked(&slice) };
	let buf_str = std::str::from_utf8(&buf[0..16]).unwrap();
	queue!(stdout, MoveTo(0, 0)).unwrap();
	queue!(stdout, Print(buf_str)).unwrap();
	
	let buf_str = std::str::from_utf8(&buf[16..32]).unwrap();
	queue!(stdout, MoveTo(0, 1)).unwrap();
	queue!(stdout, Print(buf_str)).unwrap();

	stdout.flush().unwrap();

	let _buf = vec![
		b'a', b'a', b'a', b'a', //  0  1  2  3
		b'b', 0, 0, 0, //  4  5  6  7
		b'c', 0, 0, 0, //  8  9 10 11
		b' ', 0, 0, 0, // 12 13 14 15
		0,    0, 0, 0, // 16 17 18 19
		// invalid utf8 example
		// 0xf0, 0x28, 0x8c, 0xbc,
		// 128, 223,
	];

	// '💖'.encode_utf8(&mut buf[16..20]);

}


pub struct App {
	pub can_resize: bool,
	pub has_paused_rendering: bool,

	pub buf: TerminalBuffer,

	pub pos: Vec3,
	pub rot: Vec3,
}


impl App {
	fn init_with_screen() -> App {
		let (screen_width, screen_height) = size().unwrap();
		Self::init(screen_width, screen_height, true)
	}

	fn init_wh(width: u16, height: u16) -> App {
		Self::init(width, height, false)
	}

	fn init(width: u16, height: u16, can_resize: bool) -> App {
		Self {
			can_resize,
			has_paused_rendering: false,

			buf: TerminalBuffer::new(width, height),
			pos: Vec3::zero(),
			rot: Vec3::zero(),
		}
	}

	fn resize_realloc(&mut self, w: u16, h: u16) {

		if !self.can_resize { return; }

		self.buf.resize_and_render_clear(w + 1, h + 1);

		// not a good idea but when rendering is disabled, we could copy the content of the previous frame
		// reescale it and draw it into the new one (this would require a mut ref to the terminal)
	}
}