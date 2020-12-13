use crate::data::{Frame2, Point, Rect, Size};
use crate::state::{sheet_editor::Message, Message as RootMessage};
use crate::util::coord::Coord;
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{event, layout, mouse, Clipboard, Color, Element, Event, Hasher, Layout, Length, Rectangle, Vector, Widget};

pub enum State {
	Idle,
	Active,
}

impl Default for State {
	fn default() -> Self {
		Self::Idle
	}
}

pub struct Cursor<'a> {
	state: &'a mut State,
	pos: f32,
	frame: Frame2,
}

impl<'a> Cursor<'a> {
	pub fn new(state: &'a mut State, pos: f32, frame: Frame2) -> Self {
		Self { state, pos, frame }
	}
}

impl<'a, B> Widget<RootMessage, Renderer<B>> for Cursor<'a>
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
						let pos = coord.to_board_x(mouse_pos().x);
						*self.state = State::Active;
						messages.push(Message::SetCursor(pos).into());
					}
				}
				mouse::Event::CursorMoved { .. } => {
					if let State::Active = self.state {
						let pos = coord.to_board_x(mouse_pos().x);
						messages.push(Message::SetCursor(pos).into());
					}
				}
				mouse::Event::ButtonReleased(mouse::Button::Left) => {
					*self.state = State::Idle;
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
		let s_pos = coord.to_screen_x(self.pos);
		(
			Primitive::Clip {
				bounds,
				offset: Vector::new(0, 0),
				content: Box::new(Primitive::Quad {
					bounds: Rect::from_point_size(Point::new(bounds.x + s_pos, bounds.y), Size::new(1.0, bounds.height)).into(),
					background: Color::WHITE.into(),
					border_color: Color::TRANSPARENT,
					border_radius: 0,
					border_width: 0,
				}),
			},
			mouse::Interaction::Idle,
		)
	}
}

impl<'a, B> Into<Element<'a, RootMessage, Renderer<B>>> for Cursor<'a>
where
	B: Backend,
{
	fn into(self) -> Element<'a, RootMessage, Renderer<B>> {
		Element::new(self)
	}
}
