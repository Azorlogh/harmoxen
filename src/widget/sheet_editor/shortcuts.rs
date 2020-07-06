use crate::commands;
use crate::state::editors::sheet_editor::State;
use druid::{
	BoxConstraints, Env, Event, EventCtx, KeyCode, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget,
};

use super::selection;

pub struct Shortcuts;

impl Shortcuts {
	pub fn new() -> Shortcuts {
		Shortcuts
	}
}

impl Widget<State> for Shortcuts {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, _env: &Env) {
		match event {
			Event::MouseDown(_) => {
				ctx.request_focus();
			}
			Event::KeyDown(key) if key.key_code == KeyCode::Space => {
				let command = if !data.playing {
					commands::PLAY_START
				} else {
					commands::PLAY_STOP
				};
				ctx.submit_command(command, ctx.window_id());
			}
			Event::KeyDown(key) => match key.key_code {
				KeyCode::KeyZ if key.mods.ctrl => {
					ctx.submit_command(commands::HISTORY_UNDO, ctx.window_id());
				}
				KeyCode::KeyY if key.mods.ctrl => {
					ctx.submit_command(commands::HISTORY_REDO, ctx.window_id());
				}
				KeyCode::KeyN if key.mods.ctrl => {
					ctx.submit_command(commands::PROJECT_NEW, ctx.window_id());
				}
				KeyCode::KeyO if key.mods.ctrl => {
					ctx.submit_command(commands::PROJECT_OPEN, ctx.window_id());
				}
				KeyCode::KeyS if key.mods.ctrl && key.mods.shift => {
					ctx.submit_command(commands::PROJECT_SAVE_AS, ctx.window_id());
				}
				KeyCode::KeyS if key.mods.ctrl => {
					ctx.submit_command(commands::PROJECT_SAVE, ctx.window_id());
				}
				KeyCode::KeyX if key.mods.ctrl => {
					ctx.submit_command(selection::CUT, ctx.window_id());
				}
				KeyCode::KeyC if key.mods.ctrl => {
					ctx.submit_command(selection::COPY, ctx.window_id());
				}
				KeyCode::KeyV if key.mods.ctrl => {
					ctx.submit_command(selection::PASTE, ctx.window_id());
				}
				KeyCode::Delete => {
					ctx.submit_command(selection::DELETE, ctx.window_id());
				}
				KeyCode::KeyA if key.mods.ctrl => {
					ctx.submit_command(selection::SELECT_ALL, ctx.window_id());
				}
				_ => {}
			},
			_ => {}
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &State, _env: &Env) {
		match event {
			LifeCycle::WidgetAdded => {
				ctx.register_for_focus();
			}
			_ => {}
		}
	}

	fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {}

	fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &State, _env: &Env) -> Size {
		bc.max()
	}

	fn paint(&mut self, _ctx: &mut PaintCtx, _data: &State, _env: &Env) {}
}
