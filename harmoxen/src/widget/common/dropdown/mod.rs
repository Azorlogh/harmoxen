#![allow(unused)]

//! Display a dropdown of labeled messages.
use iced_native::{
	button, event,
	event::Status,
	layout, mouse, overlay,
	overlay::{menu, Menu},
	scrollable, text, Button, Clipboard, Element, Event, Hasher, Layout, Length, Point, Rectangle, Size, Widget,
};
use std::fmt;

mod renderer;

#[derive(Clone)]
pub struct Item<Message> {
	pub text: String,
	pub on_select: Message,
}

impl<Message> fmt::Display for Item<Message> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.text)
	}
}

#[allow(missing_debug_implementations)]
pub struct DropDown<'a, Message, Renderer: self::Renderer> {
	state: &'a mut State<Message>,
	label: String,
	items: Vec<Item<Message>>,
	width: Length,
	padding: u16,
	text_size: Option<u16>,
	font: Renderer::Font,
	style: <Renderer as self::Renderer>::Style,
}

#[derive(Clone)]
pub struct State<Message> {
	items_width: u16,
	menu: menu::State,
	is_open: bool,
	hovered_item: Option<usize>,
	last_selection: Option<Item<Message>>,
}

impl<Message> Default for State<Message> {
	fn default() -> Self {
		Self {
			items_width: 0,
			menu: menu::State::default(),
			is_open: bool::default(),
			hovered_item: Option::default(),
			last_selection: Option::default(),
		}
	}
}

impl<'a, Message, Renderer: self::Renderer> DropDown<'a, Message, Renderer> {
	/// Creates a new [`PickList`] with the given [`State`], a list of options,
	/// the current selected value, and the message to produce when an option is
	/// selected.
	///
	/// [`PickList`]: struct.PickList.html
	/// [`State`]: struct.State.html
	pub fn new(state: &'a mut State<Message>, label: &'static str, mut items: Vec<(&'static str, Message)>) -> Self {
		Self {
			state,
			label: label.to_string(),
			items: items
				.drain(..)
				.map(|x| Item {
					text: x.0.to_string(),
					on_select: x.1,
				})
				.collect(),
			width: Length::Shrink,
			padding: Renderer::DEFAULT_PADDING,
			text_size: None,
			font: Default::default(),
			style: Default::default(),
		}
	}

	pub fn width(mut self, width: Length) -> Self {
		self.width = width;
		self
	}

	pub fn padding(mut self, padding: u16) -> Self {
		self.padding = padding;
		self
	}

	pub fn text_size(mut self, size: u16) -> Self {
		self.text_size = Some(size);
		self
	}

	pub fn font(mut self, font: Renderer::Font) -> Self {
		self.font = font;
		self
	}

	pub fn style(mut self, style: impl Into<<Renderer as self::Renderer>::Style>) -> Self {
		self.style = style.into();
		self
	}
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for DropDown<'a, Message, Renderer>
where
	Message: 'static + Clone,
	Renderer: self::Renderer + 'a,
{
	fn width(&self) -> Length {
		Length::Shrink
	}

	fn height(&self) -> Length {
		Length::Shrink
	}

	fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
		use std::f32;

		let limits = limits.width(self.width).height(Length::Shrink).pad(f32::from(self.padding));

		let text_size = self.text_size.unwrap_or(renderer.default_size());

		let max_width = match self.width {
			Length::Shrink => {
				let (width, _) = renderer.measure(
					&self.label,
					text_size,
					Renderer::Font::default(),
					Size::new(f32::INFINITY, f32::INFINITY),
				);
				24
			}
			_ => 0,
		};

		let size = {
			let intrinsic = Size::new(max_width as f32 + f32::from(self.padding), f32::from(text_size));

			limits.resolve(intrinsic).pad(f32::from(self.padding))
		};

		layout::Node::new(size)
	}

	fn hash_layout(&self, state: &mut Hasher) {
		use std::hash::Hash as _;

		match self.width {
			Length::Shrink => {
				self.label.hash(state);
				self.items.iter().for_each(|item| item.text.hash(state));
			}
			_ => {
				self.width.hash(state);
			}
		}
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
				if self.state.is_open {
					// TODO: Encode cursor availability in the type system // ???
					self.state.is_open = cursor_position.x < 0.0 || cursor_position.y < 0.0;
				} else if layout.bounds().contains(cursor_position) {
					self.state.is_open = true;
					self.state.hovered_item = None;

					let labels = self.items.iter().map(|item| item.text.clone());
					self.state.items_width = labels
						.map(|label| {
							let (width, _) = renderer.measure(
								&label,
								self.text_size.unwrap_or(renderer.default_size()),
								Renderer::Font::default(),
								Size::new(f32::INFINITY, f32::INFINITY),
							);

							width.round() as u16
						})
						.max()
						.unwrap_or(100) + self.padding as u16 * 2;
				}

				if let Some(last_selection) = self.state.last_selection.take() {
					println!("sending a message!");

					messages.push(last_selection.on_select);

					self.state.is_open = false;
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
		self::Renderer::draw(
			renderer,
			layout.bounds(),
			cursor_position,
			self.label.clone(),
			self.padding,
			self.text_size.unwrap_or(renderer.default_size()),
			self.font,
			&self.style,
		)
	}

	fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
		if self.state.is_open {
			let bounds = layout.bounds();

			let mut menu = Menu::new(
				&mut self.state.menu,
				&self.items,
				&mut self.state.hovered_item,
				&mut self.state.last_selection,
			)
			.width(self.state.items_width)
			.padding(self.padding)
			.font(self.font)
			.style(Renderer::menu_style(&self.style));

			if let Some(text_size) = self.text_size {
				menu = menu.text_size(text_size);
			}

			Some(menu.overlay(layout.position(), bounds.height))
		} else {
			None
		}
	}
}

/// The renderer of a [`PickList`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`PickList`] in your user interface.
///
/// [`PickList`]: struct.PickList.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: text::Renderer + menu::Renderer {
	/// The default padding of a [`PickList`].
	///
	/// [`PickList`]: struct.PickList.html
	const DEFAULT_PADDING: u16;

	/// The [`PickList`] style supported by this renderer.
	///
	/// [`PickList`]: struct.PickList.html
	type Style: Default;

	/// Returns the style of the [`Menu`] of the [`PickList`].
	///
	/// [`Menu`]: ../../overlay/menu/struct.Menu.html
	/// [`PickList`]: struct.PickList.html
	fn menu_style(style: &<Self as Renderer>::Style) -> <Self as menu::Renderer>::Style;

	/// Draws a [`PickList`].
	///
	/// [`PickList`]: struct.PickList.html
	fn draw(
		&mut self,
		bounds: Rectangle,
		cursor_position: Point,
		label: String,
		padding: u16,
		text_size: u16,
		font: Self::Font,
		style: &<Self as Renderer>::Style,
	) -> Self::Output;
}

impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for DropDown<'a, Message, Renderer>
where
	Message: Clone + 'static,
	Renderer: self::Renderer + 'a,
{
	fn into(self) -> Element<'a, Message, Renderer> {
		Element::new(self)
	}
}
