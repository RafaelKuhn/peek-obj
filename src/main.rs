#![allow(dead_code)]

mod rendering;
mod maths;
mod terminal;
mod timer;
mod benchmark;
mod obj_reader;


use std::{io::{self, Stdout}, time::Duration, process};

use maths::{Vec3, build_identity_4x4};
use obj_reader::{read_mesh_from_obj, transform_mesh};
use rendering::{*};
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

type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;


fn main() {
	// TODO:
	// print file not found when a problem is found
	// if file is not found but the content is smth like "cube" or "sphere", use them instead
	// use macros for this

	let result = transform_mesh(read_mesh_from_obj("objs/teapot.obj"), &Vec3::new(0.0, -1.575, 0.0));
	// let result = read_mesh_from_obj("objs/cube.obj");
	// let result = read_mesh_from_obj("objs/VideoShip.obj");


	let mesh = match result {
		Ok(read_mesh) => read_mesh,
		Err(err) => {
			println!("{:}", err);
			process::exit(1);
		},
	};

	let terminal_mut = &mut configure_terminal();
	Terminal::hide_cursor(terminal_mut).unwrap();

	let mut app;
	{
		let Rect { width, height, .. } = terminal_mut.size().unwrap();
		app = App::new(width, height);
	}

	let mut timer = AppTimer::init();
	
	let benchmark_refresh_rate = 0.5;
	let mut benchmark = Benchmark::new(benchmark_refresh_rate);

	let mut transform_mat  = build_identity_4x4();
	let mut projection_mat = build_identity_4x4();

	loop {
		if app.is_rendering_paused {
			poll_events(terminal_mut, &mut app, &mut timer);
			timer.add_frame();
			continue;
		}

		render_clear(&mut app.text_buffer.text);	

		draw_mesh_wire(&mesh, &mut app.text_buffer.text, (app.width, app.height), &timer, (&mut transform_mat, &mut projection_mat));
		// test_bresenham(&mut app.text_buffer.text, app.width, app.height, timer.time_since_start.as_millis() as i32);
		
		poll_events(terminal_mut, &mut app, &mut timer);

		benchmark.profile_frame(&timer);
		draw_benchmark(&mut app.text_buffer.text, app.width, app.height, &benchmark);
		draw_timer(&mut app.text_buffer.text, app.width, app.height, &timer);

		timer.add_frame();

		terminal_mut.draw(|frame| terminal_render(frame, &app.text_buffer)).unwrap();
	}

	// TODO: return correctly here, quit terminal here
}

// TODO: have a look on how this would work for termion
fn configure_terminal() -> CrosstermTerminal {
	enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
	let backend = CrosstermBackend::new(stdout);
	let terminal = Terminal::new(backend).unwrap();
	return terminal;
}

fn restore_terminal(terminal: &mut CrosstermTerminal){
	disable_raw_mode().unwrap();
	execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
	terminal.show_cursor().unwrap();
}

fn terminal_render<B: Backend>(frame: &mut Frame<B>, text: &FreeText) {
	let rect = frame.size();
	frame.render_widget(text, rect);
}

struct App {
	// pub func: fn(u16, u16),
	pub width:  u16,
	pub height: u16,
	pub is_rendering_paused: bool,

	pub text_buffer: FreeText,
}


impl App {
	fn new(screen_width: u16, screen_height: u16) -> App {
		Self {
			text_buffer: FreeText::from_screen(screen_width, screen_height),
			width: screen_width,
			height: screen_height,
    		is_rendering_paused: false,
		}
	}

	fn resize_realloc(&mut self, w: u16, h: u16) {
		// I have no fucking clue why I need to add 1 here
		self.width = w + 1;
		self.height = h;
		self.text_buffer = FreeText::from_screen(w, h);
	}
}


fn poll_events(terminal: &mut CrosstermTerminal, app: &mut App, timer: &mut AppTimer) {
	let has_event = crossterm::event::poll(Duration::from_millis(0)).unwrap();
	if !has_event { return; }

	match event::read().unwrap() {
		Event::Key(key) => {
			match key.code {
				// KeyCode::Up    | KeyCode::Char('w') => quit_with_message(terminal, "Move Up"),
				// KeyCode::Left  | KeyCode::Char('a') => quit_with_message(terminal, "Move Left"),
				// KeyCode::Down  | KeyCode::Char('s') => quit_with_message(terminal, "Move Down"),
				// KeyCode::Right | KeyCode::Char('d') => quit_with_message(terminal, "Move Right"),
				// KeyCode::Char(ch) => quit_with_message(terminal, &format!("Needs to parse char {ch}")),
				KeyCode::Char('p') => {
					if app.is_rendering_paused {
						app.is_rendering_paused = false;
						timer.time_scale = 1.0;
					} else {
						app.is_rendering_paused = true;
						timer.time_scale = 0.0;
					}
				} 
				KeyCode::Esc => quit(terminal),
				_ => return,
			}
		}
		Event::Resize(new_width, new_height) => {
			app.resize_realloc(new_width, new_height);
		}
		_ => return,
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