use druid::commands as sys_cmds;
use druid::{
	AppDelegate, Command, DelegateCtx, Env, FileDialogOptions, FileSpec, LocalizedString, Selector, Target, WindowDesc,
	WindowId,
};

use std::fs;
use std::rc::Rc;
use std::sync::mpsc::*;

use crate::commands as cmds;
use crate::server;
use crate::state::{self, State};
use crate::ui;
use crate::widget;

pub const IMPL_PROJECT_NEW: Selector = Selector::new("delegate.project-new");
pub const IMPL_PROJECT_OPEN: Selector = Selector::new("delegate.project-open");

pub struct Delegate {
	to_server: Sender<server::Event>,
	after_save: Option<Box<dyn Fn(&mut DelegateCtx)>>,
}

impl Delegate {
	pub fn new(to_server: Sender<server::Event>) -> Delegate {
		Delegate {
			to_server,
			after_save: None,
		}
	}
}

impl AppDelegate<State> for Delegate {
	fn command(&mut self, ctx: &mut DelegateCtx, _target: Target, cmd: &Command, data: &mut State, _env: &Env) -> bool {
		let main_window = *data.main_window.clone().unwrap();
		let mut project_changed = false;
		let propagate = match cmd {
			_ if cmd.is(cmds::BACKEND_SET_AUDIO) => {
				self.to_server.send(server::Event::Shutdown).unwrap();
				self.to_server = server::audio::launch().unwrap();
				false
			}
			_ if cmd.is(cmds::BACKEND_SET_MPE) => {
				self.to_server.send(server::Event::Shutdown).unwrap();
				self.to_server = server::midi::launch().unwrap();
				false
			}
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
			_ if cmd.is(cmds::OPEN_LAYOUT_EDITOR) => {
				let new_win = WindowDesc::new(|| ui::build_layout_editor())
					.title(LocalizedString::new("Layout input"))
					.window_size((800.0, 300.0));
				ctx.new_window(new_win);
				false
			}
			_ if cmd.is(cmds::LAYOUT_APPLY) => {
				if let Ok(()) = data.editors.apply_layout() {
					ctx.submit_command(cmds::LAYOUT_CHANGED, Target::Global);
				}
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
			_ if cmd.is(cmds::HISTORY_SAVE) => {
				let project = state::Project::from_editors(&data.editors);
				data.history.borrow_mut().save(project);
				data.up_to_date = false;
				false
			}
			_ if cmd.is(cmds::HISTORY_UNDO) => {
				let project = data.history.borrow_mut().undo();
				project.open(&mut data.editors);
				project_changed = true;
				false
			}
			_ if cmd.is(cmds::HISTORY_REDO) => {
				let project = data.history.borrow_mut().redo();
				project.open(&mut data.editors);
				project_changed = true;
				false
			}
			_ if cmd.is(cmds::PROJECT_NEW) => {
				if data.up_to_date {
					ctx.submit_command(IMPL_PROJECT_NEW, None);
				} else {
					ctx.submit_command(
						Command::new(widget::overlay::SHOW_MIDDLE, ui::modal::save::build(IMPL_PROJECT_NEW)),
						main_window.clone(),
					);
					self.after_save = Some(Box::new(|ctx: &mut DelegateCtx| {
						ctx.submit_command(IMPL_PROJECT_NEW, None);
					}));
				}
				false
			}
			_ if cmd.is(cmds::PROJECT_OPEN) => {
				if data.up_to_date {
					ctx.submit_command(IMPL_PROJECT_OPEN, None)
				} else {
					ctx.submit_command(
						Command::new(widget::overlay::SHOW_MIDDLE, ui::modal::save::build(IMPL_PROJECT_OPEN)),
						main_window.clone(),
					);
					self.after_save = Some(Box::new(|ctx: &mut DelegateCtx| {
						ctx.submit_command(IMPL_PROJECT_OPEN, None);
					}));
				}
				false
			}
			_ if cmd.is(cmds::PROJECT_SAVE_AS) => {
				ctx.submit_command(
					Command::new(
						sys_cmds::SHOW_SAVE_PANEL,
						FileDialogOptions::new().allowed_types(vec![FileSpec::new("Harmoxen project", &["hxp"])]),
					),
					Target::Window(*data.main_window.clone().unwrap()),
				);
				false
			}
			_ if cmd.is(cmds::PROJECT_SAVE) => {
				if let Some(path) = data.save_path.clone() {
					let project = state::Project::from_editors(&data.editors);
					let data = ron::to_string(&project).unwrap();
					fs::write(&*path, data).ok();
					if let Some(after_save) = self.after_save.take() {
						after_save(ctx);
					}
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
			_ if cmd.is(IMPL_PROJECT_NEW) => {
				let mut state = State::new();
				state.main_window = data.main_window.clone();
				*data = state;
				project_changed = true;
				self.after_save = None;
				false
			}
			_ if cmd.is(IMPL_PROJECT_OPEN) => {
				ctx.submit_command(
					Command::new(
						sys_cmds::SHOW_OPEN_PANEL,
						FileDialogOptions::new().allowed_types(vec![FileSpec::new("Harmoxen project", &["hxp"])]),
					),
					*data.main_window.clone().unwrap(),
				);
				self.after_save = None;
				false
			}
			_ if cmd.is(sys_cmds::SAVE_FILE) => {
				if let Some(file_info) = cmd.get_unchecked(sys_cmds::SAVE_FILE) {
					data.up_to_date = true;
					data.save_path = Some(Rc::new(file_info.path().into()));
					let project = state::Project::from_editors(&data.editors);
					let project_str = ron::to_string(&project).unwrap();
					fs::write(file_info.path(), project_str).ok();

					if let Some(after_save) = self.after_save.take() {
						after_save(ctx);
					}
				}
				true
			}
			_ if cmd.is(sys_cmds::OPEN_FILE) => {
				let file_info = cmd.get_unchecked(sys_cmds::OPEN_FILE);
				if let Ok(project_string) = fs::read_to_string(file_info.path()) {
					if let Ok(project) = ron::from_str::<state::Project>(&project_string) {
						project.open(&mut data.editors);
						data.up_to_date = true;
						data.save_path = Some(Rc::new(file_info.path().into()));
						project_changed = true;
					}
				}
				true
			}
			_ => true,
		};
		if project_changed {
			ctx.submit_command(cmds::REDRAW, Target::Global);
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
