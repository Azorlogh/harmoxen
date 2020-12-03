use crate::data::Vec2;

/// A 2D point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
	/// The X coordinate.
	pub x: f32,

	/// The Y coordinate.
	pub y: f32,
}

impl Point {
	/// The origin (i.e. a [`Point`] at (0, 0)).
	///
	/// [`Point`]: struct.Point.html
	pub const ORIGIN: Point = Point::new(0.0, 0.0);

	/// Creates a new [`Point`] with the given coordinates.
	///
	/// [`Point`]: struct.Point.html
	pub const fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub fn to_vec2(&self) -> Vec2 {
		Vec2::new(self.x, self.y)
	}

	/// Computes the distance to another [`Point`].
	///
	/// [`Point`]: struct.Point.html
	pub fn distance(&self, to: Point) -> f32 {
		let a = self.x - to.x;
		let b = self.y - to.y;

		a.hypot(b)
	}
}

impl From<[f32; 2]> for Point {
	fn from([x, y]: [f32; 2]) -> Self {
		Point { x, y }
	}
}

impl From<[u16; 2]> for Point {
	fn from([x, y]: [u16; 2]) -> Self {
		Point::new(x.into(), y.into())
	}
}

impl From<Point> for [f32; 2] {
	fn from(point: Point) -> [f32; 2] {
		[point.x, point.y]
	}
}

impl std::ops::Add<Vec2> for Point {
	type Output = Self;

	fn add(self, vector: Vec2) -> Self {
		Self {
			x: self.x + vector.x,
			y: self.y + vector.y,
		}
	}
}

impl std::ops::Sub<Vec2> for Point {
	type Output = Self;

	fn sub(self, vector: Vec2) -> Self {
		Self {
			x: self.x - vector.x,
			y: self.y - vector.y,
		}
	}
}

impl std::ops::Sub<Point> for Point {
	type Output = Vec2;

	fn sub(self, point: Point) -> Vec2 {
		Vec2::new(self.x - point.x, self.y - point.y)
	}
}

impl From<iced::Point> for Point {
	fn from(p: iced::Point) -> Point {
		Point { x: p.x, y: p.y }
	}
}

impl From<Point> for iced::Point {
	fn from(p: Point) -> iced::Point {
		iced::Point { x: p.x, y: p.y }
	}
}
