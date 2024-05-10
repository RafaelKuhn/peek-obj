// #![allow(unused)]
// #![allow(unused_variables)]
#![allow(dead_code)]

#![allow(clippy::redundant_field_names)]
#![allow(clippy::identity_op)]
// #![allow(clippy::erasing_op)]

#[deny(unused_must_use)]


mod rendering;
mod maths;
mod terminal;
mod timer;
mod fps_measure;
mod benchmark;
mod file_readers;
mod settings;
mod utils;


use std::{env, io, time::{Duration, Instant}};

use crossterm::terminal::*;

use file_readers::yade_dem_reader::YadeDemData;
use rendering::{camera::Camera, mesh::Mesh, renderer::Renderer, yade_renderer::YadeRenderer, *};
use settings::Settings;
use timer::Timer;
use fps_measure::FpsMeasure;
use terminal::*;
use maths::*;

use crate::{benchmark::Benchmark, file_readers::obj_reader::read_mesh_from_obj_file, obj_renderer::ObjRenderer};


fn main() {

	let args = env::args().skip(1);
	let settings = Settings::from_args(args);

	if !settings.has_custom_path {
		eprintln!("Provide a path");
		std::process::exit(1);
	}

	if settings.custom_path.ends_with(".obj") {
		let renderer = ObjRenderer::new(read_mesh_from_obj_file(&settings.custom_path).unwrap());
		run_pipeline(renderer);
	} else {
		let renderer = YadeRenderer::new(YadeDemData::read_from_file_or_quit(&settings.custom_path));
		run_pipeline(renderer);
	};
}

// TODO: try doing renderer with this:
// https://refactoring.guru/design-patterns/abstract-factory/rust/example

// TODO: try this less blurry crap
// https://stackoverflow.com/questions/25445761/returning-a-closure-from-a-function
// type RenderMeshFn = fn(&Mesh, &mut TerminalBuffer, &Timer, &Camera);
// type RenderYadeFn = fn(&YadeDemData, &mut TerminalBuffer, &Timer, &Camera);

fn run_pipeline<T: Renderer>(renderer: T) {
	let mut app = App::init_with_screen();
	// let mut app = App::init_wh(100, 30);

	let mut timer = Timer::new();

	const FPS_MEAS_REFRESH_RATE: f32 = 0.5;
	let mut fps_measure = FpsMeasure::new(FPS_MEAS_REFRESH_RATE);

	let mut camera = Camera::new();
	camera.configure_defaults();

	timer.set_default_time_scale(1.0);

	let mut terminal = configure_terminal();
	set_panic_hook();

	let print_to_terminal_func = if app.is_full_screen { print_and_flush_terminal_fscreen } else { print_and_flush_terminal_line_by_line };
	let yade_debug = YadeDemData::debug();

	// let mesh_debug = Mesh::pillars();
	// let mesh_debug = read_mesh_from_obj_file("data/obj/teapot.obj").unwrap();
	// let mesh_debug_box = BoundingBox::from_verts(&mesh_debug.verts);

	app.buf.update_proj_matrix();
	let mut benchmark = Benchmark::default();

	loop {
		app.buf.clear_debug();
		app.buf.write_debug(&format!("with resolution {} x {}\n", app.buf.wid, app.buf.hei));
		just_poll_while_paused(&mut app, &mut terminal, &mut timer);

		benchmark.start();
		render_clear(&mut app.buf);
		benchmark.end_and_log("render clear", &mut app.buf);

		poll_events(&mut terminal, &mut app, &mut timer);
		benchmark.end_and_log("poll events", &mut app.buf);

		update_camera(&mut camera, &mut app);

		benchmark.start();
		// render_axes(&mut app.buf, &camera, true);
		// benchmark.end_and_log("render axes", &mut app.buf);

		// render_yade(&yade_debug, &mut app.buf, &timer, &camera);
		// render_mesh(&mesh_debug, &mut app.buf, &timer, &camera);
		// benchmark.end_and_log("render debug", &mut app.buf);


		renderer.render(&mut app.buf, &timer, &camera);
		benchmark.end_and_log("renderer render", &mut app.buf);
		
		render_gizmos(&mut app.buf, &camera);
		benchmark.end_and_log("renderer gizmos", &mut app.buf);

		fps_measure.profile_frame(&timer);
		benchmark.start();
		render_benchmark(&fps_measure, &camera, &mut app.buf);
		benchmark.end_and_log("render benchmark", &mut app.buf);

		timer.run_frame();

		try_saving_screenshot(&mut app, &timer);
		benchmark.start();
		print_to_terminal_func(&mut app.buf, &mut terminal);
		benchmark.end_and_log("print to terminal", &mut app.buf);
		app.buf.write_debug(&benchmark.accum_end());
	}
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
		camera.update_view_matrix();
		return;
	}

	if app.called_set_camera_default_orientation {
		app.called_set_camera_default_orientation = false;

		camera.set_initial_pos(camera.position.x, camera.position.y, camera.position.z);
		camera.set_initial_rot(camera.rotation.x, camera.rotation.y, camera.rotation.z);
		return;
	}

	camera.rotation = camera.rotation + app.user_rot;
	camera.position = camera.position + app.user_pos;

	let dir_vec = (camera.side * app.user_dir.x) + (camera.up * app.user_dir.y) + (camera.forward * app.user_dir.z);
	camera.position = camera.position + dir_vec;

	// TODO: only needs to do this when app.dir, app.rot or app.pos changesz	
	camera.update_view_matrix();
}

fn try_saving_screenshot(app: &mut App, timer: &Timer) {
	let now = timer.last_tick;
	let diff_since_last_ss = now - app.last_screenshot_instant;

	let last_one_was_too_recent = diff_since_last_ss < App::SCREENDUMP_DELAY_DURATION;
	if last_one_was_too_recent {
		let diff_secs = diff_since_last_ss.as_millis() as f32 / 1000.0;
		render_string(&format!("{}", diff_secs), &UVec2::new(0, 0), &mut app.buf);
		return;
	}

	if app.called_take_screenshot {
		app.called_take_screenshot = false;
		app.last_screenshot_instant = now;

		app.buf.try_dump_buffer_content_to_file();
	}
}

fn set_panic_hook() {
	let hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(move |info| {
		restore_stdout(&mut io::stdout());

		hook(info);
		restore_stdout(&mut io::stdout());
	}));
}


pub struct App {
	pub is_full_screen: bool,
	pub has_paused_rendering: bool,

	pub buf: TerminalBuffer,

	// TODO: user polled data
	// or just take a reference to the camera as Rc<Camera>, simpler
	pub user_dir: Vec3,
	pub user_pos: Vec3,
	pub user_rot: Vec3,

	pub called_reset_camera: bool,
	pub called_set_camera_default_orientation: bool,
	pub called_take_screenshot: bool,
	pub last_screenshot_instant: Instant,
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
	fn init(width: u16, height: u16, is_full_screen: bool) -> App {
		Self {
			is_full_screen,
			has_paused_rendering: false,

			buf: TerminalBuffer::new(width, height),

			user_pos: Vec3::zero(),
			user_dir: Vec3::zero(),
			user_rot: Vec3::zero(),

			called_reset_camera: false,
			called_set_camera_default_orientation: false,
			called_take_screenshot: false,
			last_screenshot_instant: Instant::now() - Duration::from_millis(App::SCREENDUMP_DELAY_MS),
		}
	}

	fn resize_realloc(&mut self, w: u16, h: u16) {

		if !self.is_full_screen { return; }

		// I have NO IDEA why Windows terminals need +1 for buffer size on resize events

		#[cfg(windows)]
		self.buf.resize_and_render_clear(w + 1, h + 1);

		#[cfg(unix)]
		self.buf.resize_and_render_clear(w, h);
	
		// (this is not a good idea) but ... when rendering is disabled, we could copy
		// the content of the previous frame, reescale it and draw it into the new one
		// (simply not gonna work)

		self.buf.update_proj_matrix();
	}

}