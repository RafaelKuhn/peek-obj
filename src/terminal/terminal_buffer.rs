use std::{fmt::Debug, fs::File, io::{BufWriter, Write}};

use crate::{cull_mode::CullMode, maths::*, render_clear, render_settings::*, ASCII_BYTES_PER_CHAR};

// type DebugFile = File;
type DebugFile = BufWriter<File>;

pub struct TerminalBuffer {
	// width / height of the terminal in characters
	pub wid: u16,
	pub hei: u16,

	// global output buffer
	pub raw_ascii_screen: Vec<u8>,

	// unique 4x4 matrix buffers, reused across different rendered objects, mut be cleaned after each used
	proj_mat: Vec<f32>,
	pub transf_mat: Vec<f32>,
	pub render_mat: Vec<f32>,

	sorting_mode:   ZSortingMode,
	cull_mask:      CullMode,
	ball_fill_mode: BallFillMode,
	gizmos_mode:    GizmosType,

	debug_file: Option<DebugFile>,

	// this is to turn random crap on/off for debugging
	pub test: bool,
}

impl TerminalBuffer {
	pub fn new(w: u16, h: u16) -> Self {

		let char_len = w as usize * h as usize * ASCII_BYTES_PER_CHAR;

		let debug_file = Self::open_and_clear_debug_file();

		let mut this = TerminalBuffer {
			wid: w,
			hei: h,
			raw_ascii_screen: vec![0; char_len],

			proj_mat:   create_identity_4x4(),
			transf_mat: create_identity_4x4(),
			render_mat: create_identity_4x4(),

    		sorting_mode:   ZSortingMode::BallsLast,
			cull_mask:      CullMode::Nothing,
			ball_fill_mode: BallFillMode::Index,
			gizmos_mode:    GizmosType::None,

			debug_file,
			test: false,
		};

		render_clear(&mut this);
		this
	}


	pub fn get_sorting_mode(&self) -> &ZSortingMode {
		&self.sorting_mode
	}

	pub fn get_cull_mode(&self) -> &CullMode {
		&self.cull_mask
	}

	pub fn get_ball_fill_mode(&self) -> &BallFillMode {
		&self.ball_fill_mode
	}

	pub fn get_gizmos_mode(&self) -> &GizmosType {
		&self.gizmos_mode
	}

	fn open_and_clear_debug_file() -> Option<DebugFile> {
		// File::create(Self::DEBUG_FILE_PATH).ok()
		File::create(Self::DEBUG_FILE_PATH).map(BufWriter::new).ok()
	}

	pub fn resize_and_render_clear(&mut self, w: u16, h: u16) {

		self.wid = w;
		self.hei = h;

		let char_len = w as usize * h as usize * ASCII_BYTES_PER_CHAR;
		self.raw_ascii_screen.clear();
		self.raw_ascii_screen.resize(char_len, 0);

		render_clear(self);
	}

	pub fn update_proj_matrix(&mut self) {
		// apply_identity_to_mat_4x4(&mut self.proj_mat);
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

		let file_result = File::create(Self::SCREENSHOT_PATH).map(BufWriter::new);
		if file_result.is_err() { return }

		// let mut screenshot_file = file_result.unwrap();
		let mut screenshot_file = BufWriter::new(file_result.unwrap());

		for y in 0..self.hei {

			let y_start = y       as usize * self.wid as usize * ASCII_BYTES_PER_CHAR;
			let y_end   = (y + 1) as usize * self.wid as usize * ASCII_BYTES_PER_CHAR;

			// let buf_str = std::str::from_utf8(&self.vec[y_start .. y_end]).unwrap();
			// screenshot_file.write_all(buf_str.as_bytes()).unwrap();

			screenshot_file.write_all(&self.raw_ascii_screen[y_start .. y_end]).unwrap();
			screenshot_file.write_all(&[b'\n']).unwrap();
		}
	}

	pub fn toggle_z_sorting_mode(&mut self) {
		match self.sorting_mode {
			ZSortingMode::ClosestPoint  => self.sorting_mode = ZSortingMode::FarthestPoint,
			ZSortingMode::FarthestPoint => self.sorting_mode = ZSortingMode::BallsLast,
			ZSortingMode::BallsLast     => self.sorting_mode = ZSortingMode::LinesLast,
			ZSortingMode::LinesLast     => self.sorting_mode = ZSortingMode::ClosestPoint,
		};
	}

	pub fn toggle_cull_mode(&mut self) {
		self.cull_mask = match self.cull_mask {
			CullMode::Nothing   => CullMode::CullTris,
			CullMode::CullTris  => CullMode::CullBalls,
			CullMode::CullBalls => CullMode::Nothing,
		}
	}

	pub fn toggle_ball_fill_mode(&mut self) {
		self.ball_fill_mode = match self.ball_fill_mode {
			BallFillMode::Height     => BallFillMode::XZDistance,
			BallFillMode::XZDistance => BallFillMode::Index,
			BallFillMode::Index      => BallFillMode::Height,
		}
	}

	pub fn toggle_gizmos_mode(&mut self) {
		self.gizmos_mode = match self.gizmos_mode {
			GizmosType::None      => GizmosType::WorldAxes,
			GizmosType::WorldAxes => GizmosType::None,
		}
	}


	const DEBUG_FILE_PATH: &str = "bullshit/_debug.txt";
	pub fn clear_debug(&mut self) {	
		self.debug_file = Self::open_and_clear_debug_file();
	}

	pub fn write_debug(&mut self, string: &str) {
		if let Some(ref mut file) = &mut self.debug_file {
			file.write_all(string.as_bytes()).expect("shit, couldn't write to file");
		}
	}

	// OTHER WAYS OF DOING THIS SHIT:

	#[allow(clippy::style)]
	pub fn write_debug2(&mut self, string: &str) {
		if self.debug_file.is_none() { return }
		let file = self.debug_file.as_mut().unwrap();
		file.write_all(string.as_bytes()).expect("shit, couldn't write to file");
	}

	#[allow(clippy::style)]
	pub fn write_debug3(&mut self, string: &str) {
		match self.debug_file.as_mut() {
			None => (),
			Some(file) => { file.write_all(string.as_bytes()).expect("shit, couldn't write to file"); },
		}
	}

	#[allow(clippy::style)]
	pub fn write_debug4(&mut self, string: &str) {
		if self.debug_file.is_none() { return }

		self.debug_file.as_mut().unwrap().write_all(string.as_bytes()).expect("shit, couldn't write to file");
	}

	#[allow(clippy::style)]
	pub fn write_debug5(&mut self, string: &str) {
		if self.debug_file.is_some() {
			self.debug_file.as_mut().unwrap().write_all(string.as_bytes()).expect("shit, couldn't write to file");
		}
	}

	#[allow(clippy::style)]
	pub fn write_debug6(&mut self, string: &str) -> Option<()> {
		let file = self.debug_file.as_mut()?;
		file.write_all(string.as_bytes()).expect("shit, couldn't write to file");
		Some(())
	}

}