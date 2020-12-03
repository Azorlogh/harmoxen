use derive_more::Display;

use super::LayoutParseError;
use crate::data::layout::TimePattern;

pub fn make_time_pattern(input: &TimeInput) -> Result<Option<TimePattern>, LayoutParseError> {
	match input.clone() {
		TimeInput::None => Ok(None),
		TimeInput::Regular { ndiv, nbeats } => Ok(Some(TimePattern {
			values: (0..ndiv).map(|k| k as f32 / ndiv as f32).collect(),
			nbeats,
		})),
		TimeInput::Formula {
			ndiv,
			nbeats,
			formula,
		} => {
			let expr: meval::Expr = formula.parse().map_err(|_| LayoutParseError)?;
			let func = expr.bind("i").map_err(|_| LayoutParseError)?;
			Ok(Some(TimePattern {
				values: (0..ndiv).map(|i| func(i as f64) as f32).collect(),
				nbeats,
			}))
		}
		TimeInput::Poly {
			ndiv0,
			ndiv1,
			nbeats,
		} => {
			if ndiv0 == 0 || ndiv1 == 0 {
				return Err(LayoutParseError);
			}
			let mut out: Vec<f32> = (0..ndiv0)
				.map(|k| k as f32 / ndiv0 as f32)
				.chain((1..ndiv1).map(|k| k as f32 / ndiv1 as f32))
				.collect();
			out.sort_by(|a, b| a.partial_cmp(b).unwrap());
			Ok(Some(TimePattern {
				values: out,
				nbeats,
			}))
		}
	}
}

#[derive(Clone, Display)]
pub enum TimeInput {
	#[display(fmt = "None")]
	None,
	#[display(fmt = "Regular")]
	Regular { ndiv: usize, nbeats: usize },
	#[display(fmt = "Poly")]
	Poly {
		ndiv0: usize,
		ndiv1: usize,
		nbeats: usize,
	},
	#[display(fmt = "Formula")]
	Formula {
		ndiv: usize,
		nbeats: usize,
		formula: String,
	},
}

impl Default for TimeInput {
	fn default() -> TimeInput {
		TimeInput::Regular { ndiv: 4, nbeats: 4 }
	}
}
