#[derive(PartialEq)]
pub enum State {
	Attack(f64),
	Decay(f64),
	Sustain,
	Release(f64),
	Dead,
}
pub use State::*;

pub struct ADSR {
	attack: f64,
	decay: f64,
	sustain: f64,
	release: f64,
	pub state: State,
}

impl ADSR {
	pub fn new(attack: f64, decay: f64, sustain: f64, release: f64) -> ADSR {
		let mut adsr = ADSR {
			attack,
			decay,
			sustain,
			release,
			state: Attack(0.0),
		};
		adsr.advance(0.0);
		adsr
	}

	pub fn sample(&self) -> f64 {
		match self.state {
			Attack(t) => t / self.attack,
			Decay(t) => 1.0 - (1.0 - self.sustain) * t / self.decay,
			Sustain => self.sustain,
			Release(t) => self.sustain * (1.0 - t / self.release),
			Dead => 0.0,
		}
	}

	pub fn release(&mut self) {
		self.sustain = self.sample();
		self.state = Release(0.0);
	}

	pub fn advance(&mut self, x: f64) {
		self.state = match self.state {
			Attack(t) => {
				if t + x >= self.attack {
					Decay(0.0)
				} else {
					Attack(t + x)
				}
			}
			Decay(t) => {
				if t + x > self.decay {
					Sustain
				} else {
					Decay(t + x)
				}
			}
			Sustain => Sustain,
			Release(t) => {
				if t + x > self.release {
					Dead
				} else {
					Release(t + x)
				}
			}
			Dead => Dead,
		};
	}
}
