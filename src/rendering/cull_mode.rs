use core::fmt;


pub enum CullMode {
	Nothing,
	CullTris,
	CullBalls,
}

impl fmt::Display for CullMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			CullMode::Nothing   => write!(f, "nothing", ),
			CullMode::CullBalls => write!(f, "cull balls", ),
			CullMode::CullTris  => write!(f, "cull triangles", ),
		}
	}
}
