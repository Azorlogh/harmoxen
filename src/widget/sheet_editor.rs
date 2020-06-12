use druid::kurbo::Line;
use druid::{
	BoxConstraints, Color, Command, ContextMenu, Data, Env, Event, EventCtx, KeyCode, KeyEvent, LayoutCtx, LifeCycle,
	LifeCycleCtx, LocalizedString, MenuDesc, MenuItem, PaintCtx, Point, Rect, RenderContext, Selector, Size, UpdateCtx, Widget,
	WidgetExt, WidgetPod,
};

use generational_arena::Index;
use std::time::Instant;

use crate::commands;
use crate::data::{
	icp,
	sheet::{Interval, Note, Pitch},
};
use crate::state::editors::sheet_editor::State;
use crate::theme;
use crate::util::coord::Coord;
use crate::widget::common::{ParseLazy, TextBox};

mod layout;
mod notes;

pub const REDRAW: Selector = Selector::new("sheet-editor.redraw");
pub const ADD_RELATIVE_NOTE: Selector<(Index, f64)> = Selector::new("sheet-editor.add-relative-note");
pub const DUPLICATE_NOTE: Selector<(Index, f64)> = Selector::new("sheet-editor.duplicate-note");
pub const DELETE_NOTE: Selector<Index> = Selector::new("sheet-editor.delete-note");

#[derive(PartialEq)]
pub enum EditAction {
	Idle,
	Moving(Index, f64),
	Scaling(Index),
}
use EditAction::*;

impl EditAction {
	pub fn note_id(&self) -> Option<Index> {
		match self {
			Idle => None,
			Moving(id, _) => Some(*id),
			Scaling(id) => Some(*id),
		}
	}
}

pub struct SheetEditor {
	hover: EditAction,
	action: EditAction,
	note_len: f64,
	last_left_click: (Point, Instant), // until druid supports multi-clicks
	prev_mouse_b_pos: Option<Point>,   // previous mouse position in board coordinates
	interval_input: Option<(Index, WidgetPod<State, Box<dyn Widget<State>>>)>,
	action_effective: bool, // true if the current action state has changed the sheet
}

impl SheetEditor {
	pub fn new() -> SheetEditor {
		SheetEditor {
			hover: Idle,
			action: Idle,
			note_len: 1.0,
			last_left_click: ((f64::INFINITY, f64::INFINITY).into(), Instant::now()),
			prev_mouse_b_pos: None,
			interval_input: None,
			action_effective: false,
		}
	}
}

fn get_action(pos: Point, coord: Coord, data: &State, env: &Env) -> EditAction {
	let hovered_note_id = data
		.sheet
		.borrow()
		.get_note_at(pos, coord.to_board_h(env.get(theme::NOTE_HEIGHT)));
	let action = match hovered_note_id {
		None => Idle,
		Some(id) => {
			let note = data.sheet.borrow().get_note(id).unwrap();
			if pos.x > note.end() - coord.to_board_w(env.get(theme::NOTE_SCALE_KNOB)) && pos.x > note.start + note.length * 0.60
			{
				Scaling(id)
			} else {
				Moving(id, pos.x - note.start)
			}
		}
	};
	action
}

impl Widget<State> for SheetEditor {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, env: &Env) {
		if let Some(interval_input) = &mut self.interval_input {
			if let Event::KeyDown(KeyEvent {
				key_code: KeyCode::Return,
				..
			}) = event
			{
				let mut sheet = data.sheet.borrow_mut();
				let note = sheet.get_note_mut(interval_input.0).unwrap();
				if let Pitch::Relative(_, ref mut interval) = note.pitch {
					*interval = data.interval_input;
				}
				ctx.request_layout();
				ctx.request_paint();
				ctx.set_handled();
			} else {
				interval_input.1.event(ctx, event, data, env);
			}
			if ctx.is_handled() {
				return;
			}
		}
		let mut sheet_changed = false;
		let mut history_save = false;
		let size = ctx.size();

		let coord = Coord::new(data.frame.clone(), size);

		match event {
			Event::MouseDown(mouse) if mouse.button.is_left() => {
				ctx.request_focus();
				let is_double_click = mouse.pos == self.last_left_click.0 && self.last_left_click.1.elapsed().as_millis() < 500;
				self.last_left_click = (mouse.pos, Instant::now()); // remove this once druid has multi-clicks
				let pos = coord.to_board_p(mouse.pos);
				if is_double_click {
					if let Some(id) = get_action(pos, coord, data, env).note_id() {
						let menu = ContextMenu::new(make_note_context_menu::<crate::state::State>(id, pos.x), mouse.window_pos);
						ctx.show_context_menu(menu);
					}
				} else {
					let mut action = get_action(pos, coord, data, env);
					if action == Idle {
						let note = data.layout.borrow().quantize_note(Note::new(pos, self.note_len));
						let mut sheet = data.sheet.borrow_mut();
						let id = sheet.add_note(note);
						action = Moving(id, 0.0);
						history_save = true;
						sheet_changed = true;
					}
					self.action = action;
					if let Some(id) = self.action.note_id() {
						let note = data.sheet.borrow().get_note(id).unwrap();
						self.note_len = note.length;
					}
					if let Moving(id, _) = self.action {
						let sheet = data.sheet.borrow();
						let note = sheet.get_note(id).unwrap();
						let note_freq = sheet.get_freq(note.pitch);
						let cmd = Command::new(
							commands::ICP,
							icp::Event::NotePlay(icp::Note {
								id: 2000,
								freq: note_freq,
							}),
						);
						ctx.submit_command(cmd, ctx.window_id());
						if let Pitch::Relative(_, interval) = note.pitch {
							let widget = WidgetPod::new(
								ParseLazy::new(TextBox::new())
									.lens(State::interval_input)
									.background(Color::rgb8(255, 0, 0)),
							)
							.boxed();
							data.interval_input = interval;
							self.interval_input = Some((id, widget));
							ctx.children_changed();
							ctx.request_layout();
						}
					}
					ctx.set_active(true);
				}
			}
			Event::MouseMove(mouse) if !mouse.buttons.has_right() => {
				let pos = coord.to_board_p(mouse.pos);
				if mouse.buttons.has_left() {
					ctx.set_handled();
					match self.action {
						Scaling(id) => {
							let time = data.layout.borrow().quantize_time(pos.x, false);
							let note = data.sheet.borrow().get_note(id).unwrap();
							if time > note.start && time != note.end() {
								data.sheet.borrow_mut().resize_note_to(id, time);
								self.action_effective = true;
								sheet_changed = true;
								self.note_len = time - note.start;
							}
						}
						Moving(id, anchor) => {
							let (start, freq) = data
								.layout
								.borrow()
								.quantize_position((pos.x - anchor).max(0.0), 2f64.powf(pos.y));
							let mut sheet = data.sheet.borrow_mut();
							let note = sheet.get_note(id).unwrap();
							if note.start != start || sheet.get_freq(note.pitch) != freq {
								sheet.move_note(id, start, freq);
								sheet_changed = true;
								self.action_effective = true;
								let note = sheet.get_note(id).unwrap();
								let cmd =
									Command::new(commands::ICP, icp::Event::NoteChangeFreq(2000, sheet.get_freq(note.pitch)));
								ctx.submit_command(cmd, ctx.window_id());
								if let Pitch::Relative(_, _) = note.pitch {
									ctx.request_layout();
								}
							}
						}
						_ => {}
					}
				}
				let hover = get_action(pos, coord, data, env);
				if self.hover != hover {
					ctx.request_paint();
				}
				self.hover = hover;
			}
			Event::MouseDown(mouse) if mouse.button.is_right() => {
				let point = coord.to_board_p(mouse.pos);
				self.interval_input = None;
				let mut sheet = data.sheet.borrow_mut();
				if let Some(id) = sheet.get_note_at(point, coord.to_board_h(env.get(theme::NOTE_HEIGHT))) {
					sheet.remove_note(id);
					self.action_effective = true;
					sheet_changed = true;
				} else {
					self.prev_mouse_b_pos = Some(point);
				}
				ctx.request_focus();
			}
			Event::MouseMove(mouse) if mouse.buttons.has_right() => {
				let point = coord.to_board_p(mouse.pos);
				if let Some(prev_point) = self.prev_mouse_b_pos {
					let mut sheet = data.sheet.borrow_mut();
					let notes_len_before = sheet.notes.len();
					sheet.remove_notes_along(Line::new(prev_point, point), coord.to_board_h(env.get(theme::NOTE_HEIGHT)));
					if notes_len_before != sheet.notes.len() {
						self.action_effective = true;
						sheet_changed = true;
					}
					self.prev_mouse_b_pos = Some(point);
				}
			}
			Event::MouseUp(mouse) if mouse.button.is_left() => {
				if self.action_effective {
					history_save = true;
					self.action_effective = false;
				}
				self.action = Idle;
				ctx.set_active(false);
				self.prev_mouse_b_pos = None;
				ctx.request_paint();
				let cmd = Command::new(commands::ICP, icp::Event::NoteStop(2000));
				ctx.submit_command(cmd, ctx.window_id());
			}
			Event::KeyDown(key) if key.key_code == KeyCode::Space => {
				let command = if !data.playing {
					commands::PLAY_START
				} else {
					commands::PLAY_STOP
				};
				ctx.submit_command(command, ctx.window_id());
			}
			Event::KeyDown(key) if key.mods.ctrl && key.key_code == KeyCode::KeyZ => {
				ctx.submit_command(commands::HISTORY_UNDO, ctx.window_id());
			}
			Event::KeyDown(key) if key.mods.ctrl && key.key_code == KeyCode::KeyY => {
				ctx.submit_command(commands::HISTORY_REDO, ctx.window_id());
			}
			Event::KeyDown(key) if key.mods.ctrl && key.key_code == KeyCode::KeyN => {
				ctx.submit_command(commands::PROJECT_NEW, ctx.window_id());
			}
			Event::KeyDown(key) if key.mods.ctrl && key.key_code == KeyCode::KeyO => {
				ctx.submit_command(commands::PROJECT_OPEN, ctx.window_id());
			}
			Event::KeyDown(key) if key.mods.ctrl && key.mods.shift && key.key_code == KeyCode::KeyS => {
				ctx.submit_command(commands::PROJECT_SAVE_AS, ctx.window_id());
			}
			Event::KeyDown(key) if key.mods.ctrl && key.key_code == KeyCode::KeyS => {
				ctx.submit_command(commands::PROJECT_SAVE, ctx.window_id());
			}
			Event::WindowSize(_) => {
				ctx.request_layout();
				ctx.request_paint();
			}
			Event::Command(cmd) if cmd.is(commands::REDRAW) || cmd.is(REDRAW) => {
				ctx.request_layout();
				ctx.request_paint();
				if let Some(interval_input) = &mut self.interval_input {
					let mut sheet = data.sheet.borrow_mut();
					if sheet.get_note_mut(interval_input.0).is_none() {
						self.interval_input = None;
					}
				}
				let bounds = data.sheet.borrow().get_bounds();
				data.frame.x.bounds.1 = ((bounds.0).1 * 1.25).max(5.0);
			}
			Event::Command(ref cmd) if cmd.is(ADD_RELATIVE_NOTE) => {
				let (root, time) = *cmd.get_unchecked(ADD_RELATIVE_NOTE);
				let note = data.layout.borrow().quantize_note(Note {
					start: time,
					length: self.note_len,
					pitch: Pitch::Relative(root, Interval::Ratio(3, 2)),
				});
				let mut sheet = data.sheet.borrow_mut();
				sheet.add_note(note);
				sheet_changed = true;
			}
			Event::Command(ref cmd) if cmd.is(DUPLICATE_NOTE) => {
				let mut sheet = data.sheet.borrow_mut();
				let (original, time) = *cmd.get_unchecked(DUPLICATE_NOTE);
				if let Some(original) = sheet.get_note(original) {
					let note = data.layout.borrow().quantize_note(Note {
						start: time,
						length: original.length,
						pitch: original.pitch,
					});
					sheet.add_note(note);
					sheet_changed = true;
				}
			}
			Event::Command(ref cmd) if cmd.is(DELETE_NOTE) => {
				let id = *cmd.get_unchecked(DELETE_NOTE);
				data.sheet.borrow_mut().remove_note(id);
				sheet_changed = true;
			}
			_ => {}
		}
		if sheet_changed {
			ctx.submit_command(commands::SHEET_CHANGED, ctx.window_id());
		}
		if history_save {
			ctx.submit_command(commands::HISTORY_SAVE, ctx.window_id());
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &State, env: &Env) {
		match event {
			LifeCycle::WidgetAdded => {
				ctx.register_for_focus();
			}
			_ => {}
		}
		if let Some(widget) = &mut self.interval_input {
			widget.1.lifecycle(ctx, event, data, env);
		}
	}

	fn update(&mut self, ctx: &mut UpdateCtx, old_data: &State, data: &State, env: &Env) {
		if old_data.frame != data.frame || old_data.cursor != data.cursor {
			ctx.request_layout();
			ctx.request_paint();
		}
		if let Some(widget) = &mut self.interval_input {
			widget.1.update(ctx, data, env);
		}
	}

	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &State, env: &Env) -> Size {
		let xrange = data.frame.x.view;
		let yrange = data.frame.y.view;
		let Size { width, height } = bc.max();
		let to_screen = |p: Point| {
			Point::new(
				((p.x - xrange.0) / xrange.size()) * width,
				height - ((p.y.log2() - yrange.0) / yrange.size()) * height,
			)
		};
		if let Some((id, widget)) = &mut self.interval_input {
			let sheet = data.sheet.borrow();
			let note = sheet.get_note(*id).unwrap();
			if let Pitch::Relative(root, _) = note.pitch {
				let root = sheet.get_note(root).unwrap();
				let position = Point::new(note.start, (sheet.get_freq(note.pitch) + sheet.get_freq(root.pitch)) / 2.0);
				let screen_pos = to_screen(position);
				let size = widget.layout(ctx, bc, data, env);
				let layout_rect = Rect::from_origin_size(screen_pos, size);
				widget.set_layout_rect(ctx, data, env, layout_rect);
			}
		}
		bc.max()
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &State, env: &Env) {
		let size = ctx.size();
		let rect = Rect::from_origin_size(Point::ORIGIN, size);
		ctx.clip(rect);
		ctx.fill(rect, &env.get(theme::BACKGROUND_0));

		let coord = Coord::new(data.frame.clone(), size);

		// LAYOUT
		let layout = data.layout.borrow();
		self.draw_layout(ctx, &coord, &layout, env);

		// NOTES
		let sheet = &data.sheet.borrow();
		self.draw_notes(ctx, &coord, &sheet, env);

		// CURSOR
		let cursor = coord.to_screen_x(data.cursor);
		let line = Line::new(Point::new(cursor, 0.0), Point::new(cursor, size.height));
		ctx.stroke(line, &Color::WHITE, 1.0);

		// INTERVAL INPUT
		if let Some(widget) = &mut self.interval_input {
			widget.1.paint(ctx, data, env);
		}
	}
}

fn make_note_context_menu<T: Data>(id: Index, time: f64) -> MenuDesc<T> {
	MenuDesc::empty()
		.append(MenuItem::new(
			LocalizedString::new("Add relative note"),
			Command::new(ADD_RELATIVE_NOTE, (id, time)),
		))
		.append(MenuItem::new(
			LocalizedString::new("Duplicate note"),
			Command::new(DUPLICATE_NOTE, (id, time)),
		))
		.append(MenuItem::new(
			LocalizedString::new("Delete note"),
			Command::new(DELETE_NOTE, id),
		))
}
