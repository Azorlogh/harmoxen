//! Stack widgets on top of each other
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{event, layout, mouse, overlay, Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle, Widget};

#[allow(missing_debug_implementations)]
pub struct Stack<'a, Message, B: Backend> {
	children: Vec<Element<'a, Message, Renderer<B>>>,
}

impl<'a, Message, B: Backend> Stack<'a, Message, B> {
	pub fn new() -> Self {
		Self::with_children(Vec::new())
	}

	pub fn with_children(children: Vec<Element<'a, Message, Renderer<B>>>) -> Self {
		Stack { children }
	}

	pub fn push<E>(mut self, child: E) -> Self
	where
		E: Into<Element<'a, Message, Renderer<B>>>,
	{
		self.children.push(child.into());
		self
	}
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for Stack<'a, Message, B>
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

	fn layout(&self, renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
		layout::Node::with_children(
			limits.max(),
			self.children.iter().map(|child| child.layout(renderer, limits)).collect(),
		)
	}

	fn hash_layout(&self, _action: &mut Hasher) {}

	fn on_event(
		&mut self,
		event: Event,
		layout: Layout<'_>,
		cursor_position: Point,
		messages: &mut Vec<Message>,
		renderer: &Renderer<B>,
		clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let layouts: Vec<Layout> = layout.children().collect();
		for i in (0..self.children.len()).rev() {
			let child = &mut self.children[i];
			let status = child.on_event(event.clone(), layouts[i], cursor_position, messages, renderer, clipboard);
			if status == event::Status::Captured {
				return event::Status::Captured;
			}
		}
		event::Status::Ignored
	}

	fn draw(
		&self,
		renderer: &mut Renderer<B>,
		defaults: &Defaults,
		layout: Layout<'_>,
		cursor_position: Point,
		viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		let mut mouse_interaction = mouse::Interaction::default();

		(
			Primitive::Group {
				primitives: self
					.children
					.iter()
					.zip(layout.children())
					.map(|(child, layout)| {
						let (primitive, new_mouse_interaction) =
							child.draw(renderer, defaults, layout, cursor_position, viewport);
						if new_mouse_interaction > mouse_interaction {
							mouse_interaction = new_mouse_interaction;
						}
						primitive
					})
					.collect(),
			},
			mouse_interaction,
		)
	}

	fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer<B>>> {
		self.children
			.iter_mut()
			.zip(layout.children())
			.filter_map(|(child, layout)| child.overlay(layout))
			.next()
	}
}

impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for Stack<'a, Message, B>
where
	Message: 'a + Clone,
	B: Backend + 'static,
{
	fn into(self) -> Element<'a, Message, Renderer<B>> {
		Element::new(self)
	}
}
