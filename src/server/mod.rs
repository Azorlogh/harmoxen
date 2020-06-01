use std::error::Error;
use std::sync::mpsc::*;
use std::thread;

use crate::icp;
use crate::state::sheet_editor::sheet::*;
use crate::util::*;

pub fn launch() -> Result<Sender<Event>, Box<dyn Error>> {
	let (sender, receiver) = channel();
	thread::spawn(move || {
		run(receiver);
	});

	Ok(sender)
}

#[derive(Debug)]
pub enum Event {
	SetTempo(f64),
	PlayStart(Sheet, f64),
	PlayStop,
	SheetChanged(Sheet),
	ICP(icp::Event),
}

pub fn run(receiver: Receiver<Event>) {
	use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

	let host = cpal::default_host();

	let device = host.default_output_device().expect("no output device available");
	let format = device.default_output_format().unwrap();

	let event_loop = host.event_loop();
	let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

	event_loop.play_stream(stream_id.clone()).unwrap();

	let sample_rate = format.sample_rate.0 as f64;

	let mut engine = Engine::new(sample_rate);

	event_loop.run(move |id, result| {
		while let Ok(event) = receiver.try_recv() {
			engine.process_event(event);
		}

		let data = match result {
			Ok(data) => data,
			Err(err) => {
				eprintln!("an error occurred on stream {:?}: {}", id, err);
				return;
			}
		};

		match data {
			cpal::StreamData::Output {
				buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
			} => {
				engine.update(buffer.len() / format.channels as usize);
				for sample in buffer.chunks_mut(format.channels as usize) {
					let value = engine.next_sample();
					for out in sample.iter_mut() {
						*out = value as f32;
					}
				}
			}
			_ => (),
		}
	});
}

mod synth;
use synth::Synth;

struct Engine {
	sample_rate: f64,
	sheet: Sheet,
	cursor: f64,
	active: bool,
	synth: Synth,
	tempo: f64,
}

impl Engine {
	pub fn new(sample_rate: f64) -> Engine {
		Engine {
			sample_rate,
			sheet: Sheet::default(),
			cursor: 0.0,
			active: false,
			synth: Synth::new(sample_rate),
			tempo: 140.0,
		}
	}

	pub fn process_event(&mut self, event: Event) {
		match event {
			Event::SetTempo(tempo) => {
				self.tempo = tempo;
			}
			Event::PlayStart(sheet, cursor) => {
				self.cursor = cursor;
				self.synth.process_events(&sheet.get_events_at_time(cursor));
				self.sheet = sheet;
				self.active = true;
			}
			Event::PlayStop => {
				self.active = false;
				self.synth.process_events(&[icp::Event::NoteStopAll]);
			}
			Event::SheetChanged(sheet) => {
				self.sheet = sheet;
			}
			Event::ICP(icp) => {
				self.synth.process_events(&[icp]);
			}
		}
	}

	/*
		length => 1s
		?      =>
	*/

	pub fn update(&mut self, samples: usize) {
		if self.active {
			let length = samples as f64 / self.sample_rate * (self.tempo / 60.0);
			let range = Range(self.cursor, self.cursor + length);
			self.cursor += length;
			let mut events = self.sheet.get_events(range);
			if self.cursor > self.sheet.get_size() {
				self.cursor %= self.sheet.get_size();
				events.extend(self.sheet.get_events(Range(0.0, self.cursor)));
			}
			self.synth.process_events(&events);
		}
	}

	pub fn next_sample(&mut self) -> f64 {
		self.synth.next_sample()
	}
}
