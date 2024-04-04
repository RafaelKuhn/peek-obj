use crate::UTF32_BYTES_PER_CHAR;



pub fn xy_to_it(x: u16, y: u16, width: u16) -> usize {
	let y_offset = y as usize * width as usize * UTF32_BYTES_PER_CHAR;
	y_offset + x as usize * UTF32_BYTES_PER_CHAR
}
