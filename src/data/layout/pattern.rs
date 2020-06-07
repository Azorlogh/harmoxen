use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimePattern {
	pub values: Vec<f64>,
	pub nbeats: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FreqPattern {
	pub values: Vec<f64>,
	pub base: f64,
}

impl FreqPattern {
	pub fn new(base: f64, values: Vec<f64>) -> FreqPattern {
		FreqPattern { base, values }.normalize()
	}

	fn normalize(mut self) -> FreqPattern {
		let period = self.period();
		for i in 0..(self.values.len() - 1) {
			self.values[i] = self.values[i] * period.powf((1.0 / self.values[i]).log(period).ceil());
		}
		self
	}

	pub fn period(&self) -> f64 {
		self.values[self.values.len() - 1] / self.values[0]
	}
}

// the elements of each component are assumed to be sorted
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pattern {
	pub time: Option<TimePattern>,
	pub freq: Option<FreqPattern>,
}

impl Pattern {
	pub const EMPTY: Pattern = Pattern { time: None, freq: None };
}

impl Default for Pattern {
	fn default() -> Pattern {
		Pattern::EMPTY
	}
}
