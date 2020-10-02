use crate::commands;
use crate::state::editors::sheet_editor::State;
use crate::theme;
use crate::util::coord::Coord;
use druid::{
	BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, RenderContext, Selector,
	Size, UpdateCtx, Widget,
};
use std::collections::HashSet;

pub const CUT: Selector = Selector::new("selection.cut");
pub const COPY: Selector = Selector::new("selection.copy");
pub const PASTE: Selector = Selector::new("selection.paste");
pub const DELETE: Selector = Selector::new("selection.delete");
pub const SELECT_ALL: Selector = Selector::new("selection.select-all");

#[derive(Debug, PartialEq)]
pub enum Action {
	Idle,
	SelectionAdd(Point, Point),
	SelectionRemove(Point, Point),
}

pub struct Selection {
	action: Action,
	action_effective: bool, // true if the current action state has changed the sheet
}

impl Selection {
	pub fn new() -> Selection {
		Selection {
			action: Action::Idle,
			action_effective: false,
		}
	}
}

impl Widget<State> for Selection {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, env: &Env) {
		// handle events
		let size = ctx.size();
		let coord = Coord::new(data.frame.clone(), size);
		let mut history_save = false;
		match event {
			Event::MouseDown(mouse) => {
				let sheet = data.sheet.borrow();
				let pos = coord.to_board_p(mouse.pos);
				if mouse.mods.ctrl() {
					if mouse.button.is_left() {
						self.action = Action::SelectionAdd(pos, pos);
						ctx.set_active(true);
						ctx.set_handled();
					} else if mouse.button.is_right() {
						self.action = Action::SelectionRemove(pos, pos);
						ctx.set_active(true);
						ctx.set_handled();
					}
				} else {
					let note = sheet.get_note_at(pos, coord.to_board_h(env.get(theme::NOTE_HEIGHT)));
					if !note.map(|idx| data.selection.borrow().contains(&idx)).unwrap_or(false) {
						data.selection.borrow_mut().clear();
						ctx.request_paint();
					}
				}
			}
			Event::MouseMove(mouse) => {
				let pos = coord.to_board_p(mouse.pos);
				if ctx.is_active() {
					ctx.set_handled();
					match self.action {
						Action::SelectionAdd(_, ref mut to) => {
							*to = pos;
							ctx.request_paint();
						}
						Action::SelectionRemove(_, ref mut to) => {
							*to = pos;
							ctx.request_paint();
						}
						_ => {}
					}
				}
			}
			Event::MouseUp(_) => {
				let sheet = data.sheet.borrow();
				match self.action {
					Action::SelectionAdd(p0, p1) => {
						let notes =
							sheet.get_notes_rect(Rect::from_points(p0, p1), coord.to_board_h(env.get(theme::NOTE_HEIGHT)));
						if !notes.is_empty() {
							let mut selection = data.selection.borrow_mut();
							selection.extend(notes);
							self.action_effective = true;
						}
					}
					Action::SelectionRemove(p0, p1) => {
						let notes =
							sheet.get_notes_rect(Rect::from_points(p0, p1), coord.to_board_h(env.get(theme::NOTE_HEIGHT)));
						if !notes.is_empty() {
							let mut selection = data.selection.borrow_mut();
							*selection = &*selection - &notes.into_iter().collect::<HashSet<_>>();
							self.action_effective = true;
						}
					}
					_ => {}
				}
				if self.action_effective {
					history_save = true;
					self.action_effective = false;
					ctx.submit_command(super::REDRAW.to(ctx.window_id()));
				}
				self.action = Action::Idle;
				ctx.set_active(false);
			}
			Event::Command(cmd) if cmd.is(CUT) => {
				let mut selection = data.selection.borrow_mut();
				let mut sheet = data.sheet.borrow_mut();
				let mut clipboard = data.clipboard.borrow_mut();
				clipboard.cut(&mut sheet, &mut selection);
				history_save = true;
				ctx.submit_command(commands::SHEET_CHANGED.to(ctx.window_id()));
			}
			Event::Command(cmd) if cmd.is(COPY) => {
				let sheet = data.sheet.borrow();
				let selection = data.selection.borrow();
				let mut clipboard = data.clipboard.borrow_mut();
				clipboard.copy(&sheet, &selection);
			}
			Event::Command(cmd) if cmd.is(PASTE) => {
				let mut sheet = data.sheet.borrow_mut();
				let mut selection = data.selection.borrow_mut();
				let clipboard = data.clipboard.borrow();
				clipboard.paste(&mut sheet, &mut selection);
				history_save = true;
				ctx.submit_command(commands::SHEET_CHANGED.to(ctx.window_id()));
			}
			Event::Command(cmd) if cmd.is(DELETE) => {
				let mut sheet = data.sheet.borrow_mut();
				let mut selection = data.selection.borrow_mut();
				for idx in selection.drain() {
					sheet.remove_note(idx);
				}
				history_save = true;
				ctx.submit_command(commands::SHEET_CHANGED.to(ctx.window_id()));
			}
			Event::Command(cmd) if cmd.is(SELECT_ALL) => {
				let sheet = data.sheet.borrow();
				let mut selection = data.selection.borrow_mut();
				*selection = sheet.indices.iter().copied().collect();
				history_save = true;
				ctx.submit_command(super::REDRAW.to(ctx.window_id()));
			}
			_ => {}
		}
		if history_save {
			ctx.submit_command(commands::HISTORY_SAVE.to(ctx.window_id()));
		}
	}

	fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &State, _env: &Env) {}

	fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {}

	fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &State, _env: &Env) -> Size {
		bc.max()
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &State, env: &Env) {
		let size = ctx.size();
		let rect = Rect::from_origin_size(Point::ORIGIN, size);
		ctx.clip(rect);

		let coord = Coord::new(data.frame.clone(), size);

		// SELECTION EDITING
		if let Action::SelectionAdd(p0, p1) = self.action {
			ctx.fill(
				Rect::from_points(coord.to_screen_p(p0), coord.to_screen_p(p1)),
				&env.get(theme::SELECTION_ADD_INSIDE),
			);
			ctx.stroke(
				Rect::from_points(coord.to_screen_p(p0), coord.to_screen_p(p1)),
				&env.get(theme::SELECTION_ADD_BORDER),
				1.0,
			);
		}
		if let Action::SelectionRemove(p0, p1) = self.action {
			ctx.fill(
				Rect::from_points(coord.to_screen_p(p0), coord.to_screen_p(p1)),
				&env.get(theme::SELECTION_REMOVE_INSIDE),
			);
			ctx.stroke(
				Rect::from_points(coord.to_screen_p(p0), coord.to_screen_p(p1)),
				&env.get(theme::SELECTION_REMOVE_BORDER),
				1.0,
			);
		}
	}
}
