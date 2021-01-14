use crate::state::{sheet_editor::Message, Message as RootMessage};
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
	event, keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle, Widget,
};

#[derive(Default)]
pub struct State {
	ctrl: bool,
}

pub struct Selection<'a> {
	state: &'a mut State,
}

impl<'a> Selection<'a> {
	pub fn new(state: &'a mut State) -> Self {
		Self { state }
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
		_layout: Layout,
		_cursor_position: Point,
		messages: &mut Vec<RootMessage>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let state = &mut self.state;
		println!("truc {:?}", event);
		let captured = match event {
			Event::Keyboard(keyboard::Event::ModifiersChanged(mods)) => {
				state.ctrl = mods.control;
				false
			}
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if state.ctrl => {
				println!("begin select");
				true
			}
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) if state.ctrl => {
				println!("begin deselect");
				true
			}
			_ => false,
		};
		println!("state.ctrl == {}", state.ctrl);
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

impl<'a, B> Into<Element<'a, RootMessage, Renderer<B>>> for Selection<'a>
where
	B: Backend,
{
	fn into(self) -> Element<'a, RootMessage, Renderer<B>> {
		Element::new(self)
	}
}
