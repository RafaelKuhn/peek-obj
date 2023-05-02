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
	let mut last_tick = Instant::now();
	let mut delta_time = Duration::from_millis(0);

	let Rect { width: screen_width, height: screen_height, .. } = terminal.size().unwrap();
	let chars_size = (screen_height * screen_width) as usize;

	let mut text_buffer = FreeText::from_chars(rendering::BACKGROUND_FILL_CHAR, chars_size);
	
	// let mesh = Mesh::cube();

	let screen_space_tris = vec![
		ScreenTriangle {
			p0: UVec2::new(screen_width/2+00, screen_height/2+00),
			p1: UVec2::new(screen_width/2-10, screen_height/2+06),
			p2: UVec2::new(screen_width/2+10, screen_height/2+05),
		},
		ScreenTriangle {
			p0: UVec2::new(screen_width/4-10, screen_height/4-00),
			p1: UVec2::new(screen_width/4+00, screen_height/4+05),
			p2: UVec2::new(screen_width/4+10, screen_height/4-00),
		},
	];

	let start = Instant::now();
	let mut frame_count: i32 = 0;

	let mut accum_time = 1.0;
	let mut fps_frame_count = 0;

	let mut benchmark = Benchmark::default();

	loop {
		render_clear(&mut text_buffer.text);
		let delta_time_millis = delta_time.as_millis() as f32 / 1000.0;
		
		let last_tick_temp = Instant::now();
		let time_spent = (last_tick_temp - start).as_millis();

		test_besenham(&mut text_buffer.text, screen_width, screen_height, time_spent as i32);
		draw_triangles_wire(&screen_space_tris, &mut text_buffer.text, screen_width);
		
		poll_events(terminal);
		
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

		draw_benchmark(&mut text_buffer.text, screen_width, screen_height, &benchmark);
		terminal.draw(|frame| terminal_render(frame, &text_buffer)).unwrap();
	}
}


fn terminal_render<B: Backend>(frame: &mut Frame<B>, text: &FreeText) {
	let rect = frame.size();
	frame.render_widget(text, rect);
}

fn poll_events(terminal: &mut CTerminal) {
	let has_event = crossterm::event::poll(Duration::from_millis(0)).unwrap();
	if !has_event { return; }

	if let Event::Key(key) = event::read().unwrap() {
		match key.code {
    		KeyCode::Up    | KeyCode::Char('w') => quit_with_message(terminal, "Move Up"),
    		KeyCode::Left  | KeyCode::Char('a') => quit_with_message(terminal, "Move Left"),
    		KeyCode::Down  | KeyCode::Char('s') => quit_with_message(terminal, "Move Down"),
    		KeyCode::Right | KeyCode::Char('d') => quit_with_message(terminal, "Move Right"),
    		KeyCode::Char(ch) => quit_with_message(terminal, &format!("Needs to parse char {ch}")),
    		KeyCode::Esc => quit(terminal),
			_ => return,
		}
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