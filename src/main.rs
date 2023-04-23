use std::{io::{self, Stdout}, time::{Duration, Instant}, process};

use peekfbx::rendering::{mesh::Mesh, draw_triangles, ScreenXY};
use tui::{
	backend::{CrosstermBackend, Backend},
	Terminal, Frame, widgets::{Clear, Widget}, layout::Rect, buffer::Buffer,
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


// fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) {
fn run_app(terminal: &mut CTerminal) -> ! {
	let mut last_tick = Instant::now();
	let mut delta_time = Duration::from_millis(0);
	let mut frame_count: u32 = 0;

	let Rect { width, height, .. } = terminal.size().unwrap();
	// let chars_size = ((height - 1) * (width - 1)) as usize;
	let chars_size = (height * width) as usize;

	let mut text_buffer = FreeText::from_text(str::repeat(" ", chars_size));
	
	let mesh = Mesh::cube();

	let screen_space_tris = vec![
		ScreenXY { x: width/2,   y: height/2 },
		ScreenXY { x: width/2,   y: height/2+5 },
		ScreenXY { x: width/2-5, y: height/2+5 },
	];

	loop {
		// render_into_buffer(&mut text_buffer.text, &mesh, width, height);
		draw_triangles(&screen_space_tris, &mut text_buffer.text, width, height);

		// terminal.draw(|frame| terminal_render(frame, &delta_time, &text_buffer)).unwrap();
		terminal.draw(|frame| terminal_render(frame, &text_buffer)).unwrap();

		poll_events(terminal);

		let last_tick_temp = Instant::now();
		delta_time = last_tick_temp - last_tick;
		last_tick = last_tick_temp;
	}
}



fn terminal_render<B: Backend>(frame: &mut Frame<B>, text: &FreeText) {
	let rect = frame.size();
	frame.render_widget(Clear, rect);
	frame.render_widget(text, rect);
}

fn poll_events(terminal: &mut CTerminal) {
	// let has_event = crossterm::event::poll(*timeout).unwrap();
	let has_event = crossterm::event::poll(Duration::from_millis(0)).unwrap();
	if !has_event { return; }

	if let Event::Key(key) = event::read().unwrap() {
		if let KeyCode::Esc = key.code { quit(terminal); }
	}
}

fn quit(terminal: &mut CTerminal) {
	restore_terminal(terminal);
	process::exit(0);
}


	
#[derive(Debug)]
pub struct FreeText {
	text: Vec<char>,
	// TODO:
	// width:  u16,
	// height: u16,
}

impl FreeText {
	pub fn from_text(text: String) -> Self {
		Self {
			text: text.chars().collect(),
		}
	}
}

impl Widget for &FreeText {
	fn render(self, area: Rect, buf: &mut Buffer) {

		let area_botom: u16 = area.bottom();
		let area_right: u16 = area.right();

		let x_start = 0;
		let y_start = 0;
		let aspect = area.right() as f32 / area.bottom() as f32;

		let mut char_i = 0;
		for y in y_start .. area_botom {
			for x in x_start .. area_right {
				// buf.get_mut(x, y).symbol = "a".to_string();
				/* actually right - left (0) */
				// let char_i = (y * (area_right) + x) as usize;
				
				if let Some(ch) =  self.text.get(char_i) {
					buf.get_mut(x, y).set_char(*ch);
				}

				// buf.get_mut(x, y).set_char(self.text[char_i] as char);
				// buf.get_mut(x, y).set_char(self.text[char_i]);

				char_i += 1;
			}
		}

		// for y in half_heigth + half_heigth/2 .. area.bottom() {
		// 	for x in half_width + half_width/2 .. area.right() {
		// 		buf.get_mut(x, y).symbol = "a".to_string();
		// 	}
		// }

		// buf = Buffer::set_string(&mut self, x, y, string, style)


		// let displacement = 5;
		// for y in 0 .. area.bottom() {
		// 	let yf = ((y as i16 - ((half_heigth/2) as i16 + (displacement as f32 * aspect) as i16)) as f32 + 0.5 ) * aspect;
		// 	for x in 0 .. area.right() {
		// 		let xf = f32::from(x as i16 - (half_width as i16 + displacement)) + 0.5;

		// 		let len = f32::sqrt(xf * xf + yf * yf);
				
		// 		if len < 32f32 {
		// 			buf.get_mut(x, y).symbol = "a".to_string();
		// 			continue;
		// 		}
		// 		buf.get_mut(x, y).symbol = ".".to_string();
		// 		continue;
		// 	}
		// }

		// buf.get_mut(area.right()/2, area.bottom()/2).set_char(self.text.chars().nth(0).unwrap());

		// for y in area.top()+area.bottom()/4..area.bottom()/2 {
		// 	for x in area.left()..area.right()/2 {
		// 		buf.get_mut(x, y).symbol = "a".to_string();
		// 	}
		// }
	}
}