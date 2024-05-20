use core::fmt;


pub enum GizmosType {
	None,
	WorldAxes
}

impl fmt::Display for GizmosType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			GizmosType::None      => write!(f, "none", ),
			GizmosType::WorldAxes => write!(f, "world axis", ),
		}
	}
}