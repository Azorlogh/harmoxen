use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range(pub f32, pub f32);

impl Default for Range {
	fn default() -> Range {
		Range(0.0, 1.0)
	}
}

#[allow(dead_code)]
impl Range {
	pub fn size(&self) -> f32 {
		self.1 - self.0
	}

	pub fn contains(&self, x: f32) -> bool {
		self.0 <= x && x <= self.1
	}
	pub fn is_below(&self, x: f32) -> bool {
		self.1 < x
	}
	pub fn is_above(&self, x: f32) -> bool {
		x < self.0
	}

	// scale the range with a fixed point p
	pub fn scale_around(&mut self, p: f32, s: f32) {
		let new_0 = -(p - self.0) * s + p;
		let new_1 = -(p - self.1) * s + p;
		self.0 = new_0;
		self.1 = new_1;
	}

	// scale the range with a fixed middle point
	pub fn scale(&mut self, s: f32) {
		let middle = (self.0 + self.1) / 2.0;
		self.scale_around(s, middle);
	}
}

impl ops::Add<f32> for Range {
	type Output = Range;
	fn add(self, offset: f32) -> Range {
		Range(self.0 + offset, self.1 + offset)
	}
}
impl ops::Sub<f32> for Range {
	type Output = Range;
	fn sub(self, offset: f32) -> Range {
		Range(self.0 - offset, self.1 - offset)
	}
}
impl ops::Mul<f32> for Range {
	type Output = Range;
	fn mul(self, offset: f32) -> Range {
		Range(self.0 * offset, self.1 * offset)
	}
}
impl ops::Div<f32> for Range {
	type Output = Range;
	fn div(self, offset: f32) -> Range {
		Range(self.0 / offset, self.1 / offset)
	}
}

impl ops::AddAssign<f32> for Range {
	fn add_assign(&mut self, offset: f32) {
		self.0 += offset;
		self.1 += offset;
	}
}
impl ops::SubAssign<f32> for Range {
	fn sub_assign(&mut self, offset: f32) {
		self.0 -= offset;
		self.1 -= offset;
	}
}
impl ops::MulAssign<f32> for Range {
	fn mul_assign(&mut self, offset: f32) {
		self.0 *= offset;
		self.1 *= offset;
	}
}
impl ops::DivAssign<f32> for Range {
	fn div_assign(&mut self, offset: f32) {
		self.0 /= offset;
		self.1 /= offset;
	}
}
