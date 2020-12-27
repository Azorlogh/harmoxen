use crate::data::{icp, Frame2, Point, Rect, Size};
use crate::state::Message as RootMessage;
use crate::util::coord::Coord;
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{event, layout, mouse, Clipboard, Color, Element, Event, Hasher, Layout, Length, Rectangle, Vector, Widget};

mod style;
pub use style::{Style, StyleSheet};

pub enum State {
	Idle,
	Active(f32),
}

impl Default for State {
	fn default() -> Self {
		Self::Idle
	}
}

pub struct Preview<'a> {
	state: &'a mut State,
	frame: Frame2,
	style: Box<dyn StyleSheet>,
}

impl<'a> Preview<'a> {
	pub fn new(state: &'a mut State, frame: Frame2) -> Self {
		Self {
			state,
			frame,
			style: Default::default(),
		}
	}

	pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
		self.style = style.into();
		self
	}
}

impl<'a, B> Widget<RootMessage, Renderer<B>> for Preview<'a>
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
		layout: Layout<'_>,
		cursor_position: iced::Point,
		messages: &mut Vec<RootMessage>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let lbounds = layout.bounds();
		let mouse_pos = || Into::<Point>::into(cursor_position) - Into::<Point>::into(lbounds.position()).to_vec2();
		let coord = Coord::new(self.frame, lbounds.size());
		match event {
			Event::Mouse(event) => match event {
				mouse::Event::ButtonPressed(mouse::Button::Left) => {
					if lbounds.contains(cursor_position) {
						let pos = coord.to_board_y(mouse_pos().y);
						*self.state = State::Active(pos);
						messages.push(RootMessage::Backend(
							icp::Event::NotePlay(icp::Note {
								id: 2000,
								freq: 2f32.powf(pos),
							})
							.into(),
						));
					}
				}
				mouse::Event::CursorMoved { .. } => {
					if let State::Active(_) = self.state {
						let pos = coord.to_board_y(mouse_pos().y);
						*self.state = State::Active(pos);
						messages.push(RootMessage::Backend(icp::Event::NoteChangeFreq(2000, 2f32.powf(pos)).into()));
					}
				}
				mouse::Event::ButtonReleased(mouse::Button::Left) => {
					if let State::Active(_) = &self.state {
						*self.state = State::Idle;
						messages.push(RootMessage::Backend(icp::Event::NoteStop(2000).into()));
					}
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
		_cursor_position: iced::Point,
		_viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		let bounds = layout.bounds();
		let coord = Coord::new(self.frame, layout.bounds().size());
		let cursor = if let State::Active(pos) = &self.state {
			let s_pos = coord.to_screen_y(*pos);
			Primitive::Clip {
				bounds,
				offset: Vector::new(0, 0),
				content: Box::new(Primitive::Quad {
					bounds: Rect::from_point_size(Point::new(bounds.x, bounds.y + s_pos), Size::new(bounds.height, 1.0)).into(),
					background: Color::WHITE.into(),
					border_color: Color::TRANSPARENT,
					border_radius: 0,
					border_width: 0,
				}),
			}
		} else {
			Primitive::None
		};
		(
			Primitive::Group {
				primitives: vec![
					Primitive::Quad {
						bounds,
						background: self.style.active().background,
						border_color: Color::TRANSPARENT.into(),
						border_radius: 0,
						border_width: 0,
					},
					cursor,
				],
			},
			mouse::Interaction::Idle,
		)
	}
}

impl<'a, B> Into<Element<'a, RootMessage, Renderer<B>>> for Preview<'a>
where
	B: Backend,
{
	fn into(self) -> Element<'a, RootMessage, Renderer<B>> {
		Element::new(self)
	}
}
