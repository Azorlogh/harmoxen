use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimePattern {
	pub values: Vec<f32>,
	pub nbeats: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FreqPattern {
	pub values: Vec<f32>,
	pub base: f32,
}

impl FreqPattern {
	pub fn new(base: f32, values: Vec<f32>) -> FreqPattern {
		FreqPattern { base, values }.normalize()
	}

	fn normalize(mut self) -> FreqPattern {
		let period = self.period();
		for i in 0..(self.values.len() - 1) {
			self.values[i] =
				self.values[i] * period.powf((1.0 / self.values[i]).log(period).ceil());
		}
		self
	}

	pub fn period(&self) -> f32 {
		self.values[self.values.len() - 1] / self.values[0]
	}
}

// the elements of each component are assumed to be sorted
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Pattern {
	pub time: Option<TimePattern>,
	pub freq: Option<FreqPattern>,
}

impl Pattern {
	pub const EMPTY: Pattern = Pattern {
		time: None,
		freq: None,
	};
}

//

use derive_more::Display;
use std::error::Error;

pub mod time_input;
use time_input::TimeInput;
pub mod freq_input;
use freq_input::FreqInput;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatternInput {
	pub time: TimeInput,
	pub freq: FreqInput,
}
impl Default for PatternInput {
	fn default() -> PatternInput {
		PatternInput {
			time: Default::default(),
			freq: Default::default(),
		}
	}
}

impl PatternInput {
	pub fn build(&self) -> Result<Pattern, LayoutParseError> {
		Ok(Pattern {
			time: self.time.build()?,
			freq: self.freq.build()?,
		})
	}
}

// Errors

#[derive(Debug, Display)]
pub struct LayoutParseError;
impl Error for LayoutParseError {}
