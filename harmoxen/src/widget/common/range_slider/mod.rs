#![allow(unused)]

use crate::{
	data::{Axis, Frame, Range},
	Message,
};
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
	event, layout, mouse, Background, Clipboard, Color, Element, Event, Hasher, Layout, Length, Point, Rectangle, Size, Vector,
	Widget,
};

pub type State = Action;

mod style;
pub use style::{Style, StyleSheet};

pub enum Hover {
	Before,
	Start,
	Inside,
	End,
	After,
}

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
	handle_offset: f32,
	padding: f32,
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
			handle_offset: 8.0,
			padding: 3.0,
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
			handle_offset: 8.0,
			padding: 3.0,
		}
	}

	pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
		self.style = style.into();
		self
	}

	pub fn hover(&self, screen_view: Range, pos: f32) -> Hover {
		if (screen_view.0 + self.handle_offset - pos).abs() < self.handle_offset {
			return Hover::Start;
		} else if (screen_view.1 - self.handle_offset - pos).abs() < self.handle_offset {
			return Hover::End;
		} else if pos < screen_view.0 {
			return Hover::Before;
		} else if pos > screen_view.1 {
			return Hover::After;
		} else {
			return Hover::Inside;
		}
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
		let style = self.style.active();
		let lbounds = layout.bounds();
		let dir = self.dir;
		let bounds = &self.frame.bounds;
		let view = &self.frame.view;
		let pos = if self.reversed {
			dir.major(lbounds.position()) + dir.major(lbounds.size()) - dir.major(cursor_position) - self.padding
		} else {
			dir.major(cursor_position) - dir.major(lbounds.position()) - self.padding
		};
		match event {
			Event::Mouse(evt) => match evt {
				mouse::Event::ButtonPressed(mouse::Button::Left) => {
					let size = dir.major(lbounds.size()) - self.padding * 2.0;
					let gpos = pos * (bounds.size() / size) + bounds.0;
					let screen_view = (*view - bounds.0) * (size as f32 / bounds.size() as f32);
					if lbounds.contains(cursor_position) {
						match self.hover(screen_view, pos) {
							Hover::Start => {
								*self.action = State::Scaling(Side::Start);
							}
							Hover::End => *self.action = State::Scaling(Side::End),
							Hover::Before => {
								messages.push((self.on_change)(
									*view - (bounds.size() / 8.0).min(view.0 - gpos).min(view.0 - bounds.0),
								));
							}
							Hover::After => {
								messages.push((self.on_change)(
									*view + (bounds.size() / 8.0).min(gpos - view.1).min(bounds.1 - view.1),
								));
							}
							Hover::Inside => *self.action = Action::Moving(gpos - view.0),
						}
					}
				}
				mouse::Event::CursorMoved { .. } => {
					let size = dir.major(lbounds.size());
					let gpos = pos * (bounds.size() / (size - self.padding * 2.0)) + bounds.0;
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
							if pos < screen_view.1 - self.handle_offset * 6.0 {
								if !self.limits.0 || pos > 3.0 + self.handle_offset {
									messages.push((self.on_change)(Range(
										gpos - self.handle_offset * (bounds.size() / size),
										view.1,
									)));
								} else {
									messages.push((self.on_change)(Range(bounds.0, view.1)));
								}
							} else {
								messages.push((self.on_change)(Range(
									view.1 - self.handle_offset * 6.0 * (bounds.size() / size),
									view.1,
								)));
							}
						}
						State::Scaling(Side::End) => {
							if pos > screen_view.0 + self.handle_offset * 6.0 {
								if !self.limits.1 || pos < size - 3.0 - self.handle_offset {
									messages.push((self.on_change)(Range(
										view.0,
										gpos + self.handle_offset * (bounds.size() / size),
									)));
								} else {
									messages.push((self.on_change)(Range(view.0, bounds.1)));
								}
							} else {
								messages.push((self.on_change)(Range(
									view.0,
									view.0 + self.handle_offset * 6.0 * (bounds.size() / size),
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
		cursor_position: Point,
		viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		let style = self.style.active();
		let lbounds = layout.bounds();
		let size = lbounds.size();
		let size_m = self.dir.major(size);
		let size_inner = Size::new(size.width - self.padding * 2.0, size.height - self.padding * 2.0);
		let size_inner_m = self.dir.major(size_inner);
		let pos = lbounds.position();
		let view = self.frame.view;
		let bounds = self.frame.bounds;
		let cursor_spos = if self.reversed {
			self.dir.major(pos) + self.dir.major(pos) - self.dir.major(cursor_position) - self.padding
		} else {
			self.dir.major(cursor_position) - self.dir.major(pos) - self.padding
		};
		let screen_view = (view - bounds.0) * (size_m as f32 / bounds.size() as f32);
		let mut start_handle_color = style.handle_color;
		let mut end_handle_color = style.handle_color;
		let mut bar_color = style.bar_color;

		match &self.action {
			Action::Scaling(Side::Start) => start_handle_color = style.handle_highlight,
			Action::Scaling(Side::End) => end_handle_color = style.handle_highlight,
			Action::Moving(_) => bar_color = style.bar_highlight,
			_ => {}
		}
		if lbounds.contains(cursor_position) {
			match self.hover(screen_view, cursor_spos) {
				Hover::Start => {
					start_handle_color = style.handle_highlight;
				}
				Hover::End => {
					end_handle_color = style.handle_highlight;
				}
				Hover::Inside => {
					bar_color = style.bar_highlight;
				}
				_ => {}
			}
		}
		let bg = Primitive::Quad {
			bounds: lbounds,
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
		let bar = Primitive::Group {
			primitives: vec![
				Primitive::Quad {
					bounds: Rectangle::new(
						self.dir.with_major(
							Point::new(lbounds.x + self.padding, lbounds.y + self.padding),
							self.dir.major(pos) + self.padding + size_inner_m * rview.0,
						),
						self.dir.with_major(size_inner, size_inner_m * rview.size()),
					),
					background: bar_color.into(),
					border_radius: style.bar_border_radius,
					border_width: style.bar_border_width,
					border_color: style.bar_border_color,
				},
				Primitive::Quad {
					bounds: Rectangle::new(
						self.dir.with_major(
							Point::new(lbounds.x + self.padding, lbounds.y + self.padding),
							self.dir.major(pos) + self.padding + size_inner_m * rview.0,
						),
						self.dir.with_major(size_inner, self.handle_offset * 2.0),
					),
					background: start_handle_color.into(),
					border_radius: style.bar_border_radius,
					border_width: style.bar_border_width,
					border_color: style.bar_border_color,
				},
				Primitive::Quad {
					bounds: Rectangle::new(
						self.dir.with_major(
							Point::new(lbounds.x + self.padding, lbounds.y + self.padding),
							self.dir.major(pos) + self.padding + size_inner_m * rview.1 - self.handle_offset * 2.0,
						),
						self.dir.with_major(size_inner, self.handle_offset * 2.0),
					),
					background: end_handle_color.into(),
					border_radius: style.bar_border_radius,
					border_width: style.bar_border_width,
					border_color: style.bar_border_color,
				},
			],
		};
		let output = Primitive::Group {
			primitives: vec![
				bg,
				Primitive::Clip {
					bounds: lbounds,
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
