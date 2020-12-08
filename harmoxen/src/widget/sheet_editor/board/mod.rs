use crate::data::{
	icp,
	layout::Layout,
	sheet::{Index, Interval, Note, Pitch, Sheet},
	Frame2, Line, Point, Rect, Vec2,
};
use crate::state::{sheet_editor::Message, Message as RootMessage};
use crate::util::coord::Coord;
use crate::{
	backend,
	widget::{context_menu, ContextMenu},
};
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{event, layout as iced_layout, mouse, overlay, Clipboard, Element, Event, Hasher, Length, Rectangle, Widget};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const NOTE_HEIGHT: f32 = 24.0;
const NOTE_SCALE_KNOB: f32 = 32.0;

mod layout;
mod notes;
mod style;

pub use style::{Style, StyleSheet};

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

#[derive(Debug)]
pub enum Action {
	Idle,
	Move(Index, HashMap<Index, Vec2>, Rect), // root note, offsets to mouse, extent of selection around mouse
	Scale(Index, HashMap<Index, f32>),       // root note, original lengths of notes
	DeleteNotes(Point),
	Context {
		menu: context_menu::State<RootMessage>,
		pos: iced::Point,
	},
}

pub struct State {
	hover: Hover,
	action: Action,
	note_len: f32,
	last_left_click: (Point, Instant),
	action_effective: bool,
}

impl Default for State {
	fn default() -> State {
		State {
			hover: Hover::Idle,
			action: Action::Idle,
			note_len: 1.0,
			last_left_click: (Point::new(f32::INFINITY, f32::INFINITY), Instant::now()),
			action_effective: false,
		}
	}
}

impl State {
	pub fn set_action_move(&mut self, idx: Index, rect: Rect) {
		let mut notes = HashMap::new();
		notes.insert(idx, Vec2::ZERO);
		self.action = Action::Move(idx, notes, rect);
	}
}

pub struct Board<'a> {
	state: &'a mut State,
	sheet: &'a Sheet,
	frame: &'a Frame2,
	layout: &'a Layout,
	selection: &'a HashSet<Index>,
	style: Box<dyn StyleSheet>,
}

impl<'a> Board<'a> {
	pub fn new(
		state: &'a mut State,
		sheet: &'a Sheet,
		frame: &'a Frame2,
		layout: &'a Layout,
		selection: &'a HashSet<Index>,
	) -> Self {
		Self {
			state,
			sheet,
			frame,
			layout,
			selection,
			style: Default::default(),
		}
	}

	pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
		self.style = style.into();
		self
	}

	fn stop_action(&mut self, messages: &mut Vec<RootMessage>, history_save: &mut bool) {
		if self.state.action_effective {
			*history_save = true;
			self.state.action_effective = false;
		}
		self.state.action = Action::Idle;
		messages.push(RootMessage::Backend(backend::Event::ICP(icp::Event::NoteStop(2000))));
	}
}

impl<'a, B> Widget<RootMessage, Renderer<B>> for Board<'a>
where
	Message: Clone,
	B: Backend + iced_graphics::backend::Text + 'static,
{
	fn width(&self) -> Length {
		Length::Fill
	}

	fn height(&self) -> Length {
		Length::Fill
	}

	fn layout(&self, _renderer: &Renderer<B>, limits: &iced_layout::Limits) -> iced_layout::Node {
		iced_layout::Node::new(limits.max())
	}

	fn hash_layout(&self, _action: &mut Hasher) {}

	fn on_event(
		&mut self,
		event: Event,
		layout: iced_native::Layout,
		cursor_position: iced::Point,
		messages: &mut Vec<RootMessage>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let lbounds = layout.bounds();
		let lposition: Point = lbounds.position().into();
		let mouse_pos = Into::<Point>::into(cursor_position) - lposition.to_vec2();
		let mut history_save = false;
		let size = layout.bounds().size();
		let coord = Coord::new(*self.frame, size);

		let state = &mut self.state;

		let sheet = self.sheet;
		let layout = self.layout;
		let selection = self.selection;

		match event {
			Event::Mouse(mouse::Event::ButtonPressed(btn)) if lbounds.contains(cursor_position) => {
				let pos = coord.to_board_p(mouse_pos);
				if let Action::Context { .. } = &state.action {
					state.action = Action::Idle;
				}
				if btn == mouse::Button::Left {
					let is_double_click =
						mouse_pos == state.last_left_click.0 && state.last_left_click.1.elapsed().as_millis() < 500;
					state.last_left_click = (mouse_pos, Instant::now());
					if is_double_click {
						if let Some(id) = get_hover(pos, coord, &self.sheet).note_idx() {
							let mut note = sheet.get_note(id).unwrap();
							note.start += pos.x;
							let items = vec![
								context_menu::Item::new(
									"Add relative note",
									Message::AddNote(
										Note {
											start: pos.x,
											length: self.state.note_len,
											pitch: Pitch::Relative(id, Interval::Ratio(3, 2)),
										},
										false,
									)
									.into(),
								),
								context_menu::Item::new("Duplicate note", Message::AddNote(note, false).into()),
								context_menu::Item::new("Delete note", Message::DeleteNote(id).into()),
							];
							self.state.action = Action::Context {
								menu: context_menu::State::new(items),
								pos: cursor_position,
							};
						}
					} else {
						match state.hover {
							Hover::Idle => {
								let note = layout.quantize_note(Note::new(pos, state.note_len));
								if sheet.get_note_at(Point::new(note.start, note.y(&sheet)), 0.01).is_none() {
									messages.push(Message::AddNote(note, true).into());
									messages.push(RootMessage::Backend(backend::Event::ICP(icp::Event::NotePlay(icp::Note {
										id: 2000,
										freq: sheet.get_freq(note.pitch),
									}))));
								}
							}
							Hover::Move(idx) => {
								if selection.len() > 0 {
									let mut notes = HashMap::new();
									let root = sheet.get_note(idx).unwrap();
									let mut rect = root.rect(&sheet, 0.0);
									for idx in selection.iter() {
										let note = sheet.get_note(*idx).expect("selection contained a dead note");
										let offset = note.start_pt(&sheet) - pos;
										rect = rect.union(&note.rect(&sheet, 0.0));
										notes.insert(*idx, offset);
									}
									rect = rect + -pos.to_vec2();
									state.action = Action::Move(idx, notes, rect);
								} else {
									let note = sheet.get_note(idx).unwrap();
									let mut notes = HashMap::new();
									notes.insert(idx, note.start_pt(&sheet).to_vec2() - pos.to_vec2());
									state.action = Action::Move(idx, notes, note.rect(&sheet, 0.0) - pos.to_vec2());
									let note = sheet.get_note(idx).unwrap();
									state.note_len = note.length;
									messages.push(RootMessage::Backend(backend::Event::ICP(icp::Event::NotePlay(icp::Note {
										id: 2000,
										freq: sheet.get_freq(note.pitch),
									}))));
								}
							}
							Hover::Scale(idx) => {
								if selection.len() > 0 {
									let mut notes = HashMap::new();
									for &idx in selection.iter() {
										notes.insert(idx, sheet.get_note(idx).expect("selection contained a dead note").length);
									}
									state.action = Action::Scale(idx, notes);
								} else {
									let note = sheet.get_note(idx).unwrap();
									state.note_len = note.length;
									state.action = Action::Scale(idx, [(idx, note.length)].iter().cloned().collect());
								}
							}
						}
					}
				} else if btn == mouse::Button::Right {
					if let Some(idx) = sheet.get_note_at(pos, coord.to_board_h(NOTE_HEIGHT)) {
						messages.push(Message::DeleteNote(idx).into());
						state.action_effective = true;
					} else {
						state.action = Action::DeleteNotes(pos);
					}
				}
			}
			Event::Mouse(mouse::Event::CursorMoved { .. }) => {
				if cursor_position.x == -1.0 {
					return event::Status::Ignored;
				}
				let pos = coord.to_board_p(mouse_pos);
				match &mut state.action {
					Action::Move(root_idx, offsets, bounds) => {
						let root_offset = offsets[&root_idx];
						let mut root_start_pt = pos + root_offset;
						root_start_pt.x = root_start_pt.x.max(0.0);
						let mut anchor = layout.quantize_position(root_start_pt) - root_offset;
						anchor.x = anchor.x.max(-bounds.x0);
						for (idx, offset) in offsets {
							let note = sheet.get_note(*idx).unwrap();
							let pos = anchor + *offset;
							if note.start != pos.x || note.y(&sheet) != pos.y {
								messages.push(Message::MoveNote(*idx, pos).into());
								state.action_effective = true;
								if sheet.get_y(note.pitch) != pos.y {
									messages.push(RootMessage::Backend(backend::Event::ICP(icp::Event::NoteStop(2000))));
									messages.push(RootMessage::Backend(backend::Event::ICP(icp::Event::NotePlay(icp::Note {
										id: 2000,
										freq: sheet.get_freq(Pitch::Absolute(2f32.powf(pos.y))),
									}))));
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
								messages.push(Message::ResizeNote(*idx, *length + dist).into());
							}
							state.action_effective = true;
							state.note_len = time - note.start;
						}
					}
					Action::DeleteNotes(ref mut prev_pos) => {
						for idx in sheet.get_notes_along(Line::new(*prev_pos, pos), coord.to_board_h(NOTE_HEIGHT)) {
							state.action_effective = true;
							messages.push(Message::DeleteNote(idx).into());
						}
						*prev_pos = pos;
					}
					_ => {}
				}
				state.hover = get_hover(pos, coord, &sheet);
			}
			Event::Mouse(mouse::Event::ButtonReleased(_)) => match self.state.action {
				Action::Context { .. } => {}
				_ => self.stop_action(messages, &mut history_save),
			},
			_ => {}
		}
		event::Status::Ignored
	}

	fn draw(
		&self,
		_renderer: &mut Renderer<B>,
		_defaults: &Defaults,
		layout: iced_native::Layout,
		_cursor_position: iced::Point,
		_viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		let offset = Into::<Point>::into(layout.bounds().position()).to_vec2();
		let size = layout.bounds().size();
		let coord = Coord::new(self.frame.clone(), size);
		let style = self.style.active();

		let layout_primitives = self.draw_layout(size, &coord, self.layout, style);

		let notes = self.draw_notes(&coord, style);

		(
			Primitive::Clip {
				bounds: layout.bounds(),
				offset: Vec2::<u32>::new(0, 0).into(),
				content: Box::new(Primitive::Translate {
					translation: offset.into(),
					content: Box::new(Primitive::Group {
						primitives: vec![layout_primitives, notes],
					}),
				}),
			},
			mouse::Interaction::Idle,
		)
	}

	fn overlay(&mut self, _layout: iced_native::Layout) -> Option<overlay::Element<'_, RootMessage, Renderer<B>>> {
		if let Action::Context { menu, pos } = &mut self.state.action {
			Some(ContextMenu::new(menu).padding(4).overlay(pos.clone()))
		} else {
			None
		}
	}
}

fn get_hover(pos: Point, coord: Coord, sheet: &Sheet) -> Hover {
	let hovered_note_idx = sheet.get_note_at(pos, coord.to_board_h(NOTE_HEIGHT));
	match hovered_note_idx {
		None => Hover::Idle,
		Some(idx) => {
			let note = sheet.get_note(idx).unwrap();
			if pos.x > note.end() - coord.to_board_w(NOTE_SCALE_KNOB) && pos.x > note.start + note.length * 0.60 {
				Hover::Scale(idx)
			} else {
				Hover::Move(idx)
			}
		}
	}
}

impl<'a, B> Into<Element<'a, RootMessage, Renderer<B>>> for Board<'a>
where
	Message: 'a + Clone,
	B: Backend + iced_graphics::backend::Text + 'static,
{
	fn into(self) -> Element<'a, RootMessage, Renderer<B>> {
		Element::new(self)
	}
}
