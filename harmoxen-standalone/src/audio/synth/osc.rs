const TAU: f32 = std::f32::consts::PI * 2.0;

#[derive(Clone)]
#[allow(dead_code)]
pub enum Mode {
	Sine,
	Saw,
	Square,
}

#[derive(Clone)]
pub struct Osc {
	pub mode: Mode,
	pub phase: f32,
}

use std::f32::consts::*;

impl Osc {
	pub fn new(mode: Mode) -> Osc {
		Osc { mode, phase: 0.0 }
	}

	pub fn next(&mut self, delta: f32) -> f32 {
		let out = match self.mode {
			Mode::Sine => (self.phase * TAU).sin(),
			Mode::Saw => (self.phase - 0.5) * 2.0 - 1.0 - poly_blep(self.phase, delta),
			Mode::Square => {
				(if self.phase < 0.5 { 1.0 } else { -1.0 }) + poly_blep(self.phase, delta) - poly_blep(self.phase, delta)
			}
		};
		self.phase += delta;
		if self.phase > 1. {
			self.phase -= 1.;
		}
		out
	}
}

fn poly_blep(mut t: f32, delta: f32) -> f32 {
	if t < delta {
		t /= delta;
		t + t - t * t - 1.0
	} else if t > 1.0 - delta {
		t = (t - 1.0) / delta;
		t * t + t + t + 1.0
	} else {
		0.0
	}
}
