use std::time::{Instant, Duration};



pub struct AppTimer {
	pub frame_count: u32,
	pub delta_time: Duration,
	pub time_since_start: Duration,
	
	start: Instant,
	last_tick: Instant,
}

impl AppTimer {
	pub fn init() -> Self {
		let now = Instant::now();
		let duration2ms = Duration::from_millis(2);
		Self {
			start: now,
			last_tick: now,
    		frame_count: 0,
    		delta_time: duration2ms,
    		time_since_start: duration2ms,
		}
	}

	pub fn add_frame(&mut self) {
		let now = Instant::now();

		self.delta_time = now - self.last_tick;
		self.last_tick = now;
		
		self.frame_count += 1;

		self.time_since_start = now - self.start;
		// time_since_start = (last_tick_temp - start).as_millis();
	}
	
}