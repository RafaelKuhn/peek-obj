use std::process;


pub enum ControlMode {
	Automatic,
	Wasd,
}

pub struct Settings {
	pub has_custom_path: bool,
	pub custom_path: String,
	pub draw_normals: bool,
	pub control_mode: ControlMode,
}

impl Settings {
	// pub fn from_args<'a, It>(args: It) -> Settings
	pub fn from_args<It>(args: It) -> Settings
	where
		It: Iterator<Item = String>
	{
		let mut settings = Settings {
			has_custom_path: false,
			custom_path: "obj/teapot.obj".to_string(),
			draw_normals: false,
			control_mode: ControlMode::Wasd,
		};

		for arg in args {

			let is_option = arg.as_str().starts_with('-');
			if is_option {
				match arg.as_str() {
					"-n" | "--normal" | "--normals" => {
						settings.draw_normals = true;
					}
					"-c" | "--control" | "--controls" | "--wasd" => {
						settings.control_mode = ControlMode::Wasd;
					}
					_ => {
						let chars_after_slash = arg.chars().skip(1);
						println!("Unknown option -- {}", String::from_iter(chars_after_slash));
						process::exit(1);
					}
				}
			}

			if settings.has_custom_path {
				println!("Too many arguments!");
				process::exit(1);
			}

			settings.has_custom_path = true;
			settings.custom_path = arg;
		}

		settings
	}


}
