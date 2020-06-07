use derive_more::Display;
use druid::{Data, Lens};
use std::error::Error;

pub mod time_input;
use time_input::TimeInput;
pub mod freq_input;
use freq_input::FreqInput;

use crate::data::layout::Pattern;

#[derive(Clone, Data, Lens)]
pub struct State {
	pub time: TimeInput,
	pub freq: FreqInput,
}
impl Default for State {
	fn default() -> State {
		State {
			time: Default::default(),
			freq: Default::default(),
		}
	}
}

pub fn make_pattern(input: &State) -> Result<Pattern, LayoutParseError> {
	Ok(Pattern {
		time: time_input::make_time_pattern(&input.time)?,
		freq: freq_input::make_freq_pattern(&input.freq)?,
	})
}

// Errors

#[derive(Debug, Display)]
pub struct LayoutParseError;
impl Error for LayoutParseError {}
