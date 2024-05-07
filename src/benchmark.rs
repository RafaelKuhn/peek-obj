use std::time::{Duration, Instant};

use crate::TerminalBuffer;

pub struct Benchmark {
	name: Option<String>,
	last_measure: Instant,
	total_mc: u128,
	total_ms: u128,
}

impl Default for Benchmark {
	fn default() -> Self {
		Self {
			last_measure: Instant::now(),
			total_mc: Default::default(),
			total_ms: Default::default(),
    		name: Default::default(),
		}
	}
}

impl Benchmark {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn named(name: &str) -> Self {
		let mut b: Self = Default::default();
		b.name = Some(name.to_owned());
		b
	}

	pub fn start(&mut self) {
		self.last_measure = Instant::now();
	}

	pub fn end_and_log(&mut self, msg: &str, buf: &mut TerminalBuffer) {
		// self.just_end(); return;

		let result = self.end(msg);
		buf.write_debug(&result);
	}

	 fn end(&mut self, msg: &str) -> String {
		let now = Instant::now();
		let dur = now - self.last_measure;
		self.last_measure = now;
		
		let millis = dur.as_millis();
		let micros = dur.as_micros();
		self.total_mc += micros;
		self.total_ms += millis;

		return if let Some(name) = &self.name {
			format!("took {:<3} ms, {:<6} us, {}: '{}'\n", millis, micros, name, msg)
		} else {
			format!("took {:<3} ms, {:<6} us, '{}'\n", millis, micros, msg)
		}
	}

	fn just_end(&mut self) {
		let now = Instant::now();
		let dur = now - self.last_measure;
		self.last_measure = now;
		
		let millis = dur.as_millis();
		let micros = dur.as_micros();
		self.total_mc += micros;
		self.total_ms += millis;
	}

	pub fn accum_end(&mut self) -> String {
		let str = format!("took {:<3} ms, {:<6} us in total - {:.2} FPS\n", self.total_ms, self.total_mc, 1000.0 / self.total_ms as f32);
		self.total_ms = 0;
		self.total_mc = 0;
		str
	}

}