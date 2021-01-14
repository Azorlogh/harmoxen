use std::sync::mpsc::Sender;

mod audio;
mod midi;

fn main() {
	let server_sender = audio::launch().unwrap();

	start_ui(server_sender)
}

// use iced_winit::{application, executor, Settings};
use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use iced_baseview::*;

fn start_ui(to_server: Sender<harmoxen::BackendEvent>) {
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
