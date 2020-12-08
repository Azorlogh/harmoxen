#![allow(unused)]

use crate::data::{Axis, Frame, Range};
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
	event, layout, mouse, Background, Clipboard, Color, Element, Event, Hasher, Layout, Length, Point, Rectangle, Size, Vector,
	Widget,
};

pub type State = Action;

mod style;
pub use style::{Style, StyleSheet};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Side {
	Start,
	End,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Action {
	Idle,
	Moving(f32),
	Scaling(Side),
}

impl Default for Action {
	fn default() -> Action {
		Action::Idle
	}
}

pub struct RangeSlider<'a, Message> {
	action: &'a mut Action,
	frame: Frame,
	dir: Axis,
	limits: (bool, bool),
	reversed: bool,
	on_change: Box<dyn Fn(Range) -> Message>,
	style: Box<dyn StyleSheet>,
}

impl<'a, Message> RangeSlider<'a, Message> {
	pub fn horizontal<F>(
		action: &'a mut Action,
		frame: Frame,
		limits: (bool, bool),
		reversed: bool,
		on_change: F,
	) -> RangeSlider<'a, Message>
	where
		F: 'static + Fn(Range) -> Message,
	{
		RangeSlider {
			action,
			frame,
			dir: Axis::X,
			limits,
			reversed,
			on_change: Box::new(on_change),
			style: Default::default(),
		}
	}

	pub fn vertical<F>(
		action: &'a mut Action,
		frame: Frame,
		limits: (bool, bool),
		reversed: bool,
		on_change: F,
	) -> RangeSlider<'a, Message>
	where
		F: 'static + Fn(Range) -> Message,
	{
		RangeSlider {
			action,
			frame,
			dir: Axis::Y,
			limits,
			reversed,
			on_change: Box::new(on_change),
			style: Default::default(),
		}
	}

	pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
		self.style = style.into();
		self
	}
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for RangeSlider<'a, Message>
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
		layout: Layout<'_>,
		cursor_position: Point,
		messages: &mut Vec<Message>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let style = match self.action {
			Action::Idle => self.style.active(),
			Action::Moving(_) => self.style.hovered(),
			Action::Scaling(_) => self.style.hovered(),
		};
		let lbounds = layout.bounds();
		let dir = self.dir;
		let bounds = &self.frame.bounds;
		let view = &self.frame.view;
		let pos = if self.reversed {
			dir.major(lbounds.position()) + dir.major(lbounds.size()) - dir.major(cursor_position) - style.padding
		} else {
			dir.major(cursor_position) - dir.major(lbounds.position()) - style.padding
		};
		match event {
			Event::Mouse(evt) => match evt {
				mouse::Event::ButtonPressed(mouse::Button::Left) => {
					let size = dir.major(lbounds.size()) - style.padding * 2.0;
					let gpos = pos * (bounds.size() / size) + bounds.0;
					let screen_view = (*view - bounds.0) * (size as f32 / bounds.size() as f32);
					if lbounds.contains(cursor_position) {
						if (screen_view.0 + style.handle_offset - pos).abs() < style.handle_offset {
							*self.action = State::Scaling(Side::Start);
						} else if (screen_view.1 - style.handle_offset - pos).abs() < style.handle_offset {
							*self.action = State::Scaling(Side::End);
						} else if pos < screen_view.0 {
							messages.push((self.on_change)(
								*view - (bounds.size() / 8.0).min(view.0 - gpos).min(view.0 - bounds.0),
							));
						} else if pos > screen_view.1 {
							messages.push((self.on_change)(
								*view + (bounds.size() / 8.0).min(gpos - view.1).min(bounds.1 - view.1),
							));
						} else {
							*self.action = Action::Moving(gpos - view.0);
						}
					}
				}
				mouse::Event::CursorMoved { .. } => {
					let size = dir.major(lbounds.size());
					let gpos = pos * (bounds.size() / (size - style.padding * 2.0)) + bounds.0;
					let screen_view = (*view - bounds.0) * (size as f32 / bounds.size() as f32);
					match *self.action {
						State::Moving(anchor) => {
							let mut new = gpos - anchor;
							if self.limits.0 {
								new = new.max(bounds.0);
							}
							if self.limits.1 {
								new = new.min(bounds.1 - view.size());
							}
							messages.push((self.on_change)(*view - view.0 + new));
						}
						State::Scaling(Side::Start) => {
							if pos < screen_view.1 - style.handle_offset * 6.0 {
								if !self.limits.0 || pos > 3.0 + style.handle_offset {
									messages.push((self.on_change)(Range(
										gpos - style.handle_offset * (bounds.size() / size),
										view.1,
									)));
								} else {
									messages.push((self.on_change)(Range(bounds.0, view.1)));
								}
							} else {
								messages.push((self.on_change)(Range(
									view.1 - style.handle_offset * 6.0 * (bounds.size() / size),
									view.1,
								)));
							}
						}
						State::Scaling(Side::End) => {
							if pos > screen_view.0 + style.handle_offset * 6.0 {
								if !self.limits.1 || pos < size - 3.0 - style.handle_offset {
									messages.push((self.on_change)(Range(
										view.0,
										gpos + style.handle_offset * (bounds.size() / size),
									)));
								} else {
									messages.push((self.on_change)(Range(view.0, bounds.1)));
								}
							} else {
								messages.push((self.on_change)(Range(
									view.0,
									view.0 + style.handle_offset * 6.0 * (bounds.size() / size),
								)));
							}
						}
						_ => {}
					}
				}
				mouse::Event::ButtonReleased(mouse::Button::Left) => {
					*self.action = Action::Idle;
				}
				_ => {}
			},
			_ => {}
		}
		event::Status::Ignored
	}

	fn draw(
		&self,
		_renderer: &mut Renderer<B>,
		_defaults: &Defaults,
		layout: Layout,
		_cursor_position: Point,
		viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		let style = match self.action {
			Action::Idle => self.style.active(),
			Action::Moving(_) => self.style.hovered(),
			Action::Scaling(_) => self.style.hovered(),
		};
		let size = layout.bounds().size();
		let size_major = self.dir.major(size);
		let view = self.frame.view;
		let bounds = self.frame.bounds;
		let bg = Primitive::Quad {
			bounds: layout.bounds(),
			background: style.background,
			border_radius: style.border_radius,
			border_width: style.border_width,
			border_color: style.border_color,
		};
		let rview = if self.reversed {
			Range(
				1.0 - (view.1 - bounds.0) / bounds.size(),
				1.0 - (view.0 - bounds.0) / bounds.size(),
			)
		} else {
			Range((view.0 - bounds.0) / bounds.size(), (view.1 - bounds.0) / bounds.size())
		};
		let bar = Primitive::Quad {
			bounds: Rectangle::new(
				self.dir.with_major(
					Point::new(style.padding + layout.bounds().x, style.padding + layout.bounds().y),
					self.dir.major(layout.bounds().position()) + style.padding + (size_major - style.padding * 2.0) * rview.0,
				),
				self.dir.with_major(
					Size::new(size.width - style.padding * 2.0, size.height - style.padding * 2.0),
					(size_major - style.padding * 2.0) * rview.size(),
				),
			),
			background: style.bar_color.into(),
			border_radius: style.bar_border_radius,
			border_width: style.bar_border_width,
			border_color: style.bar_border_color,
		};
		let output = Primitive::Group {
			primitives: vec![
				bg,
				Primitive::Clip {
					bounds: layout.bounds(),
					offset: Vector::new(0, 0),
					content: Box::new(bar),
				},
			],
		};
		(output, mouse::Interaction::default())
	}
}

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for RangeSlider<'a, Message>
where
	Message: 'a + Clone,
	B: Backend,
{
	fn into(self) -> Element<'a, Message, Renderer<B>> {
		Element::new(self)
	}
}
