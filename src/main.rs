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

use crossterm::{cursor::*, queue, style::Print, terminal::*, QueueableCommand};



use file_readers::yade_dem_reader::YadeDemData;
use rendering::{renderer::Renderer, yade_renderer::YadeRenderer, *};
use settings::Settings;
use terminal_wrapper::CrosstermTerminal;
use timer::Timer;
use benchmark::Benchmark;

use crate::{file_readers::{obj_reader::{read_mesh_from_obj, translate_mesh}, yade_dem_reader}, maths::{UVec2, Vec3}, obj_renderer::ObjRenderer, rendering::{camera::Camera, mesh::Mesh}, terminal_wrapper::{configure_terminal, poll_events, queue_draw_to_terminal_and_flush, restore_terminal, TerminalBuffer}};


// type DrawMeshFunction = fn(&Mesh, &mut [char], (u16, u16), &AppTimer, (&mut [f32], &mut [f32]), &Camera);
type DrawMeshFunction = fn(&Mesh, buffer: &mut [char], width_height: (u16, u16), &Timer, matrices: (&mut [f32], &mut [f32]), &Camera);

// TODO: isolate
enum FileType {
	Mesh(Mesh),
	YadeData(YadeDemData),
}

// type DrawFunction = fn(FileType, buffer: &mut [char], width_height: (u16, u16), &Timer, matrices: (&mut [f32], &mut [f32]), &Camera);

// TODO: figure out how to do it more functionally if I wanted to
type RenderMeshFn = fn(&Mesh, &mut TerminalBuffer, &Timer, &Camera);
type RenderYadeFn = fn(&YadeDemData, &mut TerminalBuffer, &Timer, &Camera);

fn main() {

	let args = env::args().skip(1);
	let settings = Settings::from_args(args);

	if !settings.has_custom_path {
		eprintln!("Provide a path");
		std::process::exit(1);
	}

	let data_to_draw = if settings.custom_path.ends_with(".obj") {
		FileType::Mesh(read_mesh_from_obj(&settings.custom_path).unwrap())
	} else {
		FileType::YadeData(YadeDemData::read_from_file_or_quit(&settings.custom_path))
	};

	let terminal_mut = &mut configure_terminal();

	let mut app = App::init_with_screen();
	// let mut app = App::init(32, 32);

	let mut timer = Timer::new();

	const BENCHMARK_REFRESH_RATE: f32 = 0.5;
	let mut benchmark = Benchmark::new(BENCHMARK_REFRESH_RATE);

	let mut camera = Camera::new();

	// TODO: why does setting the camera like this here puts it forward? should be the opposite ...
	// camera.set_pos(0.0, 0.0, 22.0);

	// camera.set_rot(6.28318530 * 6.5/8.,  0.0, 0.0);
	// camera.set_rot(0.0,  6.2831 * 0.0825, 0.0);

	// why?
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

	// let renderer = make_renderer(data_to_draw);

	// BUNNY config
	// mesh.invert_mesh_yz();
	// translate_mesh(&mut mesh, &Vec3::new(0.0, 0.0, -0.125));

	loop {
		just_poll_while_paused(&mut app, terminal_mut, &mut timer);
		render_clear(&mut app.buf);

		poll_events(terminal_mut, &mut app, &mut timer);

		// TODO: render other crap

		benchmark.profile_frame(&timer);
		render_benchmark(&benchmark, &mut app.buf);

		match data_to_draw {
			FileType::Mesh(ref mesh) => render_mesh(&mesh, &mut app.buf, &timer, &camera),
			FileType::YadeData(ref yade_data) => render_yade(&yade_data, &mut app.buf, &timer, &camera),
		}

		// render_mesh(&mesh, &mut app.buf, &timer, &camera);
		// render_yade(&yade_data, &mut app.buf, &timer, &camera);
		// renderer.render(&mut app.buf, &timer, &camera);

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

fn just_poll_while_paused(app: &mut App, terminal_mut: &mut CrosstermTerminal, timer: &mut Timer) {
	
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
}


impl App {
	fn init_with_screen() -> App {
		let (screen_width, screen_height) = size().unwrap();
		Self {
			can_resize: true,
			has_paused_rendering: false,
			buf: TerminalBuffer::new(screen_width, screen_height)
		}
	}

	fn init(screen_width: u16, screen_height: u16) -> App {
		Self {
			can_resize: false,
			has_paused_rendering: false,
			buf: TerminalBuffer::new(screen_width, screen_height)
		}
	}

	fn resize_realloc(&mut self, w: u16, h: u16) {

		if !self.can_resize { return; }

		self.buf.resize_and_render_clear(w + 1, h + 1);

		// not a good idea but when rendering is disabled, we could copy the content of the previous frame
		// reescale it and draw it into the new one (this would require a mut ref to the terminal)
	}
}