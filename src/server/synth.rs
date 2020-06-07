use crate::data::icp::*;

mod adsr;
use adsr::ADSR;
mod limiter;
use limiter::Limiter;
mod osc;
mod svf;
use osc::Osc;

pub const ATTACK: f64 = 0.2;
pub const DECAY: f64 = 0.1;
pub const SUSTAIN: f64 = 0.8;
pub const RELEASE: f64 = 0.1;

struct Voice {
	osc: Osc,
	note: Note,
	adsr: ADSR,
}

impl Voice {
	pub fn new(note: Note) -> Voice {
		Voice {
			osc: Osc::new(osc::Mode::Saw),
			note,
			adsr: ADSR::new(ATTACK, DECAY, SUSTAIN, RELEASE),
		}
	}

	pub fn next(&mut self, delta: f64) -> f64 {
		let mut out = 0.0;
		out += self.osc.next(delta * self.note.freq);
		out *= self.adsr.sample();
		self.adsr.advance(delta);
		out
	}
}

pub struct Synth {
	sample_rate: f64,
	voices: Vec<Voice>,
	lowpass: svf::Kernel,
	limiter: Limiter,
}

impl Synth {
	pub fn new(sample_rate: f64) -> Synth {
		Synth {
			sample_rate,
			voices: vec![],
			lowpass: svf::lowpass(0.02, 0.3),
			limiter: Limiter::new(),
		}
	}

	pub fn add_voice(&mut self, note: Note) {
		self.voices.push(Voice::new(note));
	}

	pub fn process_events(&mut self, events: &[Event]) {
		for &event in events {
			match event {
				Event::NotePlay(note) => {
					self.add_voice(note);
				}
				Event::NoteStop(note_id) => {
					for voice in &mut self.voices {
						if voice.note.id == note_id {
							voice.adsr.release();
						}
					}
				}
				Event::NoteStopAll => {
					self.voices = vec![];
				}
				Event::NoteChangeFreq(note_id, freq) => {
					for voice in &mut self.voices {
						if voice.note.id == note_id {
							voice.note.freq = freq;
						}
					}
				}
			}
		}
	}

	pub fn next_sample(&mut self) -> f64 {
		let mut out = 0.0;

		for voice in &mut self.voices {
			out += voice.next(1.0 / self.sample_rate);
		}

		self.voices.retain(|voice| voice.adsr.state != adsr::Dead);

		out = self.lowpass.eval(out);
		out = self.limiter.eval(out);
		out * 0.8
	}
}
