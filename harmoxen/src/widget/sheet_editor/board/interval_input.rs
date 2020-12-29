use crate::widget::text_input;
use iced_graphics::{Backend, Renderer};
use iced_native::{layout, Point, Size};

pub struct State {
	text: String,
	text_input_state: text_input::State,
}

pub struct IntervalInput<'a> {
	state: &'a mut State,
}

impl<'a> IntervalInput<'a> {
	pub fn new(state: &mut State) -> IntervalInput {
		IntervalInput { state }
	}
}

impl<'a, Message, B> iced_native::Overlay<Message, Renderer<B>> for IntervalInput<'a>
where
	B: Backend,
{
	fn layout(&self, renderer: &Renderer<B>, bounds: Size, position: Point) -> layout::Node {
		let limits = layout::Limits::new(
			Size::ZERO,
			Size::new(
				bounds.width - position.x,
				if space_below > space_above { space_below } else { space_above },
			),
		)
		.width(Length::Units(
			self.texts
				.iter()
				.map(|text| {
					let (width, _) = renderer.measure(
						&text,
						self.text_size.unwrap_or(renderer.default_size()),
						Renderer::Font::default(),
						Size::new(f32::INFINITY, f32::INFINITY),
					);

					width.round() as u16
				})
				.max()
				.unwrap_or(100) + self.padding as u16 * 2,
		));

		let mut node = self.container.layout(renderer, &limits);
		node
	}

	fn hash_layout(&self, state: &mut Hasher, position: Point) {
		use std::hash::Hash;

		struct Marker;
		std::any::TypeId::of::<Marker>().hash(state);

		(position.x as u32).hash(state);
		(position.y as u32).hash(state);
		self.container.hash_layout(state);
	}

	fn on_event(
		&mut self,
		event: Event,
		layout: Layout<'_>,
		cursor_position: Point,
		messages: &mut Vec<Message>,
		renderer: &Renderer,
		clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		self.container
			.on_event(event.clone(), layout, cursor_position, messages, renderer, clipboard)
	}

	fn draw(
		&self,
		renderer: &mut Renderer,
		defaults: &Renderer::Defaults,
		layout: Layout<'_>,
		cursor_position: Point,
	) -> Renderer::Output {
		let primitives = self
			.container
			.draw(renderer, defaults, layout, cursor_position, &layout.bounds());

		renderer.decorate(layout.bounds(), cursor_position, &self.style, primitives)
	}
}
