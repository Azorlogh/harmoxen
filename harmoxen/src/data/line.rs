use crate::data::Point;

#[derive(Clone, Copy)]
pub struct Line {
	/// The line's start point.
	pub p0: Point,
	/// The line's end point.
	pub p1: Point,
}

impl Line {
	pub fn new(p0: Point, p1: Point) -> Line {
		Line { p0, p1 }
	}
}
