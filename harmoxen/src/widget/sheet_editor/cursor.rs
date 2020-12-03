use crate::data::Frame2;
use crate::state::Message;
use crate::util::coord::Coord;
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
	layout, mouse, Color, Element, Hasher, Layout, Length, Point, Rectangle, Size, Vector, Widget,
};

pub struct Cursor {
	pos: f32,
	frame: Frame2,
}

impl Cursor {
	pub fn new(pos: f32, frame: Frame2) -> Self {
		Self { pos, frame }
	}
}

impl<'a, B> Widget<Message, Renderer<B>> for Cursor
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

	fn draw(
		&self,
		_renderer: &mut Renderer<B>,
		_defaults: &Defaults,
		layout: Layout,
		_cursor_position: iced::Point,
		viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		let bounds = layout.bounds();
		let coord = Coord::new(self.frame, layout.bounds().size());
		let s_pos = coord.to_screen_x(self.pos);
		(
			Primitive::Clip {
				bounds,
				offset: Vector::new(0, 0),
				content: Box::new(Primitive::Quad {
					bounds: Rectangle::new(
						Point::new(bounds.x + s_pos, bounds.y),
						Size::new(1.0, bounds.height),
					),
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

impl<'a, B> Into<Element<'a, Message, Renderer<B>>> for Cursor
where
	B: Backend,
{
	fn into(self) -> Element<'a, Message, Renderer<B>> {
		Element::new(self)
	}
}
