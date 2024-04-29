
use core::panic;
use std::{f32::consts::TAU, fs::File, io::{self, Stdout, Write}, process, time::Duration};

use crossterm::{
cursor::{Hide, MoveTo, Show}, event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers}, execute, queue, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen}, QueueableCommand
};

pub struct CrosstermTerminal {
	pub stdout: Stdout
}

use crate::{maths::*, render_clear, render_string, timer::Timer, try_saving_screenshot, App, ASCII_BYTES_PER_CHAR};


pub fn configure_terminal() -> CrosstermTerminal {
	enable_raw_mode().unwrap();

	let mut stdout = io::stdout();

	// enter alternate screen, hides cursor
	execute!(stdout, EnterAlternateScreen, Hide)
		.unwrap();

	CrosstermTerminal { stdout }
}

pub fn restore_terminal(terminal: &mut CrosstermTerminal) {
	restore_stdout(&mut terminal.stdout)
}

pub fn restore_stdout(stdout: &mut Stdout) {
	disable_raw_mode().unwrap();

	// leaves alternate screen, shows cursor
	execute!(stdout, LeaveAlternateScreen, Show)
		.unwrap();
}


pub fn poll_events(terminal: &mut CrosstermTerminal, app: &mut App, timer: &mut Timer) {

	// TODO: app.polled_data.reset() or something
	app.pos = Vec3::zero();
	app.rot = Vec3::zero();
	app.dir = Vec3::zero();
	app.called_reset_camera = false;
	app.called_take_screenshot = false;

	let has_event = crossterm::event::poll(Duration::from_millis(0)).unwrap();
	if !has_event { return }

	const MOVE_SPEED: f32 = 0.2;
	const ROT_SPEED: f32 = TAU * 1./128.;

	match event::read().unwrap() {
		Event::Key(key_evt) => {
			if key_evt.kind == KeyEventKind::Release { return }
			// app.buf.write_debug(&format!("{:?}\n", key_evt));

			match key_evt.code {
				// KeyCode::Backspace => app.buf.clear_debug(),

				// ↑ ← ↓ → rotates camera around Y and X axes
				KeyCode::Up    => app.rot.x = -ROT_SPEED,
				KeyCode::Down  => app.rot.x = ROT_SPEED,
				KeyCode::Left  => app.rot.y = -ROT_SPEED,
				KeyCode::Right => app.rot.y = ROT_SPEED,

				KeyCode::Esc => quit(terminal),

				KeyCode::Char(ch) => match ch.to_ascii_lowercase() {
					'm' => app.buf.test = !app.buf.test,
					// 'c' if key.modifiers.contains(KeyModifiers::CONTROL) => quit(terminal),
					'c' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),
					'q' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),

					// WASD moves left|right and forwards|backwards
					'w' => app.dir.z = -MOVE_SPEED,
					's' => app.dir.z = MOVE_SPEED,
					'd' => app.dir.x = -MOVE_SPEED,
					'a' => app.dir.x = MOVE_SPEED,
					// EQ moves camera up|down
					'e' => app.dir.y = MOVE_SPEED,
					'q' => app.dir.y = -MOVE_SPEED,

					// IK LJ UO moves camera along the Y X and Z axes
					'i' => app.pos.y = MOVE_SPEED,
					'k' => app.pos.y = -MOVE_SPEED,
					'l' => app.pos.x = MOVE_SPEED,
					'j' => app.pos.x = -MOVE_SPEED,
					'u' => app.pos.z = MOVE_SPEED,
					'o' => app.pos.z = -MOVE_SPEED,

					// R resets camera position, T takes screenshot, P pauses rendering
					'r' => app.called_reset_camera = true,
					't' => app.called_take_screenshot = true,
					'p' => {
							if app.has_paused_rendering {
							app.has_paused_rendering = false;
							timer.reset_time_scale();
						} else {
							app.has_paused_rendering = true;
							timer.time_scale = 0.0;
						}
					}
					_ => (),
				}
				_ => (),
			}
		}
		Event::Resize(new_width, new_height) => {
			app.resize_realloc(new_width, new_height);
			// I don't remember why we draw it immediately after resizing but ok
			print_and_flush_terminal_fscreen(&app.buf, terminal);
		}
		_ => (),
	}
}

pub fn just_poll_while_paused(app: &mut App, terminal_mut: &mut CrosstermTerminal, timer: &mut Timer) {

	if !app.has_paused_rendering { return; }

	let paused_str = "PAUSED!";
	render_string(paused_str, &UVec2::new(app.buf.wid - paused_str.len() as u16, app.buf.hei - 1), &mut app.buf);
	print_and_flush_terminal_fscreen(&app.buf, terminal_mut);

	while app.has_paused_rendering {
		poll_events(terminal_mut, app, timer);
		timer.run();
		try_saving_screenshot(app, &timer);
	};
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
	// width / height of the terminal in characters
	pub wid: u16,
	pub hei: u16,
	pub vec: Vec<u8>,

	// unique, just gets popuplated once per frame
	proj_mat: Vec<f32>,

	// reused across different rendered objects, mut be cleaned
	pub transf_mat: Vec<f32>,
	pub render_mat: Vec<f32>,

	debug_file: Option<File>,

	pub test: bool,
}

impl TerminalBuffer {
	pub fn new(w: u16, h: u16) -> Self {

		let char_len = w as usize * h as usize * ASCII_BYTES_PER_CHAR;
		let vec = vec![0; char_len];

		let debug_file = File::create(Self::DEBUG_FILE_PATH).ok();

		let mut this = TerminalBuffer {
			wid: w,
			hei: h,
			vec,
			proj_mat: create_identity_4x4(),
			transf_mat: create_identity_4x4(),
			render_mat: create_identity_4x4(),
			debug_file,
			test: false,
		};

		render_clear(&mut this);
		this
	}

	pub fn resize_and_render_clear(&mut self, w: u16, h: u16) {

		self.wid = w;
		self.hei = h;

		let char_len = w as usize * h as usize * ASCII_BYTES_PER_CHAR;
		self.vec.clear();
		self.vec.resize(char_len, 0);

		render_clear(self);
	}

	// pub fn update_projection(&mut self, wid: u16, hei: u16) {
	pub fn update_proj_matrix(&mut self) {
		apply_identity_to_mat_4x4(&mut self.proj_mat);
		apply_projection_to_mat_4x4(&mut self.proj_mat, self.wid, self.hei);
	}

	pub fn reset_render_matrix(&mut self) {
		apply_identity_to_mat_4x4(&mut self.render_mat);
	}

	pub fn copy_projection_to_render_matrix(&mut self) {
		self.render_mat.copy_from_slice(&self.proj_mat);
	}

	pub fn copy_projection_to_mat4x4(&self, dst: &mut [f32]) {
		dst.copy_from_slice(&self.proj_mat);
	}
	
	const SCREENSHOT_PATH: &str = "screenshot.txt";
	pub fn try_dump_buffer_content_to_file(&mut self) {

		let file_result = File::create(Self::SCREENSHOT_PATH);

		if let Err(error) = file_result {
			panic!("Error saving screenshot: {}!", error);
			return;
		}

		let mut screenshot_file = file_result.unwrap();

		for y in 0..self.hei {

			let y_start = y       as usize * self.wid as usize * ASCII_BYTES_PER_CHAR;
			let y_end   = (y + 1) as usize * self.wid as usize * ASCII_BYTES_PER_CHAR;

			let buf_str = std::str::from_utf8(&self.vec[y_start .. y_end]).unwrap();

			let _ = screenshot_file.write(buf_str.as_bytes());
			let _ = screenshot_file.write(&['\n' as u8]);
		}
	}

	const DEBUG_FILE_PATH: &str = "bullshit/_debug";
	pub fn clear_debug(&mut self) {
		self.debug_file = File::create(Self::DEBUG_FILE_PATH).ok();
	}

	pub fn write_debug(&mut self, string: &str) {
		if let Some(ref mut file) = &mut self.debug_file {
			file.write(string.as_bytes()).expect("shit, couldn't write to file");
		}
	}

	pub fn write_debug2(&mut self, string: &str) {
		if let None = &mut self.debug_file { return }
		let file = self.debug_file.as_mut().unwrap();
		file.write(string.as_bytes()).expect("shit, couldn't write to file");
	}

	pub fn write_debug3(&mut self, string: &str) {
		match self.debug_file.as_mut() {
			Some(file) => { file.write(string.as_bytes()).expect("shit, couldn't write to file"); },
			None => (),
		}
	}

	pub fn write_debug4(&mut self, string: &str) {
		if self.debug_file.is_none() { return }

		self.debug_file.as_mut().unwrap().write(string.as_bytes()).expect("shit, couldn't write to file");
	}

	pub fn write_debug5(&mut self, string: &str) {
		if self.debug_file.is_some() {
			self.debug_file.as_mut().unwrap().write(string.as_bytes()).expect("shit, couldn't write to file");
		}
	}

	pub fn write_debug6(&mut self, string: &str) -> Option<()> {
		let file = self.debug_file.as_mut()?;
		file.write(string.as_bytes()).expect("shit, couldn't write to file");
		Some(())
	}

}

pub fn print_and_flush_terminal_fscreen(buf: &TerminalBuffer, terminal: &mut CrosstermTerminal) {
	// print_and_flush_terminal_line_by_line(buf,terminal); return;

	// let buf_str = unsafe { std::str::from_utf8_unchecked(&buf.vec) };
	let buf_str = std::str::from_utf8(&buf.vec).unwrap();
	queue!(terminal.stdout, MoveTo(0, 0), Hide, Print(buf_str)).unwrap();
	terminal.stdout.flush().unwrap();
}

pub fn print_and_flush_terminal_line_by_line(buf: &TerminalBuffer, terminal: &mut CrosstermTerminal) {
	// line by line, this is required for "init with custom width/height"

	let buf_wid = buf.wid as usize;
	for y in 0..buf.hei {

		let y_start = y as usize * buf_wid;
		let y_end   = y_start + buf_wid;

		// let buf_str = unsafe { std::str::from_utf8_unchecked(&buf.vec[y_start..y_end]) };
		let buf_str = std::str::from_utf8(&buf.vec[y_start .. y_end]).unwrap();

		// terminal.stdout.queue(Hide).unwrap();
		queue!(terminal.stdout, MoveTo(0, y), Print(buf_str), Hide).unwrap();
	}

	terminal.stdout.flush().unwrap();
}