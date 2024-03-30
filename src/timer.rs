use std::time::{Instant, Duration};


pub struct Timer {
	pub frame_count: u32,
	pub delta_time: Duration,
	pub time_since_start: Duration,
	pub time_aggr: Duration,
	pub time_scale: f32,

	start: Instant,
	last_tick: Instant,
}

impl Timer {
	pub fn new() -> Self {
		let now = Instant::now();
		let duration_of_2ms = Duration::from_millis(2);
		Self {
			frame_count:      0,
			delta_time:       duration_of_2ms,
			time_since_start: duration_of_2ms,
			time_aggr:        duration_of_2ms,
			time_scale:       1.0,
			
			start:            now,
			last_tick:        now,
		}
	}

	pub fn run(&mut self) {
		let now = Instant::now();

		self.delta_time = (now - self.last_tick).mul_f32(self.time_scale);
		self.last_tick = now;

		self.frame_count += 1;

		self.time_since_start = now - self.start;
		self.time_aggr += self.delta_time;
	}

}