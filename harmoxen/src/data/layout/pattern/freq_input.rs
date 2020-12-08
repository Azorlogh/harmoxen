use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::LayoutParseError;
use crate::data::layout::FreqPattern;

#[derive(Debug, Display, Clone, PartialEq, Eq)]
pub enum Mode {
	None,
	Equal,
	Enumeration,
	HarmonicSegment,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize)]
pub enum FreqInput {
	#[display(fmt = "None")]
	None,
	#[display(fmt = "Equal")]
	Equal { base: String, ndiv: String, interval: String },
	#[display(fmt = "Enumeration")]
	Enumeration { base: String, values: String },
	#[display(fmt = "Harmonic Segment")]
	HarmonicSegment { base: String, from: String, to: String },
}

impl Default for FreqInput {
	fn default() -> FreqInput {
		FreqInput::Equal {
			ndiv: "12".into(),
			interval: "2".into(),
			base: "440".into(),
		}
	}
}

impl FreqInput {
	pub fn default_none() -> Self {
		Self::None
	}
	pub fn default_equal() -> Self {
		Self::Equal {
			base: String::from("440"),
			ndiv: "12".into(),
			interval: "2".into(),
		}
	}
	pub fn default_enumeration() -> Self {
		Self::Enumeration {
			base: "440".into(),
			values: "38:40:43:46:48:51:54:57:61:64:68:72:76".into(),
		}
	}
	pub fn default_harmonic_segment() -> Self {
		Self::HarmonicSegment {
			base: "440".into(),
			from: "8".into(),
			to: "16".into(),
		}
	}

	pub fn mode(&self) -> Mode {
		match self {
			FreqInput::None => Mode::None,
			FreqInput::Equal { .. } => Mode::Equal,
			FreqInput::Enumeration { .. } => Mode::Enumeration,
			FreqInput::HarmonicSegment { .. } => Mode::HarmonicSegment,
		}
	}

	pub fn build(&self) -> Result<Option<FreqPattern>, LayoutParseError> {
		match self.clone() {
			FreqInput::None => Ok(None),
			FreqInput::Equal { base, ndiv, interval } => {
				let base = base.parse::<f32>()?;
				let ndiv = ndiv.parse::<usize>()?;
				let interval = interval.parse::<f32>()?;
				Ok(Some(FreqPattern::new(
					base,
					(0..ndiv + 1).map(|k| interval.powf(k as f32 / ndiv as f32)).collect(),
				)))
			}
			FreqInput::Enumeration { base, values } => {
				let base = base.parse::<f32>()?;
				let values = values
					.split(":")
					.map(|x| x.parse::<usize>())
					.collect::<Result<Vec<usize>, _>>()
					.map_err(|_| super::LayoutParseError)?;
				if values.len() == 0 {
					return Err(LayoutParseError);
				}

				let first = values[0] as f32;
				let values = values.iter().map(|&x| x as f32 / first).collect::<Vec<f32>>();
				Ok(Some(FreqPattern::new(base, values)))
			}
			FreqInput::HarmonicSegment { base, from, to } => {
				let base = base.parse::<f32>()?;
				let from = from.parse::<usize>()?;
				let to = to.parse::<usize>()?;
				if from >= to {
					return Err(LayoutParseError);
				}
				let values = (from..to + 1).map(|x| x as f32 / from as f32).collect::<Vec<f32>>();
				Ok(Some(FreqPattern::new(base, values)))
			}
		}
	}
}
