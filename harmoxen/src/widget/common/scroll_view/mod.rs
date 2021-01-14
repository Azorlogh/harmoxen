use crate::data::{Frame2, Point};
use crate::util::coord::Coord;
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
	event,
	keyboard::{self, Modifiers},
	layout,
	mouse::{self, ScrollDelta},
	Clipboard, Element, Event, Hasher, Layout, Length, Rectangle, Widget,
};

#[derive(Default)]
pub struct State {
	mods: Modifiers,
	cursor: Point,
	xmove: f32,
	ymove: f32,
	xzoom: f32,
	yzoom: f32,
}

pub fn tick(state: &mut State, frame: &mut Frame2, _dt: f32) -> bool {
	let mut moving = false;
	if state.xmove.abs() > 1e-3 {
		frame.x.view += state.xmove * 0.2;
		state.xmove *= 0.8;
		moving = true;
	}
	if state.ymove.abs() > 1e-3 {
		frame.y.view += state.ymove * 0.2;
		state.ymove *= 0.8;
		moving = true;
	}
	if state.xzoom.abs() > 1e-3 {
		frame.x.view.scale_around(state.cursor.x, 2f32.powf(state.xzoom * 0.2));
		frame.x.view -= frame.x.view.0.min(0.0);
		state.xzoom *= 0.8;
		moving = true;
	}
	if state.yzoom.abs() > 1e-3 {
		frame.y.view.scale_around(state.cursor.y, 2f32.powf(state.yzoom * 0.2));
		frame.y.view -= frame.y.view.0.min(0.0);
		state.yzoom *= 0.8;
		moving = true;
	}
	return moving;
}

pub struct ScrollView<'a, Message> {
	state: &'a mut State,
	frame: &'a Frame2,
	limits: [(bool, bool); 2], // TODO: use this
	on_change: Box<dyn Fn() -> Message>,
}

impl<'a, Message> ScrollView<'a, Message> {
	pub fn new(
		state: &'a mut State,
		frame: &'a Frame2,
		limits: [(bool, bool); 2],
		on_change: impl 'static + Fn() -> Message,
	) -> ScrollView<'a, Message> {
		ScrollView {
			state,
			frame,
			limits,
			on_change: Box::new(on_change),
		}
	}
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for ScrollView<'a, Message>
where
	Message: Clone,
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
		cursor_position: iced::Point,
		messages: &mut Vec<Message>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let lbounds = layout.bounds();
		let size = layout.bounds().size();
		match event {
			Event::Keyboard(keyboard::Event::ModifiersChanged(mods)) => {
				self.state.mods = mods;
			}
			Event::Mouse(mouse::Event::CursorMoved { .. }) => {
				let lposition: Point = lbounds.position().into();
				let cursor_position = Into::<Point>::into(cursor_position) - lposition.to_vec2();
				let coord = Coord::new(self.frame.clone(), size);
				self.state.cursor = coord.to_board_p(cursor_position.into());
			}
			Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
				let delta = match delta {
					ScrollDelta::Lines { x, y } => (x * 72.0, y * 72.0),
					ScrollDelta::Pixels { x, y } => (x, y),
				};
				let mods = self.state.mods;
				if delta.0 != 0.0 {
					let delta = delta.0;
					let factor = self.frame.x.view.size();
					self.state.xmove += (delta * factor * 0.002).max(-self.frame.x.view.0 - self.state.xmove);
				}
				if delta.1 != 0.0 {
					if mods.alt {
						self.state.xzoom += (-delta.1 / 120.0) * 0.2;
					} else if mods.control {
						self.state.yzoom += (-delta.1 / 120.0) * 0.2;
					} else if mods.shift {
						let factor = self.frame.x.view.size();
						self.state.xmove +=
							(-delta.1 * factor / layout.bounds().width).max(-self.frame.x.view.0 - self.state.xmove);
					} else {
						let factor = self.frame.y.view.size();
						self.state.ymove += delta.1 * factor / layout.bounds().height;
					};
				}

				messages.push((self.on_change)())
			}
			_ => {}
		}
		event::Status::Ignored
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

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for ScrollView<'a, Message>
where
	Message: 'a + Clone,
	B: Backend,
{
	fn into(self) -> Element<'a, Message, Renderer<B>> {
		Element::new(self)
	}
}
