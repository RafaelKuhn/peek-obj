use std::{f32::consts::TAU, io::{self, Stdout, Write}, process, time::Duration};

use crossterm::{terminal::*, event::*, cursor::*, style::*, *};


// TODO: refactor this whole shit to use methods, don't be afraid of methods, they are not OOP!

pub struct CrosstermTerminal {
	pub stdout: Stdout
	// pub stdout: BufWriter<Stderr>
	// pub stdout: BufWriter<File>,
}

use crate::{maths::*, render_clear, render_help_screen, render_string_snap_right, timer::Timer, App, TerminalBuffer};


pub fn configure_terminal() -> CrosstermTerminal {
	enable_raw_mode().unwrap();

	let mut stdout = io::stdout();
	// let mut stdout = io::stderr();

	// enter alternate screen, hides cursor
	execute!(stdout, EnterAlternateScreen, Hide)
		.unwrap();

	CrosstermTerminal { stdout: stdout }
	// CrosstermTerminal { stdout: BufWriter::new(stdout) }
	// CrosstermTerminal { stdout: File::create("bullshit/_dump").map(BufWriter::new).ok().unwrap() }
}

pub fn set_panic_hook() {
	let hook = std::panic::take_hook();
	std::panic::set_hook(Box::new(move |info| {
		restore_stdout(&mut io::stdout());

		hook(info);
		restore_stdout(&mut io::stdout());
	}));
}

pub fn poll_for_pause(terminal: &mut CrosstermTerminal, app: &mut App, timer: &mut Timer) {
	let has_event = crossterm::event::poll(Duration::from_millis(0)).unwrap();
	if !has_event { return }

	match event::read().unwrap() {
		Event::Key(key_evt) => {
			if key_evt.kind == KeyEventKind::Release { return }

			match key_evt.code {
				KeyCode::Esc => quit(terminal),

				KeyCode::Char(ch) => match ch.to_ascii_lowercase() {
					'c' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),
					'q' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),
					'p' if key_evt.modifiers == KeyModifiers::NONE => app.toggle_pause_full(timer),
					_ => (),
				}
				_ => (),
			}
		},
		Event::Resize(new_width, new_height) => {
			app.resize_realloc(new_width, new_height);
			// I don't remember why we draw it immediately after resizing but ok
			print_and_flush_terminal_fscreen(&mut app.buf, terminal);
		}
		_ => (),
	}
}

pub fn poll_in_help_screen(terminal: &mut CrosstermTerminal, app: &mut App) {
	let has_event = crossterm::event::poll(Duration::from_millis(0)).unwrap();
	if !has_event { return }

	match event::read().unwrap() {
		Event::Key(key_evt) => {
			if key_evt.kind == KeyEventKind::Release { return }

			match key_evt.code {
				KeyCode::Esc => quit(terminal),

				KeyCode::Char(ch) => match ch.to_ascii_lowercase() {
					'c' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),
					'q' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),
					'h' => app.is_help_screen = false,
					_ => (),
				}
				_ => (),
			}
		},
		Event::Resize(new_width, new_height) => {
			app.resize_realloc(new_width, new_height);

			render_clear(&mut app.buf);
			render_help_screen(&mut app.buf);

			// I don't remember why we draw it immediately after resizing but ok
			print_and_flush_terminal_fscreen(&mut app.buf, terminal);
		}
		_ => (),
	}
}

// TODO: polling channels or some sort, for instance, when paused, only poll for unpause
//       could always listen to crucial events like Ctrl+C and resize, but vary the others, hash table of 'KeyCode' or smth
// TODO: make it an impl of CrosstermTerminal
pub fn poll_events(terminal: &mut CrosstermTerminal, app: &mut App, timer: &mut Timer) {

	// TODO: app.polled_data.reset() or something
	app.user_rot = Vec3::zero();
	app.user_dir = Vec3::zero();
	app.called_reset_camera = false;

	const MOVE_SPEED: f32 = 0.2;
	const ROT_SPEED: f32 = TAU * 1./256.;

	let has_event = crossterm::event::poll(Duration::from_millis(0)).unwrap();
	if !has_event { return }

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
					'o' => app.buf.test = !app.buf.test,
					'i' if key_evt.modifiers == KeyModifiers::SHIFT => app.buf.test_i -= 1,
					'i' => app.buf.test_i += 1,

					'c' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),
					'q' if key_evt.modifiers == KeyModifiers::CONTROL => quit(terminal),

					// WASD moves left|right and forwards|backwards
					'w' => app.user_dir.z = -MOVE_SPEED,
					's' => app.user_dir.z = MOVE_SPEED,
					'd' => app.user_dir.x = if app.is_free_mov() { -MOVE_SPEED } else {  ROT_SPEED },
					'a' => app.user_dir.x = if app.is_free_mov() {  MOVE_SPEED } else { -ROT_SPEED },
					// EQ moves camera up|down
					'e' => app.user_dir.y = if app.is_free_mov() {  MOVE_SPEED } else { -ROT_SPEED },
					'q' => app.user_dir.y = if app.is_free_mov() { -MOVE_SPEED } else {  ROT_SPEED },

					'z' if key_evt.modifiers == KeyModifiers::SHIFT => app.buf.toggle_back_z_sorting_mode(),
					'z' => app.buf.toggle_z_sorting_mode(),
					'c' if key_evt.modifiers == KeyModifiers::SHIFT => app.buf.toggle_back_cull_mode(),
					'c' => app.buf.toggle_cull_mode(),
					'l' if key_evt.modifiers == KeyModifiers::SHIFT => app.buf.toggle_back_ball_fill_mode(),
					'l' => app.buf.toggle_ball_fill_mode(),
					'g' => app.buf.toggle_gizmos_mode(),
					'm' => app.called_toggle_free_mov = true,
					'v' => app.is_verbose = !app.is_verbose,

					// R resets camera position to default, shift+R sets the default
					'r' if key_evt.modifiers == KeyModifiers::SHIFT => app.called_set_camera_default_orientation = true,
					'r' => app.called_reset_camera = true,

					// T to take screenshot
					't' => app.called_take_screenshot = true,

					// P pauses the engine, shift+P pauses time (time scale)
					'p' if key_evt.modifiers == KeyModifiers::SHIFT => app.toggle_pause_anim(timer),
					// TODO: call functions ".pause" and ".unpause"
					'p' => app.toggle_pause_full(timer),

					// H to enter help screen
					'h' => app.is_help_screen = true,
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

pub fn yield_while_paused_or_help_screen(app: &mut App, terminal: &mut CrosstermTerminal, timer: &mut Timer) {


	if app.is_help_screen {
		render_clear(&mut app.buf);
		render_help_screen(&mut app.buf);
		print_and_flush_terminal_fscreen(&mut app.buf, terminal);

		while app.is_help_screen {
			poll_in_help_screen(terminal, app);
		}
	}

	if app.is_fully_paused() {
		const PAUSED_STR: &str = " ENGINE PAUSED! ";
		render_string_snap_right(PAUSED_STR, &UVec2::new(0, app.buf.hei - 1), &mut app.buf);
		print_and_flush_terminal_fscreen(&mut app.buf, terminal);

		while app.is_fully_paused() {
			poll_for_pause(terminal, app, timer);
			timer.run_tick();
		};
	}
}

pub fn print_and_flush_terminal_fscreen(buf: &mut TerminalBuffer, terminal: &mut CrosstermTerminal) {

	// Does not work, trying to write only what's necessary
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

	// let buf_str = unsafe { std::str::from_utf8_unchecked(&buf.vec) };
	let buf_str = std::str::from_utf8(&buf.raw_ascii_screen).unwrap();
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
		let buf_str = std::str::from_utf8(&buf.raw_ascii_screen[y_start .. y_end]).unwrap();

		// terminal.stdout.queue(Hide).unwrap();
		queue!(terminal.stdout, MoveTo(0, y), Print(buf_str), Hide).unwrap();
	}

	terminal.stdout.flush().unwrap();
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

fn restore_terminal(terminal: &mut CrosstermTerminal) {
	restore_stdout(&mut terminal.stdout)
}

fn restore_stdout<T: Write>(stdout: &mut T) {
	disable_raw_mode().unwrap();

	// leaves alternate screen, shows cursor
	execute!(stdout, LeaveAlternateScreen, Show)
		.unwrap();
}