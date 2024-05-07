use crate::Vec3;


pub struct BoundingBox {
	pub top_right_front: Vec3,
	pub top_right_back:  Vec3,
	pub top_left_back:   Vec3,
	pub top_left_front:  Vec3,
	pub bottom_right_front: Vec3,
	pub bottom_right_back:  Vec3,
	pub bottom_left_back:   Vec3,
	pub bottom_left_front:  Vec3,
}

impl BoundingBox {
	pub fn from_verts(verts: &[f32]) -> Self {
		debug_assert!(verts.len() % 3 == 0, "Verts length is not a multiple of 3");

		let first_x = verts[0];
		let first_y = verts[1];
		let first_z = verts[2];
		let mut biggest_x = first_x;
		let mut lowest_x  = first_x;
		let mut biggest_y = first_y;
		let mut lowest_y  = first_y;
		let mut biggest_z = first_z;
		let mut lowest_z  = first_z;

		let mut i = 3;
		while i < verts.len() {
			let x = verts[i];
			if x > biggest_x { biggest_x = x }
			else if x < lowest_x { lowest_x = x }

			i += 1;
			let y = verts[i];
			if y > biggest_y { biggest_y = y }
			else if y < lowest_y { lowest_y = y }

			i += 1;
			let z = verts[i];
			if z > biggest_z { biggest_z = z }
			else if z < lowest_z { lowest_z = z }

			i += 1;
		}

		Self {
 			top_right_front: Vec3::new(biggest_x, biggest_y, biggest_z),
 			top_right_back:  Vec3::new(biggest_x, biggest_y, lowest_z),
 			top_left_back:   Vec3::new(lowest_x,  biggest_y, lowest_x),
 			top_left_front:  Vec3::new(lowest_x,  biggest_y, biggest_z),
 			bottom_right_front: Vec3::new(biggest_x, lowest_y,  biggest_z),
 			bottom_right_back:  Vec3::new(biggest_x, lowest_y,  lowest_z),
 			bottom_left_back:   Vec3::new(lowest_x,  lowest_y,  lowest_x),
 			bottom_left_front:  Vec3::new(lowest_x,  lowest_y,  biggest_z),
		}
	}

	pub fn from_vec3_iter<'a>(mut vecs: impl Iterator<Item = &'a Vec3>) -> Self {

		let first = vecs.next().expect("Empty iterator!");
		let mut biggest_x = first.x;
		let mut lowest_x  = first.x;
		let mut biggest_y = first.y;
		let mut lowest_y  = first.y;
		let mut biggest_z = first.z;
		let mut lowest_z  = first.z;

		for vec in vecs.skip(1) {
			if vec.x > biggest_x { biggest_x = vec.x }
			else if vec.x < lowest_x { lowest_x = vec.x }

			if vec.y > biggest_y { biggest_y = vec.y }
			else if vec.y < lowest_y { lowest_y = vec.y }

			if vec.z > biggest_z { biggest_z = vec.z }
			else if vec.z < lowest_z { lowest_z = vec.z }
		}

		Self {
			top_right_front: Vec3::new(biggest_x, biggest_y, biggest_z),
			top_right_back:  Vec3::new(biggest_x, biggest_y, lowest_z),
			top_left_back:   Vec3::new(lowest_x,  biggest_y, lowest_x),
			top_left_front:  Vec3::new(lowest_x,  biggest_y, biggest_z),
			bottom_right_front: Vec3::new(biggest_x, lowest_y,  biggest_z),
			bottom_right_back:  Vec3::new(biggest_x, lowest_y,  lowest_z),
			bottom_left_back:   Vec3::new(lowest_x,  lowest_y,  lowest_x),
			bottom_left_front:  Vec3::new(lowest_x,  lowest_y,  biggest_z),
		}
	}

}