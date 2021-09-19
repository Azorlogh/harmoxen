use std::sync::mpsc::*;

mod audio;
mod midi;

use harmoxen::{Backend, Event};

fn main() {
	let (to_server, from_frontend) = channel::<Event>();

	std::thread::spawn(move || {
		let mut backend: Box<dyn Backend> = Box::new(audio::AudioBackend::new());

		while let Ok(event) = from_frontend.recv() {
			match event {
				Event::ChangeBackend(harmoxen::BackendId::Audio) => {
					backend = Box::new(audio::AudioBackend::new());
				}
				Event::ChangeBackend(harmoxen::BackendId::Midi(port)) => {
					let output = midir::MidiOutput::new("harmoxen MIDI output").unwrap();
					let port = output.ports().drain(..).skip(port).next().unwrap();
					print!("{:?}", output.port_name(&port));
					backend = Box::new(midi::MidiBackend::new(port));
				}
				Event::ToBackend(evt) => backend.send(evt),
			}
		}
	});

	start_ui(to_server);
}

// use iced_winit::{application, executor, Settings};
use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use iced_baseview::*;

fn start_ui(to_server: Sender<harmoxen::Event>) {
	let settings = Settings {
		window: WindowOpenOptions {
			title: "Harmoxen".into(),
			size: Size::new(500.0, 300.0),
			scale: WindowScalePolicy::SystemScaleFactor,
		},
		flags: harmoxen::Flags { to_server },
	};
	iced_baseview::IcedWindow::<harmoxen::State>::open_blocking(settings);
	// application::run::<harmoxen::State, executor::Tokio, iced_wgpu::window::Compositor>(
	// 	settings,
	// 	iced_wgpu::settings::Settings::default(),
	// )
	// .unwrap();
}
