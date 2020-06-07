use derive_more::Display;
use druid::Data;
use std::rc::Rc;

use super::LayoutParseError;
use crate::data::layout::FreqPattern;

pub fn make_freq_pattern(input: &FreqInput) -> Result<Option<FreqPattern>, LayoutParseError> {
	match input.clone() {
		FreqInput::None => Ok(None),
		FreqInput::Equal { ndiv, interval, base } => {
			// let min = (ndiv as f64 * (20.0 / base).log(interval)).ceil() as isize;
			// let max = (ndiv as f64 * (20000.0 / base).log(interval)).ceil() as isize;
			Ok(Some(FreqPattern::new(
				base,
				(0..ndiv + 1).map(|k| interval.powf(k as f64 / ndiv as f64)).collect(),
			)))
		}
		FreqInput::Enumeration { base, enumeration } => {
			if enumeration.0.len() == 0 {
				return Err(LayoutParseError);
			}
			let first = enumeration.0[0] as f64;
			let values = enumeration.0.iter().map(|&x| x as f64 / first).collect::<Vec<f64>>();
			Ok(Some(FreqPattern::new(base, values)))
		}
		FreqInput::HarmonicSegment { base, from, to } => {
			if from >= to {
				return Err(LayoutParseError);
			}
			let values = (from..to).map(|x| x as f64 / from as f64).collect::<Vec<f64>>();
			Ok(Some(FreqPattern::new(base, values)))
		}
	}
}

/*

*/

#[derive(Clone, Data, Display)]
pub enum FreqInput {
	#[display(fmt = "None")]
	None,
	#[display(fmt = "Equal")]
	Equal { base: f64, ndiv: usize, interval: f64 },
	#[display(fmt = "Enumeration")]
	Enumeration { base: f64, enumeration: Enumeration },
	#[display(fmt = "Harmonic Segment")]
	HarmonicSegment { base: f64, from: usize, to: usize },
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

// Enumeration

#[derive(Clone, Data)]
pub struct Enumeration(pub Rc<Vec<usize>>);

impl std::str::FromStr for Enumeration {
	type Err = super::LayoutParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Enumeration(Rc::new(
			s.split(":")
				.map(|x| x.parse::<usize>())
				.collect::<Result<Vec<usize>, _>>()
				.map_err(|_| super::LayoutParseError)?,
		)))
	}
}

impl std::fmt::Display for Enumeration {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			self.0.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(":")
		)
	}
}
