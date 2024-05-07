use std::time::{Instant, Duration};


pub struct Timer {
	pub frame_count: u32,
	pub delta_time: Duration,
	pub time_since_start: Duration,
	pub time_aggr: Duration,
	pub time_scale: f32,
	
	pub last_tick: Instant,

	default_time_scale: f32,
	start: Instant,
}

impl Timer {
	pub fn new() -> Self {
		let now = Instant::now();
		const VERY_SHORT_DURATION: Duration = Duration::from_micros(1);
		Self {
			frame_count:      0,
			delta_time:       VERY_SHORT_DURATION,
			time_since_start: VERY_SHORT_DURATION,
			time_aggr:        VERY_SHORT_DURATION,
			time_scale:       1.0,
			
			start:            now,
			last_tick:        now,
    		default_time_scale:   1.0,
		}
	}

	pub fn set_default_time_scale(&mut self, time_scale: f32) {
		self.time_scale = time_scale;
		self.default_time_scale = time_scale;
	}

	pub fn reset_time_scale(&mut self) {
		self.time_scale = self.default_time_scale;
	}

	pub fn run_frame(&mut self) {
		self.frame_count += 1;
		self.run();
	}

	pub fn run(&mut self) {
		let now = Instant::now();

		self.delta_time = (now - self.last_tick).mul_f32(self.time_scale);
		self.last_tick = now;

		self.time_since_start = now - self.start;
		self.time_aggr += self.delta_time;
	}

}