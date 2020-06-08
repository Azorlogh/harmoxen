#![feature(tau_constant)]

use druid::commands as sys_cmds;
use druid::{
	AppDelegate, AppLauncher, Command, DelegateCtx, Env, FileDialogOptions, FileSpec, LocalizedString, Size, Target,
	WindowDesc, WindowId,
};
use std::fs;
use std::rc::Rc;
use std::sync::mpsc::*;

mod commands;
mod data;
mod server;
mod state;
mod theme;
mod ui;
mod util;
mod widget;
use state::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let state = State::new();

	let to_server = server::launch()?;

	to_server
		.send(server::Event::SetTempo(state.editors.sheet_editor.tempo))
		.unwrap();

	let main_window = WindowDesc::new(|| ui::build())
		.title(LocalizedString::new("Harmoxen v0.2.0"))
		.window_size(Size::new(800.0, 500.0));

	let delegate = Delegate { to_server };

	AppLauncher::with_window(main_window)
		.delegate(delegate)
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
		let mut project_changed = false;
		let propagate = match cmd {
			_ if cmd.is(commands::TEMPO_CHANGED) => {
				let tempo = *cmd.get_unchecked(commands::TEMPO_CHANGED);
				self.to_server.send(server::Event::SetTempo(tempo)).unwrap();
				true
			}
			_ if cmd.is(commands::PLAY_START) => {
				data.editors.sheet_editor.playing = true;
				self.to_server
					.send(server::Event::PlayStart(
						data.editors.sheet_editor.sheet.borrow().clone(),
						data.editors.sheet_editor.cursor,
					))
					.unwrap();
				true
			}
			_ if cmd.is(commands::PLAY_STOP) => {
				data.editors.sheet_editor.playing = false;
				self.to_server.send(server::Event::PlayStop).unwrap();
				true
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
				if let Ok(()) = data.editors.apply_layout() {
					ctx.submit_command(commands::LAYOUT_CHANGED, Target::Global);
				}
				false
			}
			_ if cmd.is(commands::SHEET_CHANGED) => {
				self.to_server
					.send(server::Event::SheetChanged(data.editors.sheet_editor.sheet.borrow().clone()))
					.unwrap();
				project_changed = true;
				true
			}
			_ if cmd.is(commands::LAYOUT_CHANGED) => {
				project_changed = true;
				true
			}
			_ if cmd.is(commands::HISTORY_SAVE) => {
				let project = state::Project::from_editors(&data.editors);
				data.history.borrow_mut().save(project);
				false
			}
			_ if cmd.is(commands::HISTORY_UNDO) => {
				let project = data.history.borrow_mut().undo();
				project.open(&mut data.editors);
				ctx.submit_command(commands::REDRAW, Target::Global);
				false
			}
			_ if cmd.is(commands::HISTORY_REDO) => {
				let project = data.history.borrow_mut().redo();
				project.open(&mut data.editors);
				ctx.submit_command(commands::REDRAW, Target::Global);
				false
			}
			_ if cmd.is(commands::PROJECT_NEW) => {
				let mut state = State::new();
				state.main_window = data.main_window.clone();
				*data = state;
				false
			}
			_ if cmd.is(commands::PROJECT_OPEN) => {
				ctx.submit_command(
					Command::new(
						sys_cmds::SHOW_OPEN_PANEL,
						FileDialogOptions::new().allowed_types(vec![FileSpec::new("Harmoxen project", &["hxp"])]),
					),
					Target::Window(*data.main_window.clone().unwrap()),
				);
				false
			}
			_ if cmd.is(commands::PROJECT_SAVE_AS) => {
				ctx.submit_command(
					Command::new(
						sys_cmds::SHOW_SAVE_PANEL,
						FileDialogOptions::new().allowed_types(vec![FileSpec::new("Harmoxen project", &["hxp"])]),
					),
					Target::Window(*data.main_window.clone().unwrap()),
				);
				false
			}
			_ if cmd.is(commands::PROJECT_SAVE) => {
				if let Some(path) = data.save_path.clone() {
					let project = state::Project::from_editors(&data.editors);
					let data = ron::to_string(&project).unwrap();
					fs::write(&*path, data).ok();
				} else {
					let xrp = FileSpec::new("Harmoxen project", &["hxp"]);
					ctx.submit_command(
						Command::new(
							sys_cmds::SHOW_SAVE_PANEL,
							FileDialogOptions::new().allowed_types(vec![xrp]).default_type(xrp),
						),
						Target::Window(*data.main_window.clone().unwrap()),
					);
				}
				false
			}
			_ if cmd.is(sys_cmds::SAVE_FILE) => {
				if let Some(file_info) = cmd.get_unchecked(sys_cmds::SAVE_FILE) {
					data.save_path = Some(Rc::new(file_info.path().into()));
					let project = state::Project::from_editors(&data.editors);
					let data = ron::to_string(&project).unwrap();
					fs::write(file_info.path(), data).ok();
				}
				true
			}
			_ if cmd.is(sys_cmds::OPEN_FILE) => {
				let file_info = cmd.get_unchecked(sys_cmds::OPEN_FILE);
				if let Ok(project_string) = fs::read_to_string(file_info.path()) {
					if let Ok(project) = ron::from_str::<state::Project>(&project_string) {
						project.open(&mut data.editors);
						data.save_path = Some(Rc::new(file_info.path().into()));
						ctx.submit_command(commands::REDRAW, Target::Global);
					}
				}
				true
			}
			_ => true,
		};
		if project_changed {
			ctx.submit_command(commands::REDRAW, Target::Global);
		}
		propagate
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
