use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::LayoutParseError;
use crate::data::layout::FreqPattern;

#[derive(Clone, Debug, Display, Deserialize, Serialize)]
pub enum FreqInput {
	#[display(fmt = "None")]
	None,
	#[display(fmt = "Equal")]
	Equal {
		base: f32,
		ndiv: usize,
		interval: f32,
	},
	#[display(fmt = "Enumeration")]
	Enumeration { base: f32, enumeration: Enumeration },
	#[display(fmt = "Harmonic Segment")]
	HarmonicSegment { base: f32, from: usize, to: usize },
}

impl Default for FreqInput {
	fn default() -> FreqInput {
		FreqInput::Equal {
			ndiv: 12,
			interval: 2.0,
			base: 440.0,
		}
	}
}

impl FreqInput {
	pub fn build(&self) -> Result<Option<FreqPattern>, LayoutParseError> {
		match self.clone() {
			FreqInput::None => Ok(None),
			FreqInput::Equal {
				ndiv,
				interval,
				base,
			} => Ok(Some(FreqPattern::new(
				base,
				(0..ndiv + 1)
					.map(|k| interval.powf(k as f32 / ndiv as f32))
					.collect(),
			))),
			FreqInput::Enumeration { base, enumeration } => {
				if enumeration.0.len() == 0 {
					return Err(LayoutParseError);
				}
				let first = enumeration.0[0] as f32;
				let values = enumeration
					.0
					.iter()
					.map(|&x| x as f32 / first)
					.collect::<Vec<f32>>();
				Ok(Some(FreqPattern::new(base, values)))
			}
			FreqInput::HarmonicSegment { base, from, to } => {
				if from >= to {
					return Err(LayoutParseError);
				}
				let values = (from..to + 1)
					.map(|x| x as f32 / from as f32)
					.collect::<Vec<f32>>();
				Ok(Some(FreqPattern::new(base, values)))
			}
		}
	}
}

// Enumeration

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Enumeration(pub Vec<usize>);

impl std::str::FromStr for Enumeration {
	type Err = super::LayoutParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Enumeration(
			s.split(":")
				.map(|x| x.parse::<usize>())
				.collect::<Result<Vec<usize>, _>>()
				.map_err(|_| super::LayoutParseError)?,
		))
	}
}

impl std::fmt::Display for Enumeration {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self.0
				.iter()
				.map(|x| x.to_string())
				.collect::<Vec<String>>()
				.join(":")
		)
	}
}
