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
mod file_readers;

mod app;
mod timer;
mod fps_measure;
mod benchmark;
mod settings;
mod utils;


use std::{env, io};

use app::App;
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
		// TODO: "or_quit" function
		let obj_renderer = ObjRenderer::new(read_mesh_from_obj_file(&settings.custom_path).unwrap());
		run_pipeline(obj_renderer);
	} else {
		let yade_dem_renderer = YadeRenderer::new(YadeDemData::read_from_file_or_quit(&settings.custom_path));
		run_pipeline(yade_dem_renderer);
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

	const FPS_MEASURE_REFRESH_RATE_SECS: f32 = 0.5;
	let mut fps_measure = FpsMeasure::new(FPS_MEASURE_REFRESH_RATE_SECS);

	let mut camera = Camera::new();
	camera.configure_defaults();

	timer.set_default_time_scale(1.0);

	let mut terminal = configure_terminal();
	set_panic_hook();

	let print_to_terminal_func = if app.is_full_screen { print_and_flush_terminal_fscreen } else { print_and_flush_terminal_line_by_line };
	// let yade_debug = YadeDemData::debug();

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
		// render_axes(&mut app.buf, &camera, false);
		// benchmark.end_and_log("render axes", &mut app.buf);

		// render_yade(&yade_debug, &mut app.buf, &timer, &camera);
		// render_yade_sorted(&yade_debug, &mut app.buf, &timer, &camera);
		// render_mesh(&mesh_debug, &mut app.buf, &timer, &camera);
		// benchmark.end_and_log("render debug", &mut app.buf);


		renderer.render(&mut app.buf, &timer, &camera);
		benchmark.end_and_log("renderer render", &mut app.buf);

		render_gizmos(&mut app.buf, &camera);
		benchmark.end_and_log("renderer gizmos", &mut app.buf);

		benchmark.start();
		fps_measure.profile_frame(&timer);
		render_verbose(&fps_measure, &camera, &mut app.buf);
		benchmark.end_and_log("render benchmark", &mut app.buf);

		timer.run_frame();
		app.run_post_render_events(&timer);

		benchmark.start();
		print_to_terminal_func(&mut app.buf, &mut terminal);
		benchmark.end_and_log("print to terminal", &mut app.buf);
		app.buf.write_debug(&benchmark.accum_end());
	}
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

fn set_panic_hook() {
	let hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(move |info| {
		restore_stdout(&mut io::stdout());

		hook(info);
		restore_stdout(&mut io::stdout());
	}));
}