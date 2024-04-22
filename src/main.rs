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


use std::{env, io, time::{Duration, Instant}};

use crossterm::terminal::*;

use file_readers::yade_dem_reader::YadeDemData;
use file_readers::obj_reader::read_mesh_from_obj;
use rendering::{renderer::Renderer, yade_renderer::YadeRenderer, *};
use settings::Settings;
use timer::Timer;
use benchmark::Benchmark;

use crate::{maths::*, obj_renderer::ObjRenderer, rendering::{camera::Camera, mesh::Mesh}, terminal_wrapper::*};

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
	set_panic_hook();

	let mut app = App::init_with_screen();
	// let mut app = App::init(32, 32);

	let mut timer = Timer::new();

	const BENCHMARK_REFRESH_RATE: f32 = 0.5;
	let mut benchmark = Benchmark::new(BENCHMARK_REFRESH_RATE);

	let mut camera = Camera::new();

	camera.set_initial_pos(0., 0.0, 30.0);
	// TODO: use this to debug (AXIS_SZ_WORLD == 20.0)
	// camera.set_initial_pos(-4.53, 5.04, 18.23);
	// camera.set_initial_rot(0.25, 0.15, 0.0);
	// ... or this
	// camera.set_initial_pos(2.87, 2.85, 19.44);
	// camera.set_initial_rot(0.15, -0.15, 0.00);
	camera.update_view_matrix(&mut app.buf);

	// #if MESH
	// let draw_mesh: DrawMeshFunction = if settings.draw_wireframe {
	// 	if settings.draw_normals { draw_mesh_wire_and_normals } else { draw_mesh_wire }
	// } else {
	// 	if settings.draw_normals { panic!("Can't draw normals + filled yet") } else { draw_mesh_filled }
	// };
	// #endif

	// TODO: try doing renderer with this:
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
		update_camera(&mut camera, &mut app);

		render_gizmos(&mut app.buf, &camera);
		render_axes(&mut app.buf, &camera);

		// let rad = 0.01 * YADE_SCALE_TEMP;
		// render_sphere(&Vec3::new(0.0, 0.0, 12.0), rad, &mut app.buf, &timer);

		// render_circle(&UVec2::new(app.buf.wid / 2, app.buf.hei / 2), app.buf.hei as f32 / 4.0, &mut app.buf);

		render_mesh(&mesh, &mut app.buf, &timer, &camera);


		renderer.render(&mut app.buf, &timer, &camera);

		benchmark.profile_frame(&timer);
		render_benchmark(&benchmark, &camera, &mut app.buf);

		timer.run_frame();

		try_saving_screenshot(&mut app, &timer);
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

fn update_camera(camera: &mut Camera, app: &mut App) {
	if app.called_reset_camera {
		app.called_reset_camera = false;

		camera.restore_initial_pos_and_rot();
		camera.update_view_matrix(&mut app.buf);
		return;
	}

	camera.rotation = camera.rotation + app.rot;
	camera.position = camera.position + app.pos;

	let dir_vec = (camera.side * app.dir.x) + (camera.up * app.dir.y) + (camera.forward * app.dir.z);
	camera.position = camera.position + dir_vec;
	
	camera.update_view_matrix(&mut app.buf);
}

fn try_saving_screenshot(app: &mut App, timer: &Timer) {
	let now = timer.last_tick;
	let diff_since_last_ss = now - app.last_screenshot_instance;

	let last_one_was_too_recent = diff_since_last_ss < App::SCREENDUMP_DELAY_DURATION;
	if last_one_was_too_recent {
		let diff_secs = diff_since_last_ss.as_millis() as f32 / 1000.0;
		render_string(&format!("{}", diff_secs), &UVec2::new(0, 0), &mut app.buf);
		return;
	}

	if !app.called_take_screenshot { return }
	app.called_take_screenshot = false;
	app.last_screenshot_instance = now;

	app.buf.try_dump_buffer_content_to_file();
}

fn set_panic_hook() {
	let hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(move |info| {
		restore_stdout(&mut io::stdout());

		hook(info);
	}));
}


pub struct App {
	pub can_resize: bool,
	pub has_paused_rendering: bool,

	pub buf: TerminalBuffer,

	// TODO: user polled data
	// or just take a reference to the camera as Rc<Camera>, simpler
	pub dir: Vec3,
	pub pos: Vec3,
	pub rot: Vec3,
	pub called_reset_camera: bool,
	pub called_take_screenshot: bool,
	pub last_screenshot_instance: Instant,
}

impl App {
	fn init_with_screen() -> App {
		let (screen_width, screen_height) = size().unwrap();
		Self::init(screen_width, screen_height, true)
	}

	fn init_wh(width: u16, height: u16) -> App {
		Self::init(width, height, false)
	}

	const SCREENDUMP_DELAY_DURATION: Duration = Duration::from_millis(App::SCREENDUMP_DELAY_MS);
	const SCREENDUMP_DELAY_MS: u64 = 300;
	fn init(width: u16, height: u16, can_resize: bool) -> App {
		Self {
			can_resize,
			has_paused_rendering: false,

			buf: TerminalBuffer::new(width, height),

			pos: Vec3::zero(),
			dir: Vec3::zero(),
			rot: Vec3::zero(),
			called_reset_camera: false,
			called_take_screenshot: false,
			last_screenshot_instance: Instant::now() - Duration::from_millis(App::SCREENDUMP_DELAY_MS),
		}
	}

	fn resize_realloc(&mut self, w: u16, h: u16) {

		if !self.can_resize { return; }

		// I have NO IDEA why Windows terminals need +1 for buffer size on resize events

		#[cfg(windows)]
		self.buf.resize_and_render_clear(w + 1, h + 1);

		#[cfg(unix)]
		self.buf.resize_and_render_clear(w, h);

		// (this is not a good idea) but ... when rendering is disabled, we could copy
		// the content of the previous frame, reescale it and draw it into the new one
		// (simply not gonna work)
	}
}