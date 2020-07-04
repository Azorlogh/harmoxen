use crate::commands;
use crate::data::{
	icp,
	sheet::{Interval, Note, Pitch, Sheet},
};
use crate::state::editors::sheet_editor::State;
use crate::theme;
use crate::util::coord::Coord;
use crate::widget::common::{ParseLazy, TextBox};
use druid::kurbo::Line;
use druid::{
	BoxConstraints, Color, Command, ContextMenu, Data, Env, Event, EventCtx, KeyCode, KeyEvent, LayoutCtx, LifeCycle,
	LifeCycleCtx, LocalizedString, MenuDesc, MenuItem, PaintCtx, Point, Rect, RenderContext, Selector, Size, UpdateCtx, Vec2,
	Widget, WidgetExt, WidgetPod,
};
use generational_arena::Index;
use std::{
	collections::{HashMap, HashSet},
	time::Instant,
};

mod layout;
mod notes;

pub const REDRAW: Selector = Selector::new("sheet-editor.redraw");
pub const ADD_RELATIVE_NOTE: Selector<(Index, f64)> = Selector::new("sheet-editor.add-relative-note");
pub const DUPLICATE_NOTE: Selector<(Index, f64)> = Selector::new("sheet-editor.duplicate-note");
pub const DELETE_NOTE: Selector<Index> = Selector::new("sheet-editor.delete-note");

#[derive(Debug, PartialEq)]
pub enum Hover {
	Idle,
	Move(Index),
	Scale(Index),
}
impl Hover {
	pub fn note_idx(&self) -> Option<Index> {
		match self {
			Hover::Move(id) => Some(*id),
			Hover::Scale(id) => Some(*id),
			_ => None,
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum Action {
	Idle,
	Move(Index, HashMap<Index, Vec2>, Rect), // root note, offsets to mouse, extent of selection around mouse
	Scale(Index, HashMap<Index, f64>),       // root note, original lengths of notes
	DeleteNotes(Point),
	SelectionAdd(Point, Point),
	SelectionRemove(Point, Point),
}

pub struct SheetEditor {
	hover: Hover,
	action: Action,
	note_len: f64,
	last_left_click: (Point, Instant), // until druid supports multi-clicks
	interval_input: Option<(Index, WidgetPod<State, Box<dyn Widget<State>>>)>,
	action_effective: bool, // true if the current action state has changed the sheet
}

impl SheetEditor {
	pub fn new() -> SheetEditor {
		SheetEditor {
			hover: Hover::Idle,
			action: Action::Idle,
			note_len: 1.0,
			last_left_click: ((f64::INFINITY, f64::INFINITY).into(), Instant::now()),
			interval_input: None,
			action_effective: false,
		}
	}
}

fn get_hover(pos: Point, coord: Coord, sheet: &Sheet, env: &Env) -> Hover {
	let hovered_note_idx = sheet.get_note_at(pos, coord.to_board_h(env.get(theme::NOTE_HEIGHT)));
	match hovered_note_idx {
		None => Hover::Idle,
		Some(idx) => {
			let note = sheet.get_note(idx).unwrap();
			if pos.x > note.end() - coord.to_board_w(env.get(theme::NOTE_SCALE_KNOB)) && pos.x > note.start + note.length * 0.60
			{
				Hover::Scale(idx)
			} else {
				Hover::Move(idx)
			}
		}
	}
}

impl Widget<State> for SheetEditor {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, env: &Env) {
		// send events to the interval input widget
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

		// handle events
		let mut sheet = data.sheet.borrow_mut();
		let layout = data.layout.borrow();
		let mut sheet_changed = false;
		let mut history_save = false;
		let size = ctx.size();
		let coord = Coord::new(data.frame.clone(), size);
		match event {
			Event::MouseDown(mouse) => {
				ctx.request_focus();
				let pos = coord.to_board_p(mouse.pos);

				// Selection actions
				if mouse.mods.ctrl {
					if mouse.button.is_left() {
						self.action = Action::SelectionAdd(pos, pos);
						ctx.set_active(true);
					} else if mouse.button.is_right() {
						self.action = Action::SelectionRemove(pos, pos);
						ctx.set_active(true);
					}

				// General actions
				} else {
					if let Some(idx) = self.hover.note_idx() {
						if !data.selection.borrow().contains(&idx) {
							data.selection.borrow_mut().clear();
						}
					} else {
						data.selection.borrow_mut().clear();
						ctx.request_paint();
					}
					ctx.set_active(true);
					if mouse.button.is_left() {
						let is_double_click =
							mouse.pos == self.last_left_click.0 && self.last_left_click.1.elapsed().as_millis() < 500;
						self.last_left_click = (mouse.pos, Instant::now()); // remove this once druid has multi-clicks
						if is_double_click {
							if let Some(id) = get_hover(pos, coord, &sheet, env).note_idx() {
								let menu = ContextMenu::new(
									make_note_context_menu::<crate::state::State>(id, pos.x),
									mouse.window_pos,
								);
								ctx.show_context_menu(menu);
							}
						} else {
							match self.hover {
								Hover::Idle => {
									let note = layout.quantize_note(Note::new(pos, self.note_len));
									if sheet.get_note_at(Point::new(note.start, note.y(&sheet)), 0.01).is_none() {
										let idx = sheet.add_note(note);
										ctx.submit_command(
											Command::new(
												commands::ICP,
												icp::Event::NotePlay(icp::Note {
													id: 2000,
													freq: sheet.get_freq(note.pitch),
												}),
											),
											ctx.window_id(),
										);
										let mut notes = HashMap::new();
										notes.insert(idx, Vec2::ZERO);
										let note_y = sheet.get_y(note.pitch);
										self.action =
											Action::Move(idx, notes, Rect::new(note.start, note_y, note.end(), note_y));
										history_save = true;
										sheet_changed = true;
									}
								}
								Hover::Move(idx) => {
									let selection = data.selection.borrow();
									if selection.len() > 0 {
										let mut notes = HashMap::new();
										let root = sheet.get_note(idx).unwrap();
										let mut rect = root.rect(&sheet, 0.0);
										for idx in data.selection.borrow().iter() {
											let note = sheet.get_note(*idx).expect("selection contained a dead note");
											let offset = Vec2::new(note.start, sheet.get_y(note.pitch)) - pos.to_vec2();
											rect = rect.union(note.rect(&sheet, 0.0));
											notes.insert(*idx, offset);
										}
										rect = rect + -pos.to_vec2();
										self.action = Action::Move(idx, notes, rect);
									} else {
										let note = sheet.get_note(idx).unwrap();
										let mut notes = HashMap::new();
										notes.insert(idx, Vec2::new(note.start, sheet.get_y(note.pitch)) - pos.to_vec2());
										self.action = Action::Move(idx, notes, note.rect(&sheet, 0.0));
										let note = sheet.get_note(idx).unwrap();
										self.note_len = note.length;
										let note_freq = sheet.get_freq(note.pitch);
										ctx.submit_command(
											Command::new(
												commands::ICP,
												icp::Event::NotePlay(icp::Note {
													id: 2000,
													freq: note_freq,
												}),
											),
											ctx.window_id(),
										);
										if let Pitch::Relative(_, interval) = note.pitch {
											let widget = WidgetPod::new(
												ParseLazy::new(TextBox::new())
													.lens(State::interval_input)
													.background(Color::rgb8(255, 0, 0)),
											)
											.boxed();
											data.interval_input = interval;
											self.interval_input = Some((idx, widget));
											ctx.children_changed();
											ctx.request_layout();
										}
									}
								}
								Hover::Scale(idx) => {
									let selection = data.selection.borrow();
									if selection.len() > 0 {
										let mut notes = HashMap::new();
										for &idx in selection.iter() {
											notes.insert(
												idx,
												sheet.get_note(idx).expect("selection contained a dead note").length,
											);
										}
										self.action = Action::Scale(idx, notes);
									} else {
										let note = sheet.get_note(idx).unwrap();
										self.note_len = note.length;
										self.action = Action::Scale(idx, [(idx, note.length)].iter().cloned().collect());
									}
								}
							}
						}
					} else if mouse.button.is_right() {
						self.interval_input = None;
						if let Some(id) = sheet.get_note_at(pos, coord.to_board_h(env.get(theme::NOTE_HEIGHT))) {
							sheet.remove_note(id);
							self.action_effective = true;
							sheet_changed = true;
						} else {
							self.action = Action::DeleteNotes(pos);
						}
					}
				}
			}
			Event::MouseMove(mouse) => {
				let pos = coord.to_board_p(mouse.pos);
				if ctx.is_active() {
					ctx.set_handled();
					match &mut self.action {
						Action::Move(root_idx, offsets, bounds) => {
							let root_offset = offsets[&root_idx];
							let mut anchor = layout.quantize_position(pos + root_offset) - root_offset;
							anchor.x = anchor.x.max(-bounds.min_x());
							for (idx, offset) in offsets {
								let note = sheet.get_note(*idx).unwrap();
								let pos = anchor + *offset;
								if note.start != pos.x || note.y(&sheet) != pos.y {
									sheet.move_note(*idx, pos.x, pos.y);
									sheet_changed = true;
									self.action_effective = true;
									if sheet.get_y(note.pitch) != pos.y {
										let note = sheet.get_note(*idx).unwrap();
										ctx.submit_command(
											Command::new(commands::ICP, icp::Event::NoteStop(2000)),
											ctx.window_id(),
										);
										ctx.submit_command(
											Command::new(
												commands::ICP,
												icp::Event::NotePlay(icp::Note {
													id: 2000,
													freq: sheet.get_freq(note.pitch),
												}),
											),
											ctx.window_id(),
										);
									}
									if let Pitch::Relative(_, _) = note.pitch {
										ctx.request_layout();
									}
								}
							}
						}
						Action::Scale(idx, lengths) => {
							let time = layout.quantize_time(pos.x, false);
							let note = sheet.get_note(*idx).unwrap();
							if time > note.start && time != note.end() {
								let dist = time - (note.start + lengths[idx]);
								for (idx, length) in lengths {
									let note = sheet.get_note_mut(*idx).unwrap();
									note.length = *length + dist;
								}
								self.action_effective = true;
								sheet_changed = true;
								self.note_len = time - note.start;
							}
						}
						Action::DeleteNotes(ref mut prev_pos) => {
							let notes_len_before = sheet.notes.len();
							sheet.remove_notes_along(Line::new(*prev_pos, pos), coord.to_board_h(env.get(theme::NOTE_HEIGHT)));
							if notes_len_before != sheet.notes.len() {
								self.action_effective = true;
								sheet_changed = true;
							}
							*prev_pos = pos;
						}
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
				let hover = get_hover(pos, coord, &sheet, env);
				if self.hover != hover {
					ctx.request_paint();
				}
				self.hover = hover;
			}
			Event::MouseUp(_) => {
				match self.action {
					Action::SelectionAdd(p0, p1) => {
						let notes =
							sheet.get_notes_rect(Rect::from_points(p0, p1), coord.to_board_h(env.get(theme::NOTE_HEIGHT)));
						if notes.len() != 0 {
							let mut selection = data.selection.borrow_mut();
							selection.extend(notes);
							self.action_effective = true;
						}
					}
					Action::SelectionRemove(p0, p1) => {
						let notes =
							sheet.get_notes_rect(Rect::from_points(p0, p1), coord.to_board_h(env.get(theme::NOTE_HEIGHT)));
						if notes.len() != 0 {
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
				}
				self.action = Action::Idle;
				ctx.set_active(false);
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
					if sheet.get_note_mut(interval_input.0).is_none() {
						self.interval_input = None;
					}
				}
				let bounds = sheet.get_bounds();
				data.frame.x.bounds.1 = ((bounds.0).1 * 1.25).max(5.0);
			}
			Event::Command(ref cmd) if cmd.is(ADD_RELATIVE_NOTE) => {
				let (root, time) = *cmd.get_unchecked(ADD_RELATIVE_NOTE);
				let note = layout.quantize_note(Note {
					start: time,
					length: self.note_len,
					pitch: Pitch::Relative(root, Interval::Ratio(3, 2)),
				});
				sheet.add_note(note);
				sheet_changed = true;
			}
			Event::Command(ref cmd) if cmd.is(DUPLICATE_NOTE) => {
				let (original, time) = *cmd.get_unchecked(DUPLICATE_NOTE);
				if let Some(original) = sheet.get_note(original) {
					let note = layout.quantize_note(Note {
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
				sheet.remove_note(id);
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
		let sheet = data.sheet.borrow();
		let selection = data.selection.borrow();
		self.draw_notes(ctx, &coord, &sheet, &selection, env);

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
