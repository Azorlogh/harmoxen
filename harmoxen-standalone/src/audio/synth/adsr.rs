#[derive(PartialEq)]
pub enum State {
	Attack(f32),
	Decay(f32),
	Sustain,
	Release(f32),
	Dead,
}
pub use State::*;

pub struct ADSR {
	attack: f32,
	decay: f32,
	sustain: f32,
	release: f32,
	pub state: State,
}

impl ADSR {
	pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> ADSR {
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

	pub fn sample(&self) -> f32 {
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

	pub fn advance(&mut self, x: f32) {
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
