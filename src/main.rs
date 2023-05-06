use std::{io::{self, Stdout}, time::{Duration, Instant}, process};

use peekfbx::{rendering::{*, mesh::Mesh, self}, maths::UVec2, terminal::FreeText};

use tui::{
	backend::{CrosstermBackend, Backend},
	Terminal, Frame, layout::Rect,
};
use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

type CTerminal = Terminal<CrosstermBackend<Stdout>>;


fn main() {
	let mut terminal: Terminal<_> = configure_terminal();
	Terminal::hide_cursor(&mut terminal).unwrap();
	run_app(&mut terminal);
}

fn configure_terminal() -> CTerminal {
	enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
	let backend = CrosstermBackend::new(stdout);
	let terminal = Terminal::new(backend).unwrap();
	return terminal;
}

fn restore_terminal(terminal: &mut CTerminal){
	disable_raw_mode().unwrap();
	execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
	terminal.show_cursor().unwrap();
}


fn run_app(terminal: &mut CTerminal) {
	let mut app;
	{
		let Rect { width: screen_width, height: screen_height, .. } = terminal.size().unwrap();
		app = App::new(screen_width, screen_height);
	}
	// let mut text_buffer = FreeText::from_chars(rendering::BACKGROUND_FILL_CHAR, chars_size);
	
	// let mut app = App::new(screen_width, screen_height);
	// app.realloc_widget(screen_height, screen_height);

	// let mesh = Mesh::cube();

	let screen_space_tris = vec![
		ScreenTriangle {
			p0: UVec2::new(app.width/2+00, app.height/2+00),
			p1: UVec2::new(app.width/2-10, app.height/2+06),
			p2: UVec2::new(app.width/2+10, app.height/2+05),
		},
		ScreenTriangle {
			p0: UVec2::new(app.width/4-10, app.height/4-00),
			p1: UVec2::new(app.width/4+00, app.height/4+05),
			p2: UVec2::new(app.width/4+10, app.height/4-00),
		},
	];

	// let event_channel = EventChannel::new(screen_width, screen_height);

	// TODO: abstract
	let start = Instant::now();
	
	let mut frame_count: i32 = 0;
	let mut last_tick = Instant::now();
	let mut delta_time = Duration::from_millis(0);

	let mut accum_time = 1.0;
	let mut fps_frame_count = 0;

	let mut benchmark = Benchmark::default();

	loop {
		render_clear(&mut app.text_buffer.text);
		let delta_time_millis = delta_time.as_micros() as f32 * 0.000_001;
		
		let last_tick_temp = Instant::now();
		let time_spent = (last_tick_temp - start).as_millis();

		// test_besenham(&mut text_buffer.text, screen_width, screen_height, time_spent as i32);
		draw_triangles_wire(&screen_space_tris, &mut app.text_buffer.text, app.width);
		
		poll_events(terminal, &mut app);
		
		frame_count += 1;
		
		delta_time = last_tick_temp - last_tick;
		last_tick = last_tick_temp;
		

		fps_frame_count += 1;
		
		accum_time += delta_time_millis;
		
		let update_interval = 0.5;
		if accum_time > update_interval {
			benchmark.fps = (fps_frame_count as f32 / update_interval) as i32;
			benchmark.frame_count = frame_count;
			benchmark.delta_time = delta_time_millis;
			
			accum_time = 0.0;
			fps_frame_count = 0;
		}

		draw_benchmark(&mut app.text_buffer.text, app.width, app.height, &benchmark);
		terminal.draw(|frame| terminal_render(frame, &app.text_buffer)).unwrap();
	}
}


fn terminal_render<B: Backend>(frame: &mut Frame<B>, text: &FreeText) {
	let rect = frame.size();
	frame.render_widget(text, rect);
}

struct App {
	// pub func: fn(u16, u16),
	pub width:  u16,
	pub height: u16,
	pub text_buffer: FreeText,
}


impl App {
	fn new(screen_width: u16, screen_height: u16) -> App {
		Self {
			text_buffer: FreeText::from_screen(screen_width, screen_height),
			width: screen_width,
			height: screen_height,
		}
	}

	fn resize_realloc(&mut self, w: u16, h: u16) {
		self.width = w;
		self.height = h;
		self.text_buffer = FreeText::from_screen(w, h);
	}
}


fn poll_events(terminal: &mut CTerminal, app: &mut App) {
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

fn quit(terminal: &mut CTerminal) {
	restore_terminal(terminal);
	process::exit(0);
}

fn quit_with_message(terminal: &mut CTerminal, message: &str) {
	restore_terminal(terminal);
	println!("{message}");
	process::exit(0);
}