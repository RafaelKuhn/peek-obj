use std::time::Instant;

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

#[macro_export]
macro_rules! bench_clr {
	($ben: expr, $buf_ref: expr) => {
		#[cfg(debug_assertions)] {
			$buf_ref.clear_debug();
			$buf_ref.write_debug(&format!("with resolution {} x {}\n", $buf_ref.wid, $buf_ref.hei));
		}
	}
}

// #[macro_export]
// macro_rules! bench_ini {
// 	($ben: expr, $buf_ref: expr) => {
// 		#[cfg(debug_assertions)] {
// 			$buf_ref.clear_debug();
// 			$buf_ref.write_debug(&format!("with resolution {} x {}\n", $buf_ref.wid, $buf_ref.hei));
// 			$ben.start()
// 		}
// 	}
// }

#[macro_export]
macro_rules! bench {
	($ben: expr, $message: expr, $app_mut: expr) => {
		#[cfg(debug_assertions)] {
			$ben.end_and_log($message, &mut $app_mut)
		}
	};
}

#[macro_export]
macro_rules! bench_st {
	($ben: expr) => {
		#[cfg(debug_assertions)] {
			$ben.start()
		}
	};
}

#[macro_export]
macro_rules! bench_accum {
	($ben: expr, $app_mut: expr) => {
		$app_mut.write_debug(&$ben.accum_end())
	};
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
		let str = format!("\ntook {:<3} ms, {:<6} us in total - {:.2} FPS\n", self.total_ms, self.total_mc, 1_000_000.0 / self.total_mc as f32);
		self.total_ms = 0;
		self.total_mc = 0;
		str
	}

}