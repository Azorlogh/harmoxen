use crate::state::{sheet_editor::Message, Message as RootMessage};

pub struct Shortcuts;

use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
	event,
	keyboard::{self, KeyCode},
	layout,
	mouse::{self},
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

	fn hash_layout(&self, state: &mut Hasher) {
		use std::{any::TypeId, hash::Hash};
		struct Marker;
		TypeId::of::<Marker>().hash(state);
	}

	fn on_event(
		&mut self,
		event: Event,
		_layout: Layout,
		_cursor_position: Point,
		messages: &mut Vec<RootMessage>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
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
				KeyCode::A if mods.control => {
					messages.push(Message::SelectAll.into());
					true
				}
				KeyCode::X if mods.control => {
					messages.push(Message::Cut.into());
					true
				}
				KeyCode::C if mods.control => {
					messages.push(Message::Copy.into());
					true
				}
				KeyCode::V if mods.control => {
					messages.push(Message::Paste.into());
					true
				}
				KeyCode::Delete => {
					messages.push(Message::Delete.into());
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
		_viewport: &Rectangle,
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
