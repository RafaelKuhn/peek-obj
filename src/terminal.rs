use tui::{buffer::Buffer, layout::Rect, widgets::Widget};


#[derive(Debug)]
pub struct FreeText {
	pub text: Vec<char>,
	// bg: char,
	// TODO:
	// width:  u16,
	// height: u16,
}

impl FreeText {
	pub fn from_chars(char: char, length: usize) -> Self {
		Self {
			text: vec![char; length],
			// bg: char,
		}
	}

	// pub fn from_text(text: String) -> Self {
	// 	Self {
	// 		text: text.chars().collect(),
	// 	}
	// }

}

impl Widget for &FreeText {
	fn render(self, area: Rect, buf: &mut Buffer) {

		let x_start = 0;
		let y_start = 0;

		let area_botom = area.bottom();
		let area_right = area.right();

		// TODO: render clear
		// for i in 0..self.text.len() {
		// 	let ve = &mut self.text;
		// 	ve[i] = self.bg;
		// }

		// for y in y_start .. area_botom {
		// 	for x in x_start .. area_right {
        //         buf.get_mut(x, y).reset();
        //     }
        // }
		
		// let aspect = area.right() as f32 / area.bottom() as f32;

		let mut char_i = 0;
		for y in y_start .. area_botom {
			for x in x_start .. area_right {

				// TODO: debug_ass
				if let Some(ch) = self.text.get(char_i) {
					buf.get_mut(x, y).set_char(*ch);
				}

				// buf.get_mut(x, y).set_char(self.text[char_i] as char);
				// buf.get_mut(x, y).set_char(self.text[char_i]);

				char_i += 1;
			}
		}
	}
}