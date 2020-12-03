//! Allow your users to perform actions by pressing a button.
//!
//! A [`Button`] has some local [`State`].
//!
//! [`Button`]: struct.Button.html
//! [`State`]: struct.State.html
use iced_graphics::{Backend, Defaults, Primitive, Renderer};
use iced_native::{
	event, layout, mouse, Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle,
	Widget,
};
use std::hash::Hash;

mod style;
pub use style::{Style, StyleSheet};

/// A generic widget that produces a message when pressed.
///
/// ```
/// # use iced_native::{button, Text};
/// #
/// # type Button<'a, Message> =
/// #     iced_native::Button<'a, Message, iced_native::renderer::Null>;
/// #
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// let mut state = button::State::new();
/// let button = Button::new(&mut state, Text::new("Press me!"))
///     .on_press(Message::ButtonPressed);
/// ```
#[allow(missing_debug_implementations)]
pub struct Tab<'a, Message, B: Backend> {
	selected: bool,
	content: Element<'a, Message, Renderer<B>>,
	on_press: Option<Message>,
	width: Length,
	height: Length,
	min_width: u32,
	min_height: u32,
	padding: u16,
	style: Box<dyn StyleSheet>,
}

impl<'a, Message, B> Tab<'a, Message, B>
where
	Message: Clone,
	B: Backend,
{
	/// Creates a new [`Button`] with some local [`State`] and the given
	/// content.
	///
	/// [`Button`]: struct.Button.html
	/// [`State`]: struct.State.html
	pub fn new<E>(selected: bool, content: E) -> Self
	where
		E: Into<Element<'a, Message, Renderer<B>>>,
	{
		Tab {
			selected,
			content: content.into(),
			on_press: None,
			width: Length::Shrink,
			height: Length::Shrink,
			min_width: 0,
			min_height: 0,
			padding: 5,
			style: Default::default(),
		}
	}

	/// Sets the width of the [`Button`].
	///
	/// [`Button`]: struct.Button.html
	pub fn width(mut self, width: Length) -> Self {
		self.width = width;
		self
	}

	/// Sets the height of the [`Button`].
	///
	/// [`Button`]: struct.Button.html
	pub fn height(mut self, height: Length) -> Self {
		self.height = height;
		self
	}

	/// Sets the minimum width of the [`Button`].
	///
	/// [`Button`]: struct.Button.html
	pub fn min_width(mut self, min_width: u32) -> Self {
		self.min_width = min_width;
		self
	}

	/// Sets the minimum height of the [`Button`].
	///
	/// [`Button`]: struct.Button.html
	pub fn min_height(mut self, min_height: u32) -> Self {
		self.min_height = min_height;
		self
	}

	/// Sets the padding of the [`Button`].
	///
	/// [`Button`]: struct.Button.html
	pub fn padding(mut self, padding: u16) -> Self {
		self.padding = padding;
		self
	}

	/// Sets the message that will be produced when the [`Button`] is pressed.
	///
	/// [`Button`]: struct.Button.html
	pub fn on_press(mut self, msg: Message) -> Self {
		self.on_press = Some(msg);
		self
	}

	/// Sets the style of the [`Button`].
	///
	/// [`Button`]: struct.Button.html
	pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
		self.style = style.into();
		self
	}
}

impl<'a, Message, B> Widget<Message, Renderer<B>> for Tab<'a, Message, B>
where
	Message: Clone,
	B: Backend,
{
	fn width(&self) -> Length {
		self.width
	}

	fn height(&self) -> Length {
		self.height
	}

	fn layout(&self, renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
		let padding = f32::from(self.padding);
		let limits = limits
			.min_width(self.min_width)
			.min_height(self.min_height)
			.width(self.width)
			.height(self.height)
			.pad(padding);

		let mut content = self.content.layout(renderer, &limits);
		content.move_to(Point::new(padding, padding));

		let size = limits.resolve(content.size()).pad(padding);

		layout::Node::with_children(size, vec![content])
	}

	fn on_event(
		&mut self,
		event: Event,
		layout: Layout<'_>,
		cursor_position: Point,
		messages: &mut Vec<Message>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		match event {
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
				if let Some(on_press) = self.on_press.clone() {
					let bounds = layout.bounds();

					if bounds.contains(cursor_position) {
						messages.push(on_press);
					}
				}
			}
			_ => {}
		}
		return event::Status::Ignored;
	}

	fn draw(
		&self,
		renderer: &mut Renderer<B>,
		defaults: &Defaults,
		layout: Layout<'_>,
		cursor_position: Point,
		viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		let bounds = layout.bounds();
		let style = if self.selected {
			self.style.selected()
		} else if bounds.contains(cursor_position) {
			self.style.hovered()
		} else {
			self.style.active()
		};

		let background = Primitive::Quad {
			bounds,
			background: style.background,
			border_color: style.border_color,
			border_radius: style.border_radius,
			border_width: style.border_width,
		};

		let (content, _) = self.content.draw(
			renderer,
			defaults,
			layout.children().next().unwrap(),
			cursor_position,
			viewport,
		);

		(
			Primitive::Group {
				primitives: vec![background, content],
			},
			if bounds.contains(cursor_position) && !self.selected {
				mouse::Interaction::Pointer
			} else {
				mouse::Interaction::default()
			},
		)
	}

	fn hash_layout(&self, state: &mut Hasher) {
		struct Marker;
		std::any::TypeId::of::<Marker>().hash(state);

		self.width.hash(state);
		self.content.hash_layout(state);
	}
}

impl<'a, Message, B> From<Tab<'a, Message, B>> for Element<'a, Message, Renderer<B>>
where
	Message: 'a + Clone,
	B: 'a + Backend,
{
	fn from(button: Tab<'a, Message, B>) -> Element<'a, Message, Renderer<B>> {
		Element::new(button)
	}
}
