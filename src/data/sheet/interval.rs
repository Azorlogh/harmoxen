use derive_more::Display;
use druid::Data;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq, Data, Deserialize, Serialize)]
pub enum Interval {
	Ratio(usize, usize),
	Float(f64),
}

impl Mul<Interval> for f64 {
	type Output = f64;
	fn mul(self, interval: Interval) -> f64 {
		match interval {
			Interval::Ratio(num, denom) => self * (num as f64 / denom as f64),
			Interval::Float(x) => self * x,
		}
	}
}

#[derive(Debug, Display)]
pub struct IntervalParseError;
impl Error for IntervalParseError {}

impl std::str::FromStr for Interval {
	type Err = IntervalParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts = s.split('/').collect::<Vec<&str>>();
		match parts.len() {
			1 => Ok(Interval::Float(parts[0].parse::<f64>().map_err(|_| IntervalParseError)?)),
			2 => Ok(Interval::Ratio(
				parts[0].parse::<usize>().map_err(|_| IntervalParseError)?,
				parts[1].parse::<usize>().map_err(|_| IntervalParseError)?,
			)),
			_ => Err(IntervalParseError),
		}
	}
}

use std::fmt;

impl fmt::Display for Interval {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match &self {
			Interval::Ratio(num, denom) => write!(f, "{}/{}", num, denom),
			Interval::Float(x) => write!(f, "{}", x),
		}
	}
}
