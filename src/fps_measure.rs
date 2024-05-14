use std::time::{Duration, Instant};

use crate::timer::Timer;

pub struct FpsMeasure {
	pub delta_time_millis: f32,
	pub fps: f32,
	pub total_frame_count: u32,
	pub time_aggr: Duration,
	pub time_scale: f32,

	refresh_rate: f32,
	accum_time: f32,
	frame_count_measurement: u32,
}

impl Default for FpsMeasure {
	fn default() -> Self {
		Self {
			delta_time_millis: Default::default(),
			fps: Default::default(),
			total_frame_count: Default::default(),
			time_aggr: Default::default(),
			time_scale: Default::default(),
			refresh_rate: Default::default(),
			accum_time: Default::default(),
			frame_count_measurement: Default::default(),
		}
	}
}

impl FpsMeasure {
	pub fn new(refresh_rate_secs: f32) -> Self {
		let mut benchmark: FpsMeasure = Default::default();
		benchmark.set_refresh_rate_secs(refresh_rate_secs);

		benchmark
	}

	pub fn set_refresh_rate_secs(&mut self, refresh_rate: f32) {
		self.refresh_rate = refresh_rate;
	}

	pub fn profile_frame(&mut self, timer: &Timer) {
		self.frame_count_measurement += 1;
		self.accum_time += timer.delta_time.as_micros() as f32 * 0.000_001;
		self.time_aggr = timer.time_aggr;
		self.time_scale = timer.time_scale;
		self.total_frame_count = timer.frame_count;

		if self.accum_time > self.refresh_rate {
			self.fps = self.frame_count_measurement as f32 / self.accum_time;
			self.delta_time_millis = timer.delta_time.as_micros() as f32 * 0.001;

			self.accum_time = 0.0;
			self.frame_count_measurement = 0;
		}
	}

}