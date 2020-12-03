use iced::{Background, Color};

#[derive(Debug, Clone, Copy)]
pub struct Style {
	pub background: Background,
	pub border_radius: u16,
	pub border_width: u16,
	pub border_color: Color,
	pub padding: f32,
	pub bar_color: Color,
	pub bar_border_radius: u16,
	pub bar_border_width: u16,
	pub bar_border_color: Color,
	pub handle_color: Color,
	pub handle_offset: f32,
}

impl std::default::Default for Style {
	fn default() -> Self {
		Style {
			background: Background::Color([0.87, 0.87, 0.87].into()),
			border_radius: 0,
			border_width: 1,
			border_color: [0.7, 0.7, 0.7].into(),
			padding: 8.0,
			bar_color: Color::from_rgb(0.6, 0.6, 0.6),
			bar_border_radius: 0,
			bar_border_width: 1,
			bar_border_color: [0.5, 0.5, 0.5].into(),
			handle_color: Color::from_rgb(0.5, 0.5, 0.5),
			handle_offset: 3.0,
		}
	}
}

pub trait StyleSheet {
	fn active(&self) -> Style;

	fn hovered(&self) -> Style;
}

struct Default;

impl StyleSheet for Default {
	fn active(&self) -> Style {
		Style::default()
	}

	fn hovered(&self) -> Style {
		Style {
			border_color: Color::BLACK,
			..self.active()
		}
	}
}

impl std::default::Default for Box<dyn StyleSheet> {
	fn default() -> Self {
		Box::new(Default)
	}
}

impl<T> From<T> for Box<dyn StyleSheet>
where
	T: 'static + StyleSheet,
{
	fn from(style: T) -> Self {
		Box::new(style)
	}
}
