use iced::{Background, Color};

#[derive(Debug, Clone, Copy)]
pub struct Style {
	pub background: Background,
	pub border_radius: f32,
	pub border_width: f32,
	pub border_color: Color,
}

impl std::default::Default for Style {
	fn default() -> Self {
		Style {
			background: Background::Color([0.87, 0.87, 0.87].into()),
			border_radius: 0.0,
			border_width: 1.0,
			border_color: [0.7, 0.7, 0.7].into(),
		}
	}
}

pub trait StyleSheet {
	fn active(&self) -> Style;

	fn hovered(&self) -> Style;

	fn selected(&self) -> Style;
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

	fn selected(&self) -> Style {
		Style {
			background: Color::BLACK.into(),
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
