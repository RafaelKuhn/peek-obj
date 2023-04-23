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

		// example 0
		// match event::read().unwrap() {
		// 	Event::Key(key) => {
		// 		match key.code {
		// 			KeyCode::Esc => return,
		// 			KeyCode::Char(ch) => print!("{}", ch),
		// 			_ => continue,
		// 		}
		// 	},
		// 	_ => continue,	
		// }
		// return;

		// example 1
		let event = event::read().unwrap();
		let Event::Key(key) = event else { continue; };

		// example 0
		// if let KeyCode::Esc = key.code { return; }
		// let KeyCode::Char(ch) = key.code else { continue };
		// print!("{}", ch);

		// example 1
		// if let KeyCode::Esc = key.code { return; }
		// if let KeyCode::Char(ch) = key.code {
		// 	print!("{}", ch);
		// }
		// continue;
		
		// example 2
		match key.code {
			KeyCode::Esc => return,
			KeyCode::Char(ch) => print!("{}", ch),
			_ => continue,
		}
	}
}

fn render<B: Backend>(arg: &mut Frame<B>) {
	// println!("render!");
}


// use std::io;
// use tui::{backend::CrosstermBackend, Terminal};

// fn main() -> Result<(), io::Error> {
// 	let stdout = io::stdout();
// 	let backend = CrosstermBackend::new(stdout);
// 	let mut terminal = Terminal::new(backend)?;
// 	Ok(())
// }
