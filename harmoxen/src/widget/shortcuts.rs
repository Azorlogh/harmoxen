use crate::state::Message;

pub struct Shortcuts;

use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
	event,
	keyboard::{self, KeyCode},
	layout, mouse, Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle, Widget,
};

impl<'a, B> Widget<Message, Renderer<B>> for Shortcuts
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
		messages: &mut Vec<Message>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let captured = match event {
			Event::Keyboard(keyboard::Event::KeyPressed {
				key_code: key,
				modifiers: mods,
			}) => match key {
				KeyCode::N if mods.control => {
					messages.push(Message::ProjectNew);
					true
				}
				KeyCode::O if mods.control => {
					messages.push(Message::ProjectOpen);
					true
				}
				KeyCode::S if mods.control => {
					messages.push(Message::ProjectSave);
					true
				}
				KeyCode::S if mods.control && mods.shift => {
					messages.push(Message::ProjectSaveAs);
					true
				}
				KeyCode::Z if mods.control => {
					messages.push(Message::Undo);
					true
				}
				KeyCode::Y if mods.control => {
					messages.push(Message::Redo);
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

impl<'a, B> Into<Element<'a, Message, Renderer<B>>> for Shortcuts
where
	B: Backend,
{
	fn into(self) -> Element<'a, Message, Renderer<B>> {
		Element::new(self)
	}
}
