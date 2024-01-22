#![allow(dead_code)]

#![allow(clippy::identity_op)]
#![allow(clippy::erasing_op)]

mod rendering;
mod maths;
mod terminal;
mod timer;
mod benchmark;
mod obj_reader;
mod settings;


use std::{process, env};

use maths::{build_identity_4x4, Vec3};
use obj_reader::read_mesh_from_obj;
use rendering::{*};
use settings::Settings;
use terminal::{FreeText, configure_terminal, poll_events, render_free_text, get_terminal_width_height};
use timer::AppTimer;
use benchmark::Benchmark;


use crate::rendering::{mesh::Mesh, camera::Camera};


// type DrawFunction = fn(&Mesh, &mut [char], (u16, u16), &AppTimer, (&mut [f32], &mut [f32]));
type DrawFunction = fn(&Mesh, &mut [char], (u16, u16), &AppTimer, (&mut [f32], &mut [f32]), &Camera);


fn main() {

	let args = env::args().skip(1);
	let settings = Settings::from_args(args);

	// TODO: check custom only if file is not found
	let mesh_result = if !settings.has_custom_path {
		let raw_teapot_result = read_mesh_from_obj("objs/teapot.obj");
		obj_reader::translate_mesh(raw_teapot_result, &Vec3::new(0.0, -1.575, 0.0))
	} else {
		read_mesh_from_obj(&settings.custom_path)
	};

	let mesh = match mesh_result {
		// Ok(mesh) => mesh,
		Ok(mut mesh) => {
			mesh.pos.x = 0.0;
			mesh.pos.y = 0.0;
			mesh.pos.z = 22.0;
			mesh
		}
		Err(err) => {
			println!("{:}", err);
			process::exit(1);
		},
	};

	let terminal = &mut configure_terminal();
	let (width, height) = get_terminal_width_height(terminal);

	let mut app = App::new(width, height);

	let mut timer = AppTimer::init();

	const BENCHMARK_REFRESH_RATE: f32 = 0.5;
	let mut benchmark = Benchmark::new(BENCHMARK_REFRESH_RATE);

	let mut camera = Camera::new();
	camera.set_pos(0.0, 0.0, -12.0);
	camera.set_rot(0.0,  0.0, 0.0);
	camera.set_rot(0.0,  6.2831 * 0.0825, 0.0);
	
	camera.update_view_matrix();

	let mut transform_mat  = build_identity_4x4();
	let mut projection_mat = build_identity_4x4();

	let draw_mesh: DrawFunction = if settings.draw_wireframe {
		if settings.draw_normals { draw_mesh_wire_and_normals } else { draw_mesh_wire }
	} else {
		if settings.draw_normals { panic!("Can't draw normals + filled yet") } else { draw_mesh_filled }
	};

	loop {
		if app.has_paused_rendering {
			timer.run();
			poll_events(terminal, &mut app, &mut timer);
			continue;
		}

		render_clear(&mut app.text_buffer.text);

		draw_mesh(&mesh, &mut app.text_buffer.text, (app.width, app.height), &timer, (&mut transform_mat, &mut projection_mat), &camera);

		poll_events(terminal, &mut app, &mut timer);

		benchmark.profile_frame(&timer);
		draw_benchmark(&mut app.text_buffer.text, app.width, app.height, &benchmark);
		draw_timer(&mut app.text_buffer.text, app.width, app.height, &timer);

		timer.run();

		terminal.draw(|frame| render_free_text(frame, &app.text_buffer)).unwrap();
	}
}


pub struct App {
	pub width:  u16,
	pub height: u16,
	pub has_paused_rendering: bool,

	pub text_buffer: FreeText,
}


impl App {
	fn new(screen_width: u16, screen_height: u16) -> App {
		Self {
			text_buffer: FreeText::from_screen(screen_width, screen_height),
			width: screen_width,
			height: screen_height,
			has_paused_rendering: false,
		}
	}

	// fn resize_realloc(&mut self, w: u16, h: u16, terminal: &mut CrosstermTerminal) {
	fn resize_realloc(&mut self, w: u16, h: u16) {

		// I have no fucking clue why but I need to add 1 here
		self.width  = w + 1;
		self.height = h ;

		self.text_buffer = FreeText::from_screen(w, h);

		// not a good idea but when rendering is disabled, we could copy the content of the previous frame to the new one and draw
		// this would require a mut ref to the terminal
	}
}