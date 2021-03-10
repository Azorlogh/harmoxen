#![allow(unused)]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::error::Error;
use std::sync::mpsc::*;
use std::thread;

use harmoxen::backend::Event;
// use crate::util::*;
use harmoxen::data::{icp, sheet::*, Range};

pub fn launch() -> Result<Sender<Event>, Box<dyn Error>> {
	let (to_server, from_frontend) = channel();
	let (to_backend, from_server) = channel();
	thread::spawn(move || {
		let mut stream = run(from_server);
		while let Ok(event) = from_frontend.recv() {
			if let Event::Shutdown = event {
				stream.pause();
				break;
			}
			to_backend
				.send(event)
				.map_err(|e| format!("connection to audio backend closed unexpectedly: {}", e))
				.unwrap();
		}
	});
	Ok(to_server)
}

pub fn run(mut receiver: Receiver<Event>) -> Box<dyn StreamTrait> {
	let host = cpal::default_host();

	let device = host.default_output_device().expect("failed to find a default output device");
	let supported_config = device.default_output_config().expect("failed to get default output config");
	let config = supported_config.config();

	let nb_channels = config.channels as usize;

	let period = 1.0 / f64::from(config.sample_rate.0) as f32;

	let mut engine = Engine::new(period);

	let stream = match supported_config.sample_format() {
		cpal::SampleFormat::F32 => build_stream::<f32>(device, receiver, engine, config),
		cpal::SampleFormat::I16 => build_stream::<i16>(device, receiver, engine, config),
		cpal::SampleFormat::U16 => build_stream::<u16>(device, receiver, engine, config),
	};

	stream.play().expect("failed to play audio stream");

	stream
}

fn build_stream<T>(
	device: cpal::Device,
	mut receiver: Receiver<Event>,
	mut engine: Engine,
	config: cpal::StreamConfig,
) -> Box<dyn StreamTrait>
where
	T: cpal::Sample,
{
	let nb_channels = config.channels as usize;
	Box::new(
		device
			.build_output_stream::<T, _, _>(
				&config,
				move |data, _| {
					while let Ok(event) = receiver.try_recv() {
						engine.process_event(event);
					}

					let data_len = data.len() / nb_channels as usize;
					let mut i = 0;
					for frame in data.chunks_mut(nb_channels) {
						if i % 256 == 0 {
							engine.update((data_len - i).min(256));
						}
						i += 1;
						let value = cpal::Sample::from::<f32>(&(engine.next_sample() as f32));
						for sample in frame.iter_mut() {
							*sample = value;
						}
					}
				},
				|err| println!("an error occured on stream: {}", err),
			)
			.expect("failed to build the output stream"),
	)
}

mod synth;
use synth::Synth;

struct Engine {
	sheet: Sheet,
	cursor: f32,
	active: bool,
	synth: Synth,
	tempo: f32,
}

impl Engine {
	pub fn new(period: f32) -> Engine {
		Engine {
			sheet: Sheet::default(),
			cursor: 0.0,
			active: false,
			synth: Synth::new(period),
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
			_ => {}
		}
	}

	pub fn update(&mut self, samples: usize) {
		if self.active {
			let length = samples as f32 * self.synth.period * (self.tempo / 60.0);
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

	pub fn next_sample(&mut self) -> f32 {
		self.synth.next_sample()
	}
}
