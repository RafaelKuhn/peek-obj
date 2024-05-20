use core::fmt;


pub enum BallFillMode {
	Height,
	XZDistance,
	Index,
}

impl fmt::Display for BallFillMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			BallFillMode::Height     => write!(f, "by height", ),
			BallFillMode::XZDistance => write!(f, "by 2D distance", ),
			BallFillMode::Index      => write!(f, "by index", ),
		}
	}
}