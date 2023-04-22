use std::{io::{self, Stdout}, time::{Duration, Instant}, process};
use tui::{
	backend::{CrosstermBackend, Backend},
	// widgets::{Widget, Block, Borders},
	// layout::{Layout, Constraint, Direction},
	Terminal, Frame, widgets::{Clear, StatefulWidget, Widget}, layout::Rect, buffer::Buffer,
};
use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

type CTerminal = Terminal<CrosstermBackend<Stdout>>;

// ascii luminance:
// . , - ~ : ; = ! & # @"
static LUMIN: &str = " .,-~:;=!&#@";


fn main() {
	let mut terminal: Terminal<_> = configure_terminal();
	Terminal::hide_cursor(&mut terminal).unwrap();
	// run_app(&mut terminal, Duration::from_millis(0));
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
fn run_app(terminal: &mut CTerminal) {
	let mut last_tick = Instant::now();
	let mut delta_time = Duration::from_millis(0);
	let mut frame_count: u32 = 0;

	// let text = FreeText { text: "a".to_string() };
	
	loop {
		// terminal.draw(|frame| render(frame, text, &delta_time)).unwrap();
		terminal.draw(|frame| render(frame, &delta_time)).unwrap();

		// poll_events(&timeout, terminal);
		// poll_events(&Duration::from_millis(0), terminal);
		poll_events(terminal);

		let last_tick_temp = Instant::now();
		delta_time = last_tick_temp - last_tick;
		last_tick = last_tick_temp;
	}
}

// fn render<B: Backend>(frame: &mut Frame<B>, free_text: FreeText, delta_time: &Duration) {
fn render<B: Backend>(frame: &mut Frame<B>, delta_time: &Duration) {
	let rect = frame.size();
	frame.render_widget(Clear, rect);
	
	// frame.render_stateful_widget(free_text, rect, &mut "a".to_owned());
	frame.render_widget(FreeText, rect);
	
	
	// print!("render {}! dt: {}, w: {}, h: {}  ", frame_count, delta_time.as_millis(), rect.width, rect.height);
	// print!("{:3}", delta_time.as_millis());
}

// fn poll_events(timeout: &Duration, terminal: &mut CTerminal) {
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
// pub struct FreeText {
// 	text: String,
// }
pub struct FreeText;

// impl StatefulWidget for FreeText {
// 	type State = String;

//     fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
//         for x in area.left()..area.right() {
//             for y in area.top()..area.bottom() {
//                 buf.get_mut(x, y).symbol = state.to_string();
//             }
//         }
//     }
// }


impl Widget for FreeText {
	fn render(self, area: Rect, buf: &mut Buffer) {

		let area_top: u16 = area.top();
		let area_bot: u16 = area.bottom();

		let half_width  = area.right() / 2;
		let half_heigth = area.bottom() / 2;

		// for y in half_heigth/2 .. half_heigth + half_heigth/2 {
		// 	for x in half_width/2 .. half_width + half_width/2 {
		// 		buf.get_mut(x, y).symbol = "a".to_string();
		// 	}
		// }

		// for y in half_heigth + half_heigth/2 .. area.bottom() {
		// 	for x in half_width + half_width/2 .. area.right() {
		// 		buf.get_mut(x, y).symbol = "a".to_string();
		// 	}
		// }

		let aspect = area.right() as f32 / area.bottom() as f32;

		let displacement = 5;
		for y in 0 .. area.bottom() {
			let yf = ((y as i16 - ((half_heigth/2) as i16 + (displacement as f32 * aspect) as i16)) as f32 + 0.5 ) * aspect;
			for x in 0 .. area.right() {
				let xf = f32::from(x as i16 - (half_width as i16 + displacement)) + 0.5;

				let len = f32::sqrt(xf * xf + yf * yf);
				
				if len < 32f32 {
					buf.get_mut(x, y).symbol = "a".to_string();
					continue;
				}
				buf.get_mut(x, y).symbol = ".".to_string();
				continue;
			}
		}


		// for y in area.top()+area.bottom()/4..area.bottom()/2 {
		// 	for x in area.left()..area.right()/2 {
		// 		buf.get_mut(x, y).symbol = "a".to_string();
		// 	}
		// }
	}
}