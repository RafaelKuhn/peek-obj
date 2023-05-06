use std::{io::{self, Stdout}, time::{Duration, Instant}, process};

use peekfbx::{rendering::{*, mesh::Mesh, self}, maths::UVec2, terminal::FreeText, timer::AppTimer, benchmark::Benchmark};

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

	let mut timer = AppTimer::init();
	let mut benchmark = Benchmark::default();

	loop {
		if app.is_rendering_paused {
			poll_events(terminal, &mut app);
			continue;
		}

		render_clear(&mut app.text_buffer.text);		

		test_besenham(&mut app.text_buffer.text, app.width, app.height, timer.time_since_start.as_millis() as i32);
		draw_triangles_wire(&screen_space_tris, &mut app.text_buffer.text, app.width);
		
		poll_events(terminal, &mut app);

		terminal.draw(|frame| terminal_render(frame, &app.text_buffer)).unwrap();


		benchmark.profile_frame(&timer);
		draw_benchmark(&mut app.text_buffer.text, app.width, app.height, &benchmark);

		timer.add_frame();
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
		// TODO: I have no fucking clue why I need to add 1 here
		self.width = w + 1;
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
				KeyCode::Char('p') => app.is_rendering_paused = !app.is_rendering_paused,
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