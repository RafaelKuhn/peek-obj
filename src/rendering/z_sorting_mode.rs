use core::fmt;
use std::cmp::Ordering;

use crate::Primitive;

pub type SortingFn = fn(&(f32, Primitive), &(f32, Primitive)) -> Ordering;


pub enum ZSortingMode {
	ClosestPoint,
	FarthestPoint,
	LinesLast,
	BallsLast,
}

impl ZSortingMode {
	pub fn get_sorting_fn(&self) -> SortingFn {
		match self {
			ZSortingMode::ClosestPoint  => sort_by_distance,
			ZSortingMode::FarthestPoint => sort_by_distance,
			ZSortingMode::LinesLast => sort_lines_last,
			ZSortingMode::BallsLast => sort_balls_last,
		}
	}
}

impl fmt::Display for ZSortingMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ZSortingMode::ClosestPoint  => write!(f, "closest point", ),
			ZSortingMode::FarthestPoint => write!(f, "farthest point", ),
			ZSortingMode::LinesLast => write!(f, "lines last", ),
			ZSortingMode::BallsLast => write!(f, "balls last", ),
		}
	}
}

fn compare_distances(a: &f32, b: &f32) -> Ordering {
	b.partial_cmp(&a).expect("Invalid float for some reason, what the actual fuck")
}

pub fn sort_by_distance(a: &(f32, Primitive), b: &(f32, Primitive)) -> Ordering {
	let (dist_a, _) = a;
	let (dist_b, _) = b;
	return dist_b.partial_cmp(&dist_a).unwrap();
}

pub fn sort_lines_last(a: &(f32, Primitive), b: &(f32, Primitive)) -> Ordering {
	with_lines_and_balls_ordering(a, b, Ordering::Less, Ordering::Greater)
}

pub fn sort_balls_last(a: &(f32, Primitive), b: &(f32, Primitive)) -> Ordering {
	with_lines_and_balls_ordering(a, b, Ordering::Greater, Ordering::Less)
}

fn with_lines_and_balls_ordering(a: &(f32, Primitive), b: &(f32, Primitive), line_ord: Ordering, ball_ord: Ordering) -> Ordering {
	let (dist_a, primitive_a) = a;
	let (dist_b, primitive_b) = b;

	match (primitive_a, primitive_b) {
		(Primitive::Ball(_),    Primitive::Ball(_))    => compare_distances(dist_a, dist_b),
		(Primitive::Line(_, _), Primitive::Line(_, _)) => compare_distances(dist_a, dist_b),
		(Primitive::Ball(_),    Primitive::Line(_, _)) => line_ord,
		(Primitive::Line(_, _), Primitive::Ball(_))    => ball_ord,
	}
}