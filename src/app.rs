use std::time::{Duration, Instant};

use crossterm::terminal::size;

use crate::{camera::Camera, render_string, timer::Timer, TerminalBuffer, UVec2, Vec3};


pub struct App {
	pub buf: TerminalBuffer,
	pub is_full_screen: bool,

	// data that the user can toggle
	has_paused_full: bool,
	has_paused_anim_only: bool,
	is_free_mov: bool,
	pub is_verbose: bool,
	pub is_help_screen: bool,

	// this is the only "event" that needs to happen after scene is drawn
	pub called_take_screenshot: bool,

	// TODO: struct 'user polled data', this can probably be stateless
	pub user_dir: Vec3,
	pub user_rot: Vec3,
	pub called_reset_camera: bool,
	pub called_set_camera_default_orientation: bool,

	pub called_toggle_free_mov: bool,

	last_screenshot_instant: Instant,
}

impl App {
	const SCREENDUMP_DELAY_MS: u64 = 300;
	const SCREENDUMP_DELAY_DURATION: Duration = Duration::from_millis(App::SCREENDUMP_DELAY_MS);

	fn init(width: u16, height: u16, is_full_screen: bool) -> App {
		Self {
			is_full_screen,
			has_paused_full: false,
			has_paused_anim_only: false,
			is_free_mov: false,
			is_verbose: true,
			is_help_screen: false,

			buf: TerminalBuffer::new(width, height),

			user_dir: Vec3::zero(),
			user_rot: Vec3::zero(),

			called_take_screenshot: false,

			called_reset_camera: false,
			called_set_camera_default_orientation: false,
			called_toggle_free_mov: false,

			last_screenshot_instant: Instant::now() - App::SCREENDUMP_DELAY_DURATION,
		}
	}

	pub fn init_with_screen() -> App {
		let (screen_width, screen_height) = size().unwrap();
		Self::init(screen_width, screen_height, true)
	}

	pub fn init_wh(width: u16, height: u16) -> App {
		Self::init(width, height, false)
	}

	pub fn run_post_render_events(&mut self, timer: &Timer) {
		self.try_saving_screenshot(timer);
	}

	pub fn try_saving_screenshot(&mut self, timer: &Timer) {
		let now = timer.last_tick;
		let diff_since_last_screenshot = now - self.last_screenshot_instant;
	
		let last_one_was_too_recent = diff_since_last_screenshot < App::SCREENDUMP_DELAY_DURATION;
		if last_one_was_too_recent {
			let diff_secs = (App::SCREENDUMP_DELAY_MS - diff_since_last_screenshot.as_millis() as u64) as f32 / 1000.0;
			render_string(&format!("screenshot cooldown: {}", diff_secs), &UVec2::new(0, 20), &mut self.buf);
			return;
		}
	
		if self.called_take_screenshot {
			self.called_take_screenshot = false;
			self.last_screenshot_instant = now;
	
			self.buf.try_dump_buffer_content_to_file();
		}
	}

	pub fn is_fully_paused(&self) -> bool {
		self.has_paused_full
	}

	pub const fn is_free_mov(&self) -> bool {
		self.is_free_mov
	}

	pub fn toggle_free_mov(&mut self, camera: &mut Camera) {
		let will_turn_orbital = self.is_free_mov;
		if will_turn_orbital {
			camera.reset_cached_dist();
		}

		self.is_free_mov = !self.is_free_mov;
	}

	pub fn toggle_pause_anim(&mut self, timer: &mut Timer) {
		if self.has_paused_full { return }

		if self.has_paused_anim_only {
			self.has_paused_anim_only = false;
			timer.reset_time_scale()
		} else {
			self.has_paused_anim_only = true;
			timer.time_scale = 0.0;
		}
	}

	pub fn toggle_pause_full(&mut self, timer: &mut Timer) {
		if self.has_paused_anim_only {
			self.toggle_pause_anim(timer);
			return;
		}

		if self.has_paused_full {
			self.has_paused_full = false;
			self.has_paused_anim_only = false;
		} else {
			self.has_paused_full = true;
		}
	}

	pub fn resize_realloc(&mut self, w: u16, h: u16) {
		if !self.is_full_screen { return; }

		// I have NO IDEA why Windows terminals need +1 for buffer size on resize events

		#[cfg(windows)]
		self.buf.resize_and_render_clear(w + 1, h + 1);

		#[cfg(unix)]
		self.buf.resize_and_render_clear(w, h);
	
		// (this is not a good idea) but ... when rendering is disabled, we could copy
		// the content of the previous frame, reescale it and draw it into the new one
		// (simply not gonna work)

		self.buf.update_proj_matrix();
	}

}