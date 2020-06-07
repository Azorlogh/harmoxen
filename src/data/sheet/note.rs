use druid::Point;
use generational_arena::Index;
use serde::{Deserialize, Serialize};

use super::Interval;

pub type Freq = f64;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Pitch {
	Absolute(f64),
	Relative(Index, Interval),
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Note {
	pub pitch: Pitch,
	pub start: f64,
	pub length: f64,
}

impl Note {
	pub fn new(pos: Point, note_len: f64) -> Note {
		Note {
			start: pos.x,
			length: note_len,
			pitch: Pitch::Absolute(2f64.powf(pos.y)),
		}
	}

	pub fn end(&self) -> f64 {
		self.start + self.length
	}
}
