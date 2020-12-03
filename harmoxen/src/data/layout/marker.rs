use super::{Pattern, PatternInput};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Marker {
	pub at: f32,
	pub pattern: Pattern,
	pub pattern_input: PatternInput,
}

impl Default for Marker {
	fn default() -> Self {
		let input = PatternInput::default();
		Self {
			at: 0.0,
			pattern: input.build().unwrap(),
			pattern_input: input,
		}
	}
}
