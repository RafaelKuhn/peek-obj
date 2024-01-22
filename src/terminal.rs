use std::{io::{self, Stdout}, time::Duration, process};

use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
	backend::{CrosstermBackend, Backend},
	Terminal, Frame, layout::Rect, widgets::Widget, buffer::Buffer,
};

type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

use crate::{rendering, App, timer::AppTimer};


pub fn configure_terminal() -> CrosstermTerminal {
	enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

	let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();
	Terminal::hide_cursor(&mut terminal).unwrap();

	terminal
}

pub fn restore_terminal(terminal: &mut CrosstermTerminal) {
	disable_raw_mode().unwrap();
	execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
	terminal.show_cursor().unwrap();
}

pub fn render_free_text<B: Backend>(frame: &mut Frame<B>, text: &FreeText) {
	let rect = frame.size();
	frame.render_widget(text, rect);
}

pub fn poll_events(terminal: &mut CrosstermTerminal, app: &mut App, timer: &mut AppTimer) {
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

pub fn get_terminal_width_height(terminal: &CrosstermTerminal) -> (u16, u16) {
	let Rect { width, height, .. } = terminal.size().unwrap();
	(width, height)
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




#[derive(Debug)]
pub struct FreeText {
	pub text: Vec<char>,
}

impl FreeText {
	pub fn from_screen(screen_width: u16, screen_height: u16) -> Self {
		// I have no fucking clue why but I need to add 1 here
		let length = (screen_width + 1) as usize * (screen_height + 1) as usize;
		Self::from_chars(rendering::BACKGROUND_FILL_CHAR, length)
	}

	fn from_chars(char: char, length: usize) -> Self {
		Self {
			text: vec![char; length],
		}
	}
}


impl Widget for &FreeText {
	fn render(self, area: Rect, buf: &mut Buffer) {

		let x_start = 0;
		let y_start = 0;

		let area_bottom = area.bottom();
		let area_right  = area.right();

		// TODO: figure out
		// debug_assert!((area_bottom+1) * (area_right+1) == self.text.len() as u16,
		// 	"{}",
		// 	format!(": {}*{}={} != {}", area_bottom+1, area_right+1, (area_bottom+1)*(area_right+1), self.text.len()));

		let mut char_i = 0;
		for y in y_start..area_bottom {
			for x in x_start..area_right {

				// TODO: debug_assert, dont try drawing smth thats off
				if let Some(ch) = self.text.get(char_i) {
					buf.get_mut(x, y).set_char(*ch);
				}

				// buf.get_mut(x, y).set_char(self.text[char_i]);

				char_i += 1;
			}
		}
	}
}
