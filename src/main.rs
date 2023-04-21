use std::{io::{self, Stdout}};
use tui::{
	backend::{CrosstermBackend, Backend},
	// widgets::{Widget, Block, Borders},
	// layout::{Layout, Constraint, Direction},
	Terminal, Frame
};
use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
	execute,
	terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() {
	let mut terminal: Terminal<_> = configure_terminal().unwrap();

	run(&mut terminal);

	restore_terminal(&mut terminal);
}

fn configure_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
	enable_raw_mode().unwrap();
	let mut stdout = io::stdout();
	execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
	let backend = CrosstermBackend::new(stdout);
	return Terminal::new(backend);
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>){
	disable_raw_mode().unwrap();
	execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
	terminal.show_cursor().unwrap();
}


fn run<B: Backend>(terminal: &mut Terminal<B>) {
	println!("press ESC to quit");
	loop {
		terminal.draw(render).unwrap();
		let event = event::read().unwrap();
		let Event::Key(key) = event else { continue; };

		match key.code {
			KeyCode::Esc => return,
			_ => continue,
		}
	}
}

fn render<B: Backend>(frame: &mut Frame<B>) {
	let size = frame.size();
	println!("render! w: {}, h: {}", size.width, size.height);
}
