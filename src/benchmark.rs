use std::time::{Duration, Instant};

pub struct Benchmark {
	last_measure: Instant,
	total_mic: u128,
	total_ms: u128,
}

impl Default for Benchmark {
	fn default() -> Self {
		Self {
			last_measure: Instant::now(),
			total_mic: Default::default(),
			total_ms: Default::default()
		}
	}
}

impl Benchmark {
	pub fn start(&mut self) {
		self.last_measure = Instant::now();
	}

	pub fn end(&mut self, msg: &str) -> String {
		let now = Instant::now();
		let dur = now - self.last_measure;
		self.last_measure = now;

		format!("took {:<3} ms, {:<6} us, '{}'\n", dur.as_millis(), dur.as_micros(), msg)
	}

}