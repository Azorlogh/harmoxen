//! Display a dropdown list of selectable values.
use iced_graphics::backend::{self, Backend};
use iced_graphics::{Primitive, Renderer};
use iced_native::{mouse, Font, HorizontalAlignment, Point, Rectangle, VerticalAlignment};
use iced_style::menu;

pub use iced_native::pick_list::State;
pub use iced_style::pick_list::{Style, StyleSheet};

/// A widget allowing the selection of a single value from a list of options.
pub type DropDown<'a, Message, Backend> = super::DropDown<'a, Message, Renderer<Backend>>;

impl<B> super::Renderer for Renderer<B>
where
	B: Backend + backend::Text,
{
	type Style = Box<dyn StyleSheet>;

	const DEFAULT_PADDING: u16 = 5;

	fn menu_style(style: &Box<dyn StyleSheet>) -> menu::Style {
		style.menu()
	}

	fn draw(
		&mut self,
		bounds: Rectangle,
		cursor_position: Point,
		label: String,
		padding: u16,
		text_size: u16,
		font: Font,
		style: &Box<dyn StyleSheet>,
	) -> Self::Output {
		let is_mouse_over = bounds.contains(cursor_position);

		let style = if is_mouse_over {
			style.hovered()
		} else {
			style.active()
		};

		let background = Primitive::Quad {
			bounds,
			background: style.background,
			border_color: style.border_color,
			border_width: style.border_width,
			border_radius: style.border_radius,
		};

		(
			Primitive::Group {
				primitives: {
					let label = Primitive::Text {
						content: label,
						size: f32::from(text_size),
						font,
						color: style.text_color,
						bounds: Rectangle {
							x: bounds.x + f32::from(padding),
							y: bounds.center_y(),
							..bounds
						},
						horizontal_alignment: HorizontalAlignment::Left,
						vertical_alignment: VerticalAlignment::Center,
					};

					vec![background, label]
				},
			},
			if is_mouse_over {
				mouse::Interaction::Pointer
			} else {
				mouse::Interaction::default()
			},
		)
	}
}
