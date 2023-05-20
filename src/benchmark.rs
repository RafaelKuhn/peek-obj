use crate::timer::AppTimer;

pub struct Benchmark {
	pub delta_time: f32,
	pub fps: i32,
	pub total_frame_count: u32,
	
	refresh_rate: f32,
	accum_time: f32,
	frame_count_measurement: u32,
	// accum_time: f32,
}

impl Benchmark {
	pub fn new(refresh_rate: f32) -> Self {
		let mut benchmark: Benchmark = Default::default();
		benchmark.set_refresh_rate_secs(refresh_rate);
		return benchmark;
	}

	pub fn set_refresh_rate_secs(&mut self, refresh_rate: f32) {
		self.refresh_rate = refresh_rate;
	}


	pub fn profile_frame(&mut self, timer: &AppTimer) {

		self.frame_count_measurement += 1;
		self.accum_time += timer.delta_time.as_micros() as f32 * 0.000_001;

		// let update_interval = 0.5;
		if self.accum_time > self.refresh_rate {
			self.fps = (self.frame_count_measurement as f32 / self.accum_time) as i32;
			self.total_frame_count = timer.frame_count;
			self.delta_time = timer.delta_time.as_micros() as f32 * 0.000_001;
			
			self.accum_time = 0.0;
			self.frame_count_measurement = 0;
		}
	}
}

impl Default for Benchmark {
    fn default() -> Self {
        Self { 
			delta_time: Default::default(),
			fps: Default::default(),
			total_frame_count: Default::default(),
			refresh_rate: 0.5,
			accum_time: Default::default(),
			frame_count_measurement: Default::default(),
		}
    }
}