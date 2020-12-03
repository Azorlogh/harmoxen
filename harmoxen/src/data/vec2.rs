/// A 2D vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T = f32> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2<T> {
	/// Creates a new [`Vector`] with the given components.
	///
	/// [`Vector`]: struct.Vector.html
	pub const fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
}

impl Vec2<f32> {
	pub const ZERO: Vec2<f32> = Vec2 { x: 0.0, y: 0.0 };
}

impl<T> std::ops::Add for Vec2<T>
where
	T: std::ops::Add<Output = T>,
{
	type Output = Self;

	fn add(self, b: Self) -> Self {
		Self::new(self.x + b.x, self.y + b.y)
	}
}

impl<T> std::ops::Neg for Vec2<T>
where
	T: std::ops::Neg<Output = T>,
{
	type Output = Self;

	fn neg(self) -> Self {
		Self::new(-self.x, -self.y)
	}
}

impl<T> std::ops::Sub for Vec2<T>
where
	T: std::ops::Sub<Output = T>,
{
	type Output = Self;

	fn sub(self, b: Self) -> Self {
		Self::new(self.x - b.x, self.y - b.y)
	}
}

impl<T> std::ops::Mul<T> for Vec2<T>
where
	T: std::ops::Mul<Output = T> + Copy,
{
	type Output = Self;

	fn mul(self, scale: T) -> Self {
		Self::new(self.x * scale, self.y * scale)
	}
}

impl<T> Default for Vec2<T>
where
	T: Default,
{
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
		}
	}
}

impl<T> From<iced::Vector<T>> for Vec2<T> {
	fn from(p: iced::Vector<T>) -> Vec2<T> {
		Vec2 { x: p.x, y: p.y }
	}
}

impl<T> From<Vec2<T>> for iced::Vector<T> {
	fn from(p: Vec2<T>) -> iced::Vector<T> {
		iced::Vector { x: p.x, y: p.y }
	}
}
