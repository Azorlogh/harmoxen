use crate::commands;
use crate::state::editors::sheet_editor::State;
use druid::{
	BoxConstraints, Code, Env, Event, EventCtx, KbKey, KeyEvent, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx,
	Widget,
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
			Event::KeyDown(evt) if evt.code == Code::Space => {
				let command = if data.playing {
					commands::PLAY_STOP
				} else {
					commands::PLAY_START
				};
				ctx.submit_command(command.to(ctx.window_id()));
			}
			Event::KeyDown(e) => match e {
				KeyEvent {
					key: KbKey::Character(c),
					..
				} => match c.as_str() {
					"z" if e.mods.ctrl() => ctx.submit_command(commands::HISTORY_UNDO.to(ctx.window_id())),
					"y" if e.mods.ctrl() => ctx.submit_command(commands::HISTORY_REDO.to(ctx.window_id())),
					"n" if e.mods.ctrl() => ctx.submit_command(commands::PROJECT_NEW.to(ctx.window_id())),
					"o" if e.mods.ctrl() => ctx.submit_command(commands::PROJECT_OPEN.to(ctx.window_id())),
					"s" if e.mods.ctrl() && e.mods.shift() => ctx.submit_command(commands::PROJECT_SAVE_AS.to(ctx.window_id())),
					"s" if e.mods.ctrl() => ctx.submit_command(commands::PROJECT_SAVE.to(ctx.window_id())),
					"x" if e.mods.ctrl() => ctx.submit_command(selection::CUT.to(ctx.window_id())),
					"c" if e.mods.ctrl() => ctx.submit_command(selection::COPY.to(ctx.window_id())),
					"v" if e.mods.ctrl() => ctx.submit_command(selection::PASTE.to(ctx.window_id())),
					"a" if e.mods.ctrl() => ctx.submit_command(selection::SELECT_ALL.to(ctx.window_id())),
					_ => {}
				},
				e if e.code == Code::Delete => {
					ctx.submit_command(selection::DELETE.to(ctx.window_id()));
				}
				_ => {}
			},
			_ => {}
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &State, _env: &Env) {
		if let LifeCycle::WidgetAdded = event {
		    ctx.register_for_focus();
		}
	}

	fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {}

	fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &State, _env: &Env) -> Size {
		bc.max()
	}

	fn paint(&mut self, _ctx: &mut PaintCtx, _data: &State, _env: &Env) {}
}
