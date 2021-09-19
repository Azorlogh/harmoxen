use harmoxen::{
	backend::Event,
	data::{icp, sheet::*, Range},
};
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use std::error::Error;
use std::sync::mpsc::*;
use std::thread;
use std::time::{Duration, Instant};

pub struct MidiBackend {
	to_backend: Sender<Event>,
}

impl MidiBackend {
	pub fn new(port: MidiOutputPort) -> Self {
		let (to_backend, from_server) = channel();
		thread::spawn(move || {
			if let Err(err) = run(from_server, port) {
				println!("Error with the mpe server: {}", err);
			}
		});
		Self { to_backend }
	}
}
impl super::Backend for MidiBackend {
	fn send(&mut self, evt: Event) {
		self.to_backend.send(evt).unwrap()
	}
}

const UPDATE_RATE: f32 = 0.04;

pub fn run(receiver: Receiver<Event>, port: MidiOutputPort) -> Result<(), Box<dyn Error>> {
	let mut engine = Engine::new(port)?;

	let mut last_instant = Instant::now();
	let mut until_update = 0.0;

	let mut running = true;
	while running {
		let new_instant = Instant::now();
		let dt = new_instant.duration_since(last_instant).as_nanos() as f32 / 1000000000.0;
		last_instant = new_instant;

		until_update += dt;

		while until_update > UPDATE_RATE as f32 {
			while let Ok(event) = receiver.try_recv() {
				match event {
					Event::SetTempo(t) => {
						engine.tempo = t;
					}
					Event::PlayStart(sheet, start) => {
						engine.cursor = start;
						engine.setup_mpe().unwrap();
						for event in sheet.get_events_at_time(start) {
							engine.process_icp(event);
						}
						engine.sheet = sheet;
						engine.active = true;
					}
					Event::PlayStop => {
						engine.process_icp(icp::Event::NoteStopAll);
						engine.active = false;
					}
					Event::SheetChanged(sheet) => {
						engine.sheet = sheet;
					}
					Event::ICP(event) => {
						engine.process_icp(event);
					}
					Event::Shutdown => {
						running = false;
					}
					_ => {}
				}
			}
			if engine.active {
				engine.update(UPDATE_RATE);
			}
			until_update -= UPDATE_RATE;
		}
		thread::sleep(Duration::from_millis((until_update * 1000.0) as u64));
	}
	Ok(())
}

const PITCH_BEND_RANGE: f32 = 2.0; // in semitones
const PITCH_BEND_SEMITONES: u8 = PITCH_BEND_RANGE as u8;
const PITCH_BEND_CENTS: u8 = ((PITCH_BEND_RANGE % 1.0) * 100.0) as u8;

#[derive(Clone, Copy, Default)]
struct Channel {
	current: Option<icp::NoteId>,
	note_number: u8,
}

struct Engine {
	conn: MidiOutputConnection,
	tempo: f32,
	active: bool,
	cursor: f32,
	sheet: Sheet,
	channels: Vec<Channel>,
}

impl Engine {
	pub fn new(port: MidiOutputPort) -> Result<Engine, Box<dyn Error>> {
		let midi_out = MidiOutput::new("midir mpe output")?;
		let conn = midi_out.connect(&port, "midir mpe")?;

		Ok(Engine {
			conn,
			tempo: 140.0,
			active: false,
			cursor: 0.0,
			sheet: Sheet::default(),
			channels: vec![Channel::default(); 15],
		})
	}

	pub fn setup_mpe(&mut self) -> Result<(), Box<dyn Error>> {
		println!("test!!");
		self.conn.send(&[0xB0, 127, 15])?;
		self.conn.send(&[0xB0, 124, 0])?; // omni off
		self.conn.send(&[0xB0, 127, 0])?; // poly on

		// SETUP ZONE
		self.conn.send(&[0xB0, 0x79, 0x00])?;
		self.conn.send(&[0xB0, 0x64, 0x06])?;
		self.conn.send(&[0xB0, 0x65, 0x00])?;
		self.conn.send(&[0xB0, 0x06, 0x07])?;

		for i in 0..16 {
			// PITCH BEND RANGE SETUP
			self.conn.send(&[0xB0 + i, 0x64, 0x00])?; // start control
			self.conn.send(&[0xB0 + i, 0x65, 0x00])?;
			self.conn.send(&[0xB0 + i, 0x06, PITCH_BEND_SEMITONES])?;
			self.conn.send(&[0xB0 + i, 0x26, PITCH_BEND_CENTS])?;
			self.conn.send(&[0xB0 + i, 0x64, 0x7F])?; // stop control
			self.conn.send(&[0xB0 + i, 0x65, 0x7F])?;
			self.conn.send(&[0xE0 + i, 0b0000000, 0b1000000])?;
		}

		Ok(())
	}

	pub fn update(&mut self, dt: f32) {
		let length = dt * (self.tempo / 60.0);
		let range = Range(self.cursor, self.cursor + length);
		self.cursor += length;
		let mut events = self.sheet.get_events(range);
		if self.cursor > self.sheet.get_size() {
			self.cursor %= self.sheet.get_size();
			events.extend(self.sheet.get_events(Range(0.0, self.cursor)));
		}
		for event in events {
			self.process_icp(event);
		}
	}

	fn process_icp(&mut self, event: icp::Event) {
		match event {
			icp::Event::NotePlay(note) => {
				let free = self.channels.iter().position(|x| x.current == None);
				if let Some(ch) = free {
					self.note_on(ch, note).unwrap();
				}
			}
			icp::Event::NoteStop(id) => {
				for ch in 0..self.channels.len() {
					if self.channels[ch].current == Some(id) {
						self.note_off(ch).unwrap();
					}
				}
			}
			icp::Event::NoteStopAll => {
				for ch in 0..self.channels.len() {
					self.note_off(ch).unwrap();
				}
			}
			icp::Event::NoteChangeFreq(id, freq) => {
				for ch in 0..self.channels.len() {
					let channel = self.channels[ch];
					if channel.current == Some(id) {
						let pitch_bend = ((freq / 440.0).log2() * 12.0 + 69.0) - channel.note_number as f32;
						if pitch_bend.abs() < PITCH_BEND_RANGE {
							self.conn.send(&pitch_bend_msg(ch, pitch_bend)).unwrap();
						} else {
							self.note_off(ch).unwrap();
							self.note_on(ch, icp::Note { id, freq }).unwrap();
						}
					}
				}
			}
		}
	}

	fn note_on(&mut self, ch: usize, note: icp::Note) -> Result<(), Box<dyn Error>> {
		let note_number = ((note.freq / 440.0).log2() * 12.0 + 69.0) as u8;
		let pitch_bend = ((note.freq / 440.0).log2() * 12.0 + 69.0) - note_number as f32;
		self.channels[ch].current = Some(note.id);
		self.channels[ch].note_number = note_number;
		self.conn.send(&pitch_bend_msg(ch, pitch_bend))?;
		self.conn.send(&[0x91 + ch as u8, note_number, 0x70])?;
		Ok(())
	}

	fn note_off(&mut self, ch: usize) -> Result<(), Box<dyn Error>> {
		self.channels[ch].current = None;
		self.conn.send(&[0x81 + ch as u8, self.channels[ch].note_number, 0x70])?;
		Ok(())
	}
}

fn pitch_bend_msg(ch: usize, t: f32) -> [u8; 3] {
	let n = (t * 8191.0 / PITCH_BEND_RANGE + 8192.0) as usize;
	[0xE1 + ch as u8, (n & 0b1111111) as u8, (n >> 7 & 0b1111111) as u8]
}
