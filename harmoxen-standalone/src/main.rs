use std::sync::mpsc::Sender;

mod audio;
mod midi;

fn main() {
	let server_sender = audio::launch().unwrap();

	start_ui(server_sender)
}

use iced_winit::{application, executor, Settings};

fn start_ui(to_server: Sender<harmoxen::BackendEvent>) {
	let settings = Settings {
		flags: to_server,
		window: iced_winit::settings::Window::default(),
	};
	application::run::<harmoxen::State, executor::Tokio, iced_wgpu::window::Compositor>(
		settings,
		iced_wgpu::settings::Settings::default(),
	)
	.unwrap();
}
