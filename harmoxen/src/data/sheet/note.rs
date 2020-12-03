use super::{Interval, Sheet};
use crate::data::{Point, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Pitch<I> {
	Absolute(f32),
	Relative(I, Interval),
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Note<I> {
	pub pitch: Pitch<I>,
	pub start: f32,
	pub length: f32,
}

#[allow(unused)]
impl<I> Note<I> {
	pub fn new(pos: Point, note_len: f32) -> Note<I> {
		Note {
			start: pos.x,
			length: note_len,
			pitch: Pitch::Absolute(2f32.powf(pos.y)),
		}
	}

	pub fn end(&self) -> f32 {
		self.start + self.length
	}
}

use generational_arena::Index;

#[allow(unused)]
impl Note<Index> {
	pub fn y(&self, sheet: &Sheet) -> f32 {
		sheet.get_freq(self.pitch).log2()
	}

	pub fn start_pt(&self, sheet: &Sheet) -> Point {
		Point::new(self.start, sheet.get_y(self.pitch))
	}

	pub fn end_pt(&self, sheet: &Sheet) -> Point {
		Point::new(self.end(), sheet.get_y(self.pitch))
	}

	pub fn rect(&self, sheet: &Sheet, note_height: f32) -> Rect {
		let y = sheet.get_y(self.pitch);
		Rect::from_points(
			Point::new(self.start, sheet.get_y(self.pitch) - note_height / 2.0),
			Point::new(self.end(), sheet.get_y(self.pitch) - note_height / 2.0),
		)
	}
}
