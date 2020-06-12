#![feature(tau_constant)]

use druid::{AppLauncher, LocalizedString, Size, WindowDesc};

mod commands;
mod data;
mod server;
mod state;
mod theme;
mod ui;
mod util;
mod widget;
use state::State;
mod delegate;
use delegate::Delegate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let state = State::new();

	let to_server = server::launch()?;

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
		.use_simple_logger()
		.launch(state)
		.expect("launch failed");
	Ok(())
}
