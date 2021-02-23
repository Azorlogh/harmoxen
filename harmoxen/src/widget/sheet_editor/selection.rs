use crate::{
	consts::NOTE_HEIGHT,
	data::{
		sheet::{Index, Sheet},
		Frame2, Point, Rect,
	},
	state::{sheet_editor::Message, Message as RootMessage},
	util::coord::Coord,
};
use iced_graphics::{Backend, Color, Defaults, Primitive, Renderer};
use iced_native::{event, keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout, Length, Rectangle, Widget};
use std::collections::HashSet;

pub enum Action {
	Idle,
	SelectionAdd(Point, Point),
	SelectionRemove(Point, Point),
}

pub struct State {
	ctrl: bool,
	action: Action,
}

impl Default for State {
	fn default() -> Self {
		Self {
			ctrl: false,
			action: Action::Idle,
		}
	}
}

pub struct Selection<'a> {
	state: &'a mut State,
	sheet: &'a Sheet,
	frame: &'a Frame2,
	selection: &'a HashSet<Index>,
}

impl<'a> Selection<'a> {
	pub fn new(state: &'a mut State, sheet: &'a Sheet, frame: &'a Frame2, selection: &'a HashSet<Index>) -> Self {
		Self {
			state,
			sheet,
			frame,
			selection,
		}
	}
}

impl<'a, B> Widget<RootMessage, Renderer<B>> for Selection<'a>
where
	B: Backend,
{
	fn width(&self) -> Length {
		Length::Fill
	}

	fn height(&self) -> Length {
		Length::Fill
	}

	fn layout(&self, _renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
		layout::Node::new(limits.max())
	}

	fn hash_layout(&self, state: &mut Hasher) {
		use std::{any::TypeId, hash::Hash};
		struct Marker;
		TypeId::of::<Marker>().hash(state);
	}

	fn on_event(
		&mut self,
		event: Event,
		layout: Layout,
		cursor_position: iced::Point,
		messages: &mut Vec<RootMessage>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let state = &mut self.state;
		let coord = Coord::new(*self.frame, layout.bounds().size());
		let lposition: Point = layout.position().into();
		let mouse_pos = Into::<Point>::into(cursor_position) - lposition.to_vec2();
		let pos = coord.to_board_p(mouse_pos);

		let captured = match event {
			Event::Keyboard(keyboard::Event::ModifiersChanged(mods)) => {
				state.ctrl = mods.control;
				false
			}
			Event::Mouse(mouse::Event::ButtonPressed(btn)) if state.ctrl => {
				if btn == mouse::Button::Left {
					state.action = Action::SelectionAdd(pos, pos);
				} else {
					state.action = Action::SelectionRemove(pos, pos);
				}
				true
			}
			Event::Mouse(mouse::Event::ButtonPressed(_)) => {
				let note = self.sheet.get_note_at(pos, coord.to_board_h(NOTE_HEIGHT));
				false
			}
			Event::Mouse(mouse::Event::CursorMoved { x, y }) => match state.action {
				Action::SelectionAdd(_, ref mut to) => {
					*to = pos;
					true
				}
				Action::SelectionRemove(_, ref mut to) => {
					*to = pos;
					true
				}
				Action::Idle => false,
			},
			Event::Mouse(mouse::Event::ButtonReleased(_)) => {
				let sheet = self.sheet;
				match state.action {
					Action::SelectionAdd(p0, p1) => {
						let notes = sheet.get_notes_rect(Rect::from_points(p0, p1), coord.to_board_h(NOTE_HEIGHT));
						if !notes.is_empty() {
							let mut selection = self.selection.clone();
							selection.extend(notes.into_iter());
							messages.push(Message::SetSelection(selection).into());
						}
					}
					Action::SelectionRemove(p0, p1) => {
						let notes = sheet.get_notes_rect(Rect::from_points(p0, p1), coord.to_board_h(NOTE_HEIGHT));
						if !notes.is_empty() {
							let selection = self.selection.clone();
							messages
								.push(Message::SetSelection(&selection - &notes.into_iter().collect::<HashSet<_>>()).into());
						}
					}
					_ => {}
				}
				state.action = Action::Idle;
				false
			}
			_ => false,
		};
		if captured {
			event::Status::Captured
		} else {
			event::Status::Ignored
		}
	}

	fn draw(
		&self,
		_renderer: &mut Renderer<B>,
		_defaults: &Defaults,
		layout: Layout,
		_cursor_poition: iced::Point,
		_viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		let lpos: Point = layout.position().into();
		let lposv = lpos.to_vec2();
		let coord = Coord::new(*self.frame, layout.bounds().size());
		let primitive = match self.state.action {
			Action::SelectionAdd(p0, p1) => {
				let p0 = coord.to_screen_p(p0) + lposv;
				let p1 = coord.to_screen_p(p1) + lposv;
				Primitive::Quad {
					bounds: Rect::from_points(p0, p1).into(),
					background: Color::from_rgba(0.8, 0.3, 0.2, 0.5).into(),
					border_color: Color::TRANSPARENT,
					border_radius: 0.0,
					border_width: 0.0,
				}
			}
			Action::SelectionRemove(p0, p1) => {
				let p0 = coord.to_screen_p(p0) + lposv;
				let p1 = coord.to_screen_p(p1) + lposv;
				Primitive::Quad {
					bounds: Rect::from_points(p0, p1).into(),
					background: Color::from_rgba(0.2, 0.5, 0.7, 0.5).into(),
					border_color: Color::TRANSPARENT,
					border_radius: 0.0,
					border_width: 0.0,
				}
			}
			_ => Primitive::None,
		};
		(primitive, mouse::Interaction::Idle)
	}
}

impl<'a, B> Into<Element<'a, RootMessage, Renderer<B>>> for Selection<'a>
where
	B: Backend,
{
	fn into(self) -> Element<'a, RootMessage, Renderer<B>> {
		Element::new(self)
	}
}
