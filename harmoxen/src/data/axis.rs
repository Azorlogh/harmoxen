#[derive(Clone, Copy)]
pub enum Axis {
	X,
	Y,
}

impl Axis {
	pub fn major<T: Into<[f32; 2]>>(&self, coords: T) -> f32 {
		match *self {
			Axis::X => coords.into()[0],
			Axis::Y => coords.into()[1],
		}
	}

	pub fn with_major<T: Into<[f32; 2]> + From<[f32; 2]>>(&self, coords: T, value: f32) -> T {
		let mut t = coords.into();
		match *self {
			Axis::X => t[0] = value,
			Axis::Y => t[1] = value,
		}
		t.into()
	}
}
