use crate::data::{Point, Size, Vec2};

/// A rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect<T = f32> {
	/// X coordinate of the top-left corner.
	pub x0: T,

	/// Y coordinate of the top-left corner.
	pub y0: T,

	/// X coordinate of the bottom-right corner.
	pub x1: T,

	/// Y coordinate of the bottom-right corner.
	pub y1: T,
}

impl Rect<f32> {
	pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
		Self { x0, y0, x1, y1 }
	}

	/// Creates a new [`Rect`] with its top-left corner in the given
	/// [`Point`] and with the provided [`Size`].
	///
	/// [`Rect`]: struct.Rect.html
	/// [`Point`]: struct.Point.html
	/// [`Size`]: struct.Size.html
	pub fn from_point_size(top_left: Point, size: Size) -> Self {
		Self {
			x0: top_left.x,
			y0: top_left.y,
			x1: top_left.x + size.width,
			y1: top_left.y + size.height,
		}
	}

	/// Creates a new [`Rect`] with its top-left corner and its
	/// bottom-right corner in its given [`Point`]s.
	///
	/// [`Rect`]: struct.Rect.html
	/// [`Point`]: struct.Point.html
	/// [`Size`]: struct.Size.html
	pub fn from_points(top_left: Point, bottom_right: Point) -> Self {
		Self {
			x0: top_left.x,
			y0: top_left.y,
			x1: bottom_right.x,
			y1: bottom_right.y,
		}
	}

	/// Creates a new [`Rect`] with its top-left corner at the origin
	/// and with the provided [`Size`].
	///
	/// [`Rect`]: struct.Rect.html
	/// [`Size`]: struct.Size.html
	pub fn with_size(size: Size) -> Self {
		Self {
			x0: 0.0,
			y0: 0.0,
			x1: size.width,
			y1: size.height,
		}
	}

	/// Returns the [`Point`] at the center of the [`Rect`].
	///
	/// [`Point`]: struct.Point.html
	/// [`Rect`]: struct.Rect.html
	pub fn center(&self) -> Point {
		Point::new(self.center_x(), self.center_y())
	}

	/// Returns the X coordinate of the [`Point`] at the center of the
	/// [`Rect`].
	///
	/// [`Point`]: struct.Point.html
	/// [`Rect`]: struct.Rect.html
	pub fn center_x(&self) -> f32 {
		(self.x0 + self.x1) / 2.0
	}

	/// Returns the Y coordinate of the [`Point`] at the center of the
	/// [`Rect`].
	///
	/// [`Point`]: struct.Point.html
	/// [`Rect`]: struct.Rect.html
	pub fn center_y(&self) -> f32 {
		(self.y0 + self.y1) / 2.0
	}

	/// Returns the position of the top left corner of the [`Rect`].
	///
	/// [`Rect`]: struct.Rect.html
	pub fn position(&self) -> Point {
		Point::new(self.x0, self.y0)
	}

	/// Returns the [`Size`] of the [`Rect`].
	///
	/// [`Size`]: struct.Size.html
	/// [`Rect`]: struct.Rect.html
	pub fn size(&self) -> Size {
		Size::new(self.x1 - self.x0, self.y1 - self.y0)
	}

	/// Returns true if the given [`Point`] is contained in the [`Rect`].
	///
	/// [`Point`]: struct.Point.html
	/// [`Rect`]: struct.Rectangle.html
	pub fn contains(&self, point: Point) -> bool {
		self.x0 <= point.x && point.x <= self.x1 && self.y0 <= point.y && point.y <= self.y1
	}

	/// Computes the intersection with the given [`Rect`].
	///
	/// [`Rect`]: struct.Rect.html
	pub fn intersection(&self, other: &Rect<f32>) -> Option<Rect<f32>> {
		let x0 = self.x0.max(other.x0);
		let y0 = self.y0.max(other.y0);

		let x1 = self.x1.min(other.x1);
		let y1 = self.y1.min(other.y1);

		if (x1 - x0) > 0.0 && (y1 - y0) > 0.0 {
			Some(Rect { x0, y0, x1, y1 })
		} else {
			None
		}
	}

	/// Computes the union with the given [`Rect`].
	///
	/// [`Rect`]: struct.Rect.html
	pub fn union(&self, other: &Rect<f32>) -> Rect<f32> {
		let x0 = self.x0.min(other.x0);
		let y0 = self.y0.min(other.y0);

		let x1 = self.x1.max(other.x1);
		let y1 = self.y1.max(other.y1);

		Rect { x0, y0, x1, y1 }
	}

	/// Snaps the [`Rectangle`] to __unsigned__ integer coordinates.
	///
	/// [`Rectangle`]: struct.Rectangle.html
	pub fn snap(self) -> Rect<u32> {
		Rect {
			x0: self.x0 as u32,
			y0: self.y0 as u32,
			x1: self.x1.ceil() as u32,
			y1: self.y1.ceil() as u32,
		}
	}
}

impl std::ops::Mul<f32> for Rect<f32> {
	type Output = Self;

	fn mul(self, scale: f32) -> Self {
		Self {
			x0: self.x0 * scale,
			y0: self.y0 * scale,
			x1: self.x1 * scale,
			y1: self.y1 * scale,
		}
	}
}

impl From<Rect<u32>> for Rect<f32> {
	fn from(rect: Rect<u32>) -> Rect<f32> {
		Rect {
			x0: rect.x0 as f32,
			y0: rect.y0 as f32,
			x1: rect.x1 as f32,
			y1: rect.y1 as f32,
		}
	}
}

impl<T> std::ops::Add<Vec2<T>> for Rect<T>
where
	T: std::ops::Add<Output = T> + Copy,
{
	type Output = Rect<T>;

	fn add(self, translation: Vec2<T>) -> Self {
		Rect {
			x0: self.x0 + translation.x,
			y0: self.y0 + translation.y,
			x1: self.x1 + translation.x,
			y1: self.y1 + translation.y,
		}
	}
}

impl<T> std::ops::Sub<Vec2<T>> for Rect<T>
where
	T: std::ops::Sub<Output = T> + Copy,
{
	type Output = Rect<T>;

	fn sub(self, translation: Vec2<T>) -> Self {
		Rect {
			x0: self.x0 - translation.x,
			y0: self.y0 - translation.y,
			x1: self.x1 - translation.x,
			y1: self.y1 - translation.y,
		}
	}
}

impl<T> From<iced::Rectangle<T>> for Rect<T>
where
	T: std::ops::Add<Output = T> + Copy,
{
	fn from(r: iced::Rectangle<T>) -> Rect<T> {
		Rect {
			x0: r.x,
			y0: r.y,
			x1: r.x + r.width,
			y1: r.y + r.height,
		}
	}
}

impl<T> From<Rect<T>> for iced::Rectangle<T>
where
	T: std::ops::Sub<Output = T> + Copy,
{
	fn from(r: Rect<T>) -> iced::Rectangle<T> {
		iced::Rectangle {
			x: r.x0,
			y: r.y0,
			width: r.x1 - r.x0,
			height: r.y1 - r.y0,
		}
	}
}
