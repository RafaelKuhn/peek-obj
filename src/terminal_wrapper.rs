
use std::{io::{self, Stdout, Write}, process, time::Duration};

use crossterm::{
cursor::{Hide, MoveTo, Show}, event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode}, execute, queue, style::Print, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, QueueableCommand
};

pub struct CrosstermTerminal {
	pub stdout: Stdout
}

use crate::{maths::build_identity_4x4, render_clear, timer::Timer, App, UTF32_BYTES_PER_CHAR};


pub fn configure_terminal() -> CrosstermTerminal {
	enable_raw_mode().unwrap();

	let mut stdout = io::stdout();

	// enter alternate screen, hides cursor
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Hide).unwrap();

	CrosstermTerminal { stdout }
}

pub fn restore_terminal(terminal: &mut CrosstermTerminal) {
	disable_raw_mode().unwrap();

	// leaves alternate screen, shows cursor
	execute!(terminal.stdout, LeaveAlternateScreen, DisableMouseCapture, Show).unwrap();
}

pub fn poll_events(terminal: &mut CrosstermTerminal, app: &mut App, timer: &mut Timer) {
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
				KeyCode::Esc | KeyCode::Char('q') => quit(terminal),
				_ => (),
			}
		}
		Event::Resize(new_width, new_height) => {
			app.resize_realloc(new_width, new_height);
			queue_draw_to_terminal_and_flush(&app.buf, terminal);
		}
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


pub struct TerminalBuffer {
	pub wid: u16,
	pub hei: u16,
	pub vec: Vec<u8>,
	pub proj_mat: Vec<f32>,
	pub transf_mat: Vec<f32>,
}

impl TerminalBuffer {
	pub fn new(w: u16, h: u16) -> Self {

		let char_len = w as usize * h as usize * UTF32_BYTES_PER_CHAR;
		let vec = vec![0; char_len];

		let mut this = TerminalBuffer {
			wid: w,
			hei: h,
			vec,
			proj_mat: build_identity_4x4(),
			transf_mat: build_identity_4x4(),
		};

		render_clear(&mut this);
		this
	}

	pub fn resize_and_render_clear(&mut self, w: u16, h: u16) {

		self.wid = w;
		self.hei = h;

		let char_len = w as usize * h as usize * UTF32_BYTES_PER_CHAR;
		self.vec.clear();
		self.vec.resize(char_len, 0);

		render_clear(self);
	}

}

pub fn queue_draw_to_terminal_and_flush(buf: &TerminalBuffer, terminal: &mut CrosstermTerminal) {

	// let buf_str = unsafe { std::str::from_utf8_unchecked(&buf.vec) };
	let buf_str = std::str::from_utf8(&buf.vec).unwrap();

	queue!(terminal.stdout, MoveTo(0, 0), Hide, Print(buf_str)).unwrap();

	terminal.stdout.flush().unwrap();


	// // line by line sys calls
	// for y in 0..buf.hei {

	// 	terminal.stdout.queue(MoveTo(0, y)).unwrap();
	// 	let y_start = y       as usize * buf.wid as usize * UTF32_BYTES_PER_CHAR;
	// 	let y_end   = (y + 1) as usize * buf.wid as usize * UTF32_BYTES_PER_CHAR;

	// 	let buf_str = unsafe { std::str::from_utf8_unchecked(&buf.vec[y_start..y_end]) };
	// 	// let buf_str = std::str::from_utf8(&buf.vec[y_start .. y_end]).unwrap();

	// 	terminal.stdout.queue(Hide).unwrap();
	// 	terminal.stdout.queue(Print(buf_str)).unwrap();
	// }

	// terminal.stdout.flush().unwrap();
}
