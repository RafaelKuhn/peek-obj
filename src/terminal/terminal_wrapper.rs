use std::{f32::consts::TAU, io::{self, BufWriter, Stderr, Stdout, Write}, process, time::Duration};

use crossterm::{terminal::*, event::*, cursor::*, style::*, *};

pub struct CrosstermTerminal {
	pub stdout: Stdout
	// pub stdout: BufWriter<Stderr>
}

use crate::{maths::*, render_string, timer::Timer, try_saving_screenshot, App, TerminalBuffer};


pub fn configure_terminal() -> CrosstermTerminal {
	enable_raw_mode().unwrap();

	let mut stdout = io::stdout();
	// let mut stdout = io::stderr();

	// enter alternate screen, hides cursor
	execute!(stdout, EnterAlternateScreen, Hide)
		.unwrap();

	CrosstermTerminal { stdout: stdout }
	// CrosstermTerminal { stdout: BufWriter::new(stdout) }
}

pub fn restore_terminal(terminal: &mut CrosstermTerminal) {
	restore_stdout(&mut terminal.stdout)
}

pub fn restore_stdout<T: Write>(stdout: &mut T) {
	disable_raw_mode().unwrap();

	// leaves alternate screen, shows cursor
	execute!(stdout, LeaveAlternateScreen, Show)
		.unwrap();
}


pub fn poll_events(terminal: &mut CrosstermTerminal, app: &mut App, timer: &mut Timer) {

	// TODO: app.polled_data.reset() or something
	app.user_pos = Vec3::zero();
	app.user_rot = Vec3::zero();
	app.user_dir = Vec3::zero();
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
				KeyCode::Up    => app.user_rot.x = -ROT_SPEED,
				KeyCode::Down  => app.user_rot.x = ROT_SPEED,
				KeyCode::Left  => app.user_rot.y = -ROT_SPEED,
				KeyCode::Right => app.user_rot.y = ROT_SPEED,

				KeyCode::Esc => quit(terminal),

				KeyCode::Char(ch) => match ch.to_ascii_lowercase() {
					'm' => app.buf.test = !app.buf.test,
					'c' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),
					'q' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),

					// WASD moves left|right and forwards|backwards
					'w' => app.user_dir.z = -MOVE_SPEED,
					's' => app.user_dir.z = MOVE_SPEED,
					'd' => app.user_dir.x = -MOVE_SPEED,
					'a' => app.user_dir.x = MOVE_SPEED,
					// EQ moves camera up|down
					'e' => app.user_dir.y = MOVE_SPEED,
					'q' => app.user_dir.y = -MOVE_SPEED,

					// XYZ moves along the XYZ axes, shift+XYZ moves back
					'y' if key_evt.modifiers == KeyModifiers::SHIFT => app.user_pos.y = -MOVE_SPEED,
					'y' => app.user_pos.y = MOVE_SPEED,
					'x' if key_evt.modifiers == KeyModifiers::SHIFT => app.user_pos.x = -MOVE_SPEED,
					'x' => app.user_pos.x = MOVE_SPEED,
					'z' if key_evt.modifiers == KeyModifiers::SHIFT => app.user_pos.z = -MOVE_SPEED,
					'z' => app.user_pos.z = MOVE_SPEED,

					// R resets camera position to default, shift+R sets the default
					'r' if key_evt.modifiers == KeyModifiers::SHIFT => app.called_set_camera_default_orientation = true,
					'r' => app.called_reset_camera = true,
					// T takes screenshot, P pauses rendering
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
			print_and_flush_terminal_fscreen(&mut app.buf, terminal);
		}
		_ => (),
	}
}

pub fn just_poll_while_paused(app: &mut App, terminal_mut: &mut CrosstermTerminal, timer: &mut Timer) {

	if !app.has_paused_rendering { return; }

	const PAUSED_STR: &str = "PAUSED!";
	render_string(PAUSED_STR, &UVec2::new(app.buf.wid - PAUSED_STR.len() as u16, app.buf.hei - 1), &mut app.buf);
	print_and_flush_terminal_fscreen(&mut app.buf, terminal_mut);

	while app.has_paused_rendering {
		poll_events(terminal_mut, app, timer);
		timer.run();
		try_saving_screenshot(app, timer);
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

pub fn print_and_flush_terminal_fscreen(buf: &mut TerminalBuffer, terminal: &mut CrosstermTerminal) {

	// for y in 0..buf.hei {
	// 	for x in 0..buf.wid {
	// 		let i = xy_to_it(x, y, buf.wid);
	// 		let newu = buf.vec[i];
	// 		let last = buf.last_frame_vec[i];

	// 		if newu != last {
	// 			queue!(terminal.stdout, MoveTo(x, y), Print(newu as char)).unwrap();
	// 		}
	// 	}
	// }

	// terminal.stdout.flush().unwrap();
	// buf.last_frame_vec.copy_from_slice(&buf.vec);
	// return;

	// for y in buf.hei/3..buf.hei/2 {
	// 	for x in buf.wid/3..buf.wid/2 {
	// 		buf.vec[xy_to_it(x, y, buf.wid)] = b'\0';
	// 	}
	// }

	// let buf_str = unsafe { std::str::from_utf8_unchecked(&buf.vec) };
	let buf_str = std::str::from_utf8(&buf.vec).unwrap();
	queue!(terminal.stdout, MoveTo(0, 0), Hide, Print(buf_str)).unwrap();

	terminal.stdout.flush().unwrap();
}

pub fn print_and_flush_terminal_line_by_line(buf: &mut TerminalBuffer, terminal: &mut CrosstermTerminal) {
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