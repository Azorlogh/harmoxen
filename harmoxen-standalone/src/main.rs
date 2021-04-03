use std::sync::mpsc::*;

mod audio;
mod midi;

fn main() {
	// let server_sender = audio::launch().unwrap();

	let (to_server, from_frontend) = channel::<harmoxen::Event>();

	std::thread::spawn(|| {


		
		let output = midir::MidiOutput::new("harmoxen MIDI output").unwrap();
		let port = output.ports().drain(..).skip(1).next().unwrap();
		print!("{:?}", output.port_name(&port));

		let server_sender = midi::launch(port).unwrap();
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
		flags: to_server,
	};
	iced_baseview::IcedWindow::<harmoxen::State>::open_blocking(settings);
	// application::run::<harmoxen::State, executor::Tokio, iced_wgpu::window::Compositor>(
	// 	settings,
	// 	iced_wgpu::settings::Settings::default(),
	// )
	// .unwrap();
}
