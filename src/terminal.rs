use tui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::rendering;


#[derive(Debug)]
pub struct FreeText {
	pub text: Vec<char>,
	// bg: char,
	// TODO:
	// width:  u16,
	// height: u16,
}

impl FreeText {
	pub fn from_screen(screen_width: u16, screen_height: u16) -> Self {
		let length = screen_width as usize * screen_height as usize;
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
		let area_right = area.right();
		
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
