use druid::{AppDelegate, Command, DelegateCtx, Env, Target, WindowId};

use std::rc::Rc;
use std::sync::mpsc::*;

use crate::commands as cmds;
use crate::server;
use crate::state::State;

pub struct Delegate {
	to_server: Sender<server::Event>,
	after_save: Option<Box<dyn Fn(&mut DelegateCtx)>>,
	midi_ports: Vec<midir::MidiOutputPort>,
}

impl Delegate {
	pub fn new(to_server: Sender<server::Event>) -> Delegate {
		Delegate {
			to_server,
			after_save: None,
			midi_ports: vec![],
		}
	}
}

mod fileops;
mod history;
mod layout;
mod settings;

impl AppDelegate<State> for Delegate {
	fn command(&mut self, ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut State, _env: &Env) -> bool {
		let mut project_changed = false;

		let mut propagate = true;

		propagate &= self.handle_history(ctx, cmd, data, &mut project_changed);
		propagate &= self.handle_fileops(ctx, cmd, data, &mut project_changed);
		propagate &= self.handle_settings(ctx, cmd, data, &mut project_changed);
		propagate &= self.handle_layout(ctx, cmd, data, &mut project_changed);

		propagate &= match cmd {
			_ if cmd.is(cmds::TEMPO_CHANGED) => {
				let tempo = *cmd.get_unchecked(cmds::TEMPO_CHANGED);
				self.to_server.send(server::Event::SetTempo(tempo)).unwrap();
				true
			}
			_ if cmd.is(cmds::PLAY_START) => {
				data.editors.sheet_editor.playing = true;
				self.to_server
					.send(server::Event::PlayStart(
						data.editors.sheet_editor.sheet.borrow().clone(),
						data.editors.sheet_editor.cursor,
					))
					.unwrap();
				true
			}
			_ if cmd.is(cmds::PLAY_STOP) => {
				data.editors.sheet_editor.playing = false;
				self.to_server.send(server::Event::PlayStop).unwrap();
				true
			}
			_ if cmd.is(cmds::ICP) => {
				let icp_event = *cmd.get_unchecked(cmds::ICP);
				self.to_server.send(server::Event::ICP(icp_event)).unwrap();
				false
			}

			_ if cmd.is(cmds::SHEET_CHANGED) => {
				self.to_server
					.send(server::Event::SheetChanged(data.editors.sheet_editor.sheet.borrow().clone()))
					.unwrap();
				project_changed = true;
				true
			}
			_ if cmd.is(cmds::LAYOUT_CHANGED) => {
				project_changed = true;
				true
			}
			_ => true,
		};
		if project_changed {
			ctx.submit_command(cmds::REDRAW.to(Target::Global));
		}
		propagate
	}

	fn window_added(&mut self, id: WindowId, data: &mut State, _env: &Env, _ctx: &mut DelegateCtx) {
		if data.main_window.is_none() {
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
