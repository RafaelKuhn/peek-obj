pub mod mesh;

// ascii luminance:
// . , - ~ : ; = ! & # @"
pub static LUMIN: &str = " .,-~:;=!&#@";


// pub fn render_into_buffer(buffer: &mut String, mesh: &Mesh, w: u16, h: u16) {
// 	todo!()
// }

// fn project_mesh() { 
// 	todo!()
// }

#[derive(Debug)]
pub struct ScreenXY {
	pub x: u16,
	pub y: u16,
}
// pub struct vec2 {
// 	pub x: f32,
// 	pub y: f32,
// }

pub fn draw_string(str: &str, pos: ScreenXY, buffer: &mut Vec<char>, width: u16) {
	let mut index = (pos.y * width + pos.x) as usize;
	for ch in str.chars() {
		buffer[index] = ch;
		index += 1;
	}
}

pub fn draw_triangles(screen_space_tris: &Vec<ScreenXY>, buffer: &mut Vec<char>, width: u16, height: u16) {
	let mut index: usize;
	for (i, tri) in screen_space_tris.iter().enumerate() {
		index = (tri.y * width + tri.x) as usize;
		buffer[index] = 'a';
		draw_string(format!("triangle coord should be at {tri:?}").as_str(), ScreenXY { x: 4, y: 2 + (i as u16) }, buffer, width);
	}
}
