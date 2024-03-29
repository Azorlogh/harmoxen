use harmoxen::data::icp::*;

mod adsr;
use adsr::ADSR;
mod limiter;
use limiter::Limiter;
mod osc;
mod svf;
use osc::Osc;

pub const ATTACK: f32 = 0.003;
pub const DECAY: f32 = 0.5;
pub const SUSTAIN: f32 = 0.5;
pub const RELEASE: f32 = 0.1;

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

	pub fn next(&mut self, delta: f32) -> f32 {
		let mut out = 0.0;
		out += self.osc.next(delta * self.note.freq);
		out *= self.adsr.sample();
		self.adsr.advance(delta);
		out
	}
}

pub struct Synth {
	pub period: f32,
	voices: Vec<Voice>,
	lowpass: svf::Kernel,
	limiter: Limiter,
}

impl Synth {
	pub fn new(period: f32) -> Synth {
		Synth {
			period,
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

	pub fn next_sample(&mut self) -> f32 {
		let mut out = 0.0;

		for voice in &mut self.voices {
			out += voice.next(self.period);
		}

		self.voices.retain(|voice| voice.adsr.state != adsr::Dead);

		out = self.lowpass.eval(out);
		out = self.limiter.eval(out * 0.2);
		out * 0.8
	}
}
