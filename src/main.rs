#![feature(tau_constant)]

use druid::{AppDelegate, AppLauncher, Command, DelegateCtx, Env, LocalizedString, Size, Target, WindowDesc, WindowId};
use std::rc::Rc;
use std::sync::mpsc::*;

mod state;
mod theme;
use state::State;
mod commands;
mod icp;
mod server;
mod ui;
mod util;
mod widget;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut state = State::default();

	state::apply_layout(&mut state).unwrap();

	let to_server = server::launch()?;

	to_server.send(server::Event::SetTempo(state.sheet_editor.tempo)).unwrap();

	let main_window = WindowDesc::new(|| ui::build())
		.title(LocalizedString::new("Universal Piano Roll"))
		.window_size(Size::new(800.0, 500.0));

	let delegate = Delegate { to_server };

	AppLauncher::with_window(main_window)
		.delegate(delegate)
		.use_simple_logger()
		.configure_env(theme::apply)
		.launch(state)
		.expect("launch failed");
	Ok(())
}

struct Delegate {
	to_server: Sender<server::Event>,
}

impl AppDelegate<State> for Delegate {
	fn command(&mut self, ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut State, _env: &Env) -> bool {
		match cmd {
			_ if cmd.is(commands::TEMPO_CHANGED) => {
				let tempo = *cmd.get_unchecked(commands::TEMPO_CHANGED);
				self.to_server.send(server::Event::SetTempo(tempo)).unwrap();
				true
			}
			_ if cmd.is(commands::PLAY_START) => {
				data.sheet_editor.playing = true;
				self.to_server
					.send(server::Event::PlayStart(
						data.sheet_editor.sheet.borrow().clone(),
						data.sheet_editor.cursor,
					))
					.unwrap();
				true
			}
			_ if cmd.is(commands::PLAY_STOP) => {
				data.sheet_editor.playing = false;
				self.to_server.send(server::Event::PlayStop).unwrap();
				true
			}
			_ if cmd.is(commands::SHEET_CHANGED) => {
				self.to_server
					.send(server::Event::SheetChanged(data.sheet_editor.sheet.borrow().clone()))
					.unwrap();
				false
			}
			_ if cmd.is(commands::ICP) => {
				let icp_event = *cmd.get_unchecked(commands::ICP);
				self.to_server.send(server::Event::ICP(icp_event)).unwrap();
				false
			}
			_ if cmd.is(commands::OPEN_LAYOUT_EDITOR) => {
				let new_win = WindowDesc::new(|| ui::build_layout_editor())
					.title(LocalizedString::new("Layout input"))
					.window_size((800.0, 300.0));
				ctx.new_window(new_win);
				false
			}
			_ if cmd.is(commands::LAYOUT_APPLY) => {
				if let Ok(()) = state::apply_layout(data) {
					ctx.submit_command(commands::LAYOUT_CHANGED, Target::Global);
				}
				false
			}
			_ => true,
		}
	}

	fn window_added(&mut self, id: WindowId, data: &mut State, _env: &Env, _ctx: &mut DelegateCtx) {
		if let None = data.main_window {
			data.main_window = Some(Rc::new(id));
		}
	}

	fn window_removed(&mut self, id: WindowId, data: &mut State, _env: &Env, _ctx: &mut DelegateCtx) {
		let main_id = (data.main_window.clone()).unwrap();
		if id == *main_id {
			std::process::exit(0);
		}
	}
}
