use druid::{AppLauncher, LocalizedString, Size, WindowDesc};

#[macro_use]
mod util;

mod commands;
mod data;
mod server;
mod state;
mod theme;
mod ui;
mod widget;
use state::State;
mod delegate;
use delegate::Delegate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let state = State::new();

	let to_server = server::audio::launch()?;

	to_server
		.send(server::Event::SetTempo(state.editors.sheet_editor.tempo))
		.unwrap();

	let main_window = WindowDesc::new(|| ui::build())
		.title(LocalizedString::new("Harmoxen v0.2.0"))
		.window_size(Size::new(800.0, 500.0));

	let delegate = Delegate::new(to_server);

	AppLauncher::with_window(main_window)
		.delegate(delegate)
		.configure_env(theme::apply)
		.launch(state)
		.expect("launch failed");
	Ok(())
}
