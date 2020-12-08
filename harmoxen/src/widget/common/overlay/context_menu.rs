//! Build and show dropdown menus.
use iced_native::event::{self, Event};
use iced_native::layout;
use iced_native::mouse;
use iced_native::overlay;
use iced_native::scrollable;
use iced_native::{Clipboard, Container, Element, Hasher, Layout, Length, Point, Rectangle, Scrollable, Size, Vector, Widget};

#[derive(Debug, Clone)]
pub struct Item<Message> {
	pub text: String,
	pub on_select: Message,
}

use std::fmt;
impl<Message> fmt::Display for Item<Message> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.text)
	}
}

impl<Message> Item<Message> {
	pub fn new(text: &str, on_select: Message) -> Self {
		Item {
			text: text.to_string(),
			on_select,
		}
	}
}

/// A list of selectable options.
#[allow(missing_debug_implementations)]
pub struct ContextMenu<'a, Message, Renderer: self::Renderer> {
	state: &'a mut State<Message>,
	padding: u16,
	text_size: Option<u16>,
	font: Renderer::Font,
	style: <Renderer as self::Renderer>::Style,
}

#[allow(unused)]
impl<'a, Message, Renderer> ContextMenu<'a, Message, Renderer>
where
	Message: Clone,
	Renderer: self::Renderer + 'a,
{
	/// Creates a new [`ContextMenu`] with the given [`State`], a list of options, and
	/// the message to produced when an option is selected.
	///
	/// [`ContextMenu`]: struct.ContextMenu.html
	/// [`State`]: struct.State.html
	pub fn new(state: &'a mut State<Message>) -> Self {
		ContextMenu {
			state,
			padding: 0,
			text_size: None,
			font: Default::default(),
			style: Default::default(),
		}
	}

	/// Sets the padding of the [`ContextMenu`].
	///
	/// [`ContextMenu`]: struct.ContextMenu.html
	pub fn padding(mut self, padding: u16) -> Self {
		self.padding = padding;
		self
	}

	/// Sets the text size of the [`ContextMenu`].
	///
	/// [`ContextMenu`]: struct.ContextMenu.html
	pub fn text_size(mut self, text_size: u16) -> Self {
		self.text_size = Some(text_size);
		self
	}

	/// Sets the font of the [`ContextMenu`].
	///
	/// [`ContextMenu`]: struct.ContextMenu.html
	pub fn font(mut self, font: Renderer::Font) -> Self {
		self.font = font;
		self
	}

	/// Sets the style of the [`ContextMenu`].
	///
	/// [`ContextMenu`]: struct.ContextMenu.html
	pub fn style(mut self, style: impl Into<<Renderer as self::Renderer>::Style>) -> Self {
		self.style = style.into();
		self
	}

	/// Turns the [`ContextMenu`] into an overlay [`Element`] at the given target
	/// position.
	///
	/// The `target_height` will be used to display the menu either on top
	/// of the target or under it, depending on the screen position and the
	/// dimensions of the [`ContextMenu`].
	///
	/// [`ContextMenu`]: struct.ContextMenu.html
	pub fn overlay(self, position: Point) -> overlay::Element<'a, Message, Renderer> {
		overlay::Element::new(position, Box::new(Overlay::new(self)))
	}
}

/// The local state of a [`ContextMenu`].
///
/// [`ContextMenu`]: struct.ContextMenu.html
#[derive(Debug, Clone)]
pub struct State<Message> {
	scrollable: scrollable::State,
	items: Vec<Item<Message>>,
	hovered_option: Option<usize>,
}

impl<Message> Default for State<Message> {
	fn default() -> Self {
		Self {
			scrollable: Default::default(),
			items: vec![],
			hovered_option: None,
		}
	}
}

impl<Message> State<Message> {
	/// Creates a new [`State`] for a [`ContextMenu`].
	///
	/// [`State`]: struct.State.html
	/// [`ContextMenu`]: struct.ContextMenu.html
	pub fn new(items: Vec<Item<Message>>) -> Self {
		Self {
			items,
			..Default::default()
		}
	}
}

struct Overlay<'a, Message, Renderer: self::Renderer> {
	padding: u16,
	text_size: Option<u16>,
	texts: Vec<String>,
	container: Container<'a, Message, Renderer>,
	style: <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer: self::Renderer> Overlay<'a, Message, Renderer>
where
	Message: 'a + Clone,
	Renderer: 'a,
{
	pub fn new(menu: ContextMenu<'a, Message, Renderer>) -> Self {
		let ContextMenu {
			state,
			padding,
			font,
			text_size,
			style,
		} = menu;

		let container = Container::new(Scrollable::new(&mut state.scrollable).push(List {
			items: &state.items,
			hovered_option: &mut state.hovered_option,
			font,
			text_size,
			padding,
			style: style.clone(),
		}))
		.padding(1);

		Self {
			padding,
			text_size,
			texts: state.items.iter().map(|item| item.text.clone()).collect(),
			container,
			style,
		}
	}
}

impl<'a, Message, Renderer> iced_native::Overlay<Message, Renderer> for Overlay<'a, Message, Renderer>
where
	Renderer: self::Renderer,
{
	fn layout(&self, renderer: &Renderer, bounds: Size, position: Point) -> layout::Node {
		let space_below = bounds.height - position.y;
		let space_above = position.y;

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

		node.move_to(if space_below > space_above {
			position + Vector::new(0.0, 0.0)
		} else {
			position - Vector::new(0.0, node.size().height)
		});

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

struct List<'a, Message, Renderer: self::Renderer> {
	items: &'a [Item<Message>],
	hovered_option: &'a mut Option<usize>,
	padding: u16,
	text_size: Option<u16>,
	font: Renderer::Font,
	style: <Renderer as self::Renderer>::Style,
}

impl<'a, Message, Renderer: self::Renderer> Widget<Message, Renderer> for List<'a, Message, Renderer>
where
	Message: Clone,
	Renderer: self::Renderer,
{
	fn width(&self) -> Length {
		Length::Fill
	}

	fn height(&self) -> Length {
		Length::Shrink
	}

	fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
		use std::f32;

		let limits = limits.width(Length::Fill).height(Length::Shrink);
		let text_size = self.text_size.unwrap_or(renderer.default_size());

		let size = {
			let intrinsic = Size::new(0.0, f32::from(text_size + self.padding * 2) * self.items.len() as f32);

			limits.resolve(intrinsic)
		};

		layout::Node::new(size)
	}

	fn hash_layout(&self, state: &mut Hasher) {
		use std::hash::Hash as _;

		struct Marker;
		std::any::TypeId::of::<Marker>().hash(state);

		self.items.len().hash(state);
		self.text_size.hash(state);
		self.padding.hash(state);
	}

	fn on_event(
		&mut self,
		event: Event,
		layout: Layout<'_>,
		cursor_position: Point,
		messages: &mut Vec<Message>,
		renderer: &Renderer,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		match event {
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
				let bounds = layout.bounds();

				if bounds.contains(cursor_position) {
					if let Some(index) = *self.hovered_option {
						if let Some(item) = self.items.get(index) {
							messages.push(item.on_select.clone());
						}
						return event::Status::Captured;
					}
				}
			}
			Event::Mouse(mouse::Event::CursorMoved { .. }) => {
				let bounds = layout.bounds();
				let text_size = self.text_size.unwrap_or(renderer.default_size());

				if bounds.contains(cursor_position) {
					*self.hovered_option =
						Some(((cursor_position.y - bounds.y) / f32::from(text_size + self.padding * 2)) as usize);
				}
			}
			_ => {}
		}

		event::Status::Ignored
	}

	fn draw(
		&self,
		renderer: &mut Renderer,
		_defaults: &Renderer::Defaults,
		layout: Layout<'_>,
		cursor_position: Point,
		viewport: &Rectangle,
	) -> Renderer::Output {
		iced_native::overlay::menu::Renderer::draw(
			renderer,
			layout.bounds(),
			cursor_position,
			viewport,
			self.items,
			*self.hovered_option,
			self.padding,
			self.text_size.unwrap_or(renderer.default_size()),
			self.font,
			&self.style,
		)
	}
}

use iced_native::overlay::menu::Renderer;

impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for List<'a, Message, Renderer>
where
	Message: 'a + Clone,
	Renderer: 'a + self::Renderer,
{
	fn into(self) -> Element<'a, Message, Renderer> {
		Element::new(self)
	}
}
