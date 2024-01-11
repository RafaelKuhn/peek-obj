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


use std::{io::{self, Stdout}, time::Duration, process, env};

use maths::{build_identity_4x4, Vec3};
use obj_reader::read_mesh_from_obj;
use rendering::{*};
use settings::Settings;
use terminal::FreeText;
use timer::AppTimer;
use benchmark::Benchmark;

use tui::{
	backend::{CrosstermBackend, Backend},
	Terminal, Frame, layout::Rect,
};
use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::rendering::mesh::Mesh;

type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;
type DrawFunction = fn(&Mesh, &mut Vec<char>, (u16, u16), &AppTimer, (&mut Vec<f32>, &mut Vec<f32>));


fn main() {
	// TODO:
	// print "File '$' not found"
	// if file is not found but the content is smth like "cube" or "sphere", use them instead
	// use macros for this

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
		Ok(mesh) => mesh,
		Err(err) => {
			println!("{:}", err);
			process::exit(1);
		},
	};

	let terminal = &mut configure_terminal();
	Terminal::hide_cursor(terminal).unwrap();

	let mut app;
	{
		let Rect { width, height, .. } = terminal.size().unwrap();
		app = App::new(width, height);
	}

	let mut timer = AppTimer::init();

	let benchmark_refresh_rate = 0.5;
	let mut benchmark = Benchmark::new(benchmark_refresh_rate);

	let mut transform_mat  = build_identity_4x4();
	let mut projection_mat = build_identity_4x4();

	let draw_mesh: DrawFunction = if settings.draw_normals { draw_mesh_wire_and_normals } else { draw_mesh_wire };

	loop {
		if app.has_paused_rendering {
			timer.run();
			poll_events(terminal, &mut app, &mut timer);
			continue;
		}

		render_clear(&mut app.text_buffer.text);	

		draw_mesh(&mesh, &mut app.text_buffer.text, (app.width, app.height), &timer, (&mut transform_mat, &mut projection_mat));

		poll_events(terminal, &mut app, &mut timer);

		benchmark.profile_frame(&timer);
		draw_benchmark(&mut app.text_buffer.text, app.width, app.height, &benchmark);
		draw_timer(&mut app.text_buffer.text, app.width, app.height, &timer);

		timer.run();

		terminal.draw(|frame| terminal_render(frame, &app.text_buffer)).unwrap();
	}
}


fn configure_terminal() -> CrosstermTerminal {
	enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

	Terminal::new(CrosstermBackend::new(stdout)).unwrap()
}

fn restore_terminal(terminal: &mut CrosstermTerminal) {
	disable_raw_mode().unwrap();
	execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
	terminal.show_cursor().unwrap();
}

fn terminal_render<B: Backend>(frame: &mut Frame<B>, text: &FreeText) {
	let rect = frame.size();
	frame.render_widget(text, rect);
}

struct App {
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
		self.width = w + 1;
		self.height = h;

		self.text_buffer = FreeText::from_screen(w, h);

		// not a good idea but when rendering is disabled, we could copy the content of the previous frame to the new one and draw
		// this would require a mut ref to the terminal
	}
}



fn poll_events(terminal: &mut CrosstermTerminal, app: &mut App, timer: &mut AppTimer) {
	let has_event = crossterm::event::poll(Duration::from_millis(0)).unwrap();
	if !has_event { return }

	match event::read().unwrap() {
		Event::Key(key) => {
			match key.code {
				// KeyCode::Up    | KeyCode::Char('w') => quit_with_message(terminal, "Move Up"),
				// KeyCode::Left  | KeyCode::Char('a') => quit_with_message(terminal, "Move Left"),
				// KeyCode::Down  | KeyCode::Char('s') => quit_with_message(terminal, "Move Down"),
				// KeyCode::Right | KeyCode::Char('d') => quit_with_message(terminal, "Move Right"),
				// KeyCode::Char(ch) => quit_with_message(terminal, &format!("Needs to parse char {ch}")),
				KeyCode::Char('p') => {
					if app.has_paused_rendering {
						app.has_paused_rendering = false;
						timer.time_scale = 1.0;
					} else {
						app.has_paused_rendering = true;
						timer.time_scale = 0.0;
					}
				} 
				KeyCode::Esc => quit(terminal),
				// return
				_ => (),
			}
		}
		Event::Resize(new_width, new_height) => {
			app.resize_realloc(new_width, new_height);
		}
		// return
		_ => (),
	}
}

fn quit(terminal: &mut CrosstermTerminal) {
	restore_terminal(terminal);
	process::exit(0);
}

fn quit_with_message(terminal: &mut CrosstermTerminal, message: &str) {
	restore_terminal(terminal);
	println!("{message}");
	process::exit(0);
}
