use crate::data::Frame2;
use crate::state::{sheet_editor::Message, Message as RootMessage};
use crate::util::coord::Coord;

pub struct Shortcuts;

use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
	event,
	keyboard::{self, KeyCode},
	layout,
	mouse::{self, ScrollDelta},
	Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle, Widget,
};

impl<'a, B> Widget<RootMessage, Renderer<B>> for Shortcuts
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

	fn hash_layout(&self, _action: &mut Hasher) {}

	fn on_event(
		&mut self,
		event: Event,
		layout: Layout,
		cursor_position: Point,
		messages: &mut Vec<RootMessage>,
		renderer: &Renderer<B>,
		clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let captured = match event {
			Event::Keyboard(keyboard::Event::KeyPressed {
				key_code: key,
				modifiers: mods,
			}) => match key {
				KeyCode::Space => {
					messages.push(Message::Play.into());
					true
				}
				_ => false,
			},
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
		_layout: Layout,
		_cursor_poition: iced::Point,
		viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		(Primitive::None, mouse::Interaction::Idle)
	}
}

impl<'a, B> Into<Element<'a, RootMessage, Renderer<B>>> for Shortcuts
where
	B: Backend,
{
	fn into(self) -> Element<'a, RootMessage, Renderer<B>> {
		Element::new(self)
	}
}
