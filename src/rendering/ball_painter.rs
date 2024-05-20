use crate::{BallFillMode, RenderBallData, Vec3};


pub struct BallPainter {
	pub min_height: f32,
	pub max_height: f32,
	pub min_dist_xz_sq: f32,
	pub max_dist_xz_sq: f32,
	
	pub range_dist_xz: f32,
	pub range_height: f32,

	paint_algorithm: fn(&BallPainter, &RenderBallData) -> char,
}

const ASCII_LUMINANCE: &'static [char] = &[ '.', ',', '-', '~', ':', ';', '=', '!', '&', '#', '@', ];
	// = [ '.', '-', ':', '=', '!', '&', '#', '@' ];
	// = [ '.', '-', ':', '=', '!', '&', '@' ];

impl BallPainter {
	pub fn new(fill_mode: &BallFillMode) -> Self {
		let paint_algorithm = match fill_mode {
			BallFillMode::Height     => Self::paint_by_height,
			BallFillMode::XZDistance => Self::paint_by_xz_dist,
			BallFillMode::Index      => Self::paint_by_index,
		};

		Self {
			min_height:    f32::MAX,
			max_height:    f32::MIN,
			range_height:  0.0,

			min_dist_xz_sq:   f32::MAX,
			max_dist_xz_sq:   f32::MIN,
			range_dist_xz: 0.0,
			paint_algorithm,
		}
	}

	pub fn find_min_max(&mut self, transformed_pos: &Vec3, sq_dist_xz: f32) {
		self.min_dist_xz_sq = self.min_dist_xz_sq.min(sq_dist_xz);
		self.max_dist_xz_sq = self.max_dist_xz_sq.max(sq_dist_xz);
		self.range_dist_xz = self.max_dist_xz_sq - self.min_dist_xz_sq;
		
		self.min_height = self.min_height.min(transformed_pos.y);
		self.max_height = self.max_height.max(transformed_pos.y);
		self.range_height = self.max_height - self.min_height;
	}

	pub fn get_fill_letter(&self, ball_data: &RenderBallData) -> char {
		(self.paint_algorithm)(&self, ball_data)
	}

	fn paint_by_index(&self, ball_data: &RenderBallData) -> char {
		let digit = ball_data.index as u32 % ('Z' as u32 - 'A' as u32) + ('A' as u32);
		char::from_u32(digit).unwrap()
	}

	fn paint_by_xz_dist(&self, ball_data: &RenderBallData) -> char {
		let dist_clamped_0_1 = (ball_data.sq_dist_to_camera - self.min_dist_xz_sq) / self.range_dist_xz;
		let index = ( dist_clamped_0_1 * (ASCII_LUMINANCE.len() - 1) as f32 ).round() as usize;
		let index = ASCII_LUMINANCE.len() - index - 1;
		ASCII_LUMINANCE[index]
	}

	fn paint_by_height(&self, ball_data: &RenderBallData) -> char {
		let height_clamped_0_1 = (ball_data.height - self.min_height) / self.range_height;
		let index = ( height_clamped_0_1 * (ASCII_LUMINANCE.len() - 1) as f32 ).round() as usize;
		let index = ASCII_LUMINANCE.len() - index - 1;

		ASCII_LUMINANCE[index]
	}

}