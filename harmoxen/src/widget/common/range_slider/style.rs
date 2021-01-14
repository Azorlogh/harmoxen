use iced::{Background, Color};

#[derive(Debug, Clone, Copy)]
pub struct Style {
	pub background: Background,
	pub border_radius: f32,
	pub border_width: f32,
	pub border_color: Color,
	pub bar_color: Color,
	pub bar_highlight: Color,
	pub bar_border_radius: f32,
	pub bar_border_width: f32,
	pub bar_border_color: Color,
	pub handle_color: Color,
	pub handle_highlight: Color,
}

impl std::default::Default for Style {
	fn default() -> Self {
		Style {
			background: Background::Color([0.87, 0.87, 0.87].into()),
			border_radius: 0.0,
			border_width: 1.0,
			border_color: [0.7, 0.7, 0.7].into(),
			bar_color: Color::from_rgb(0.5, 0.5, 0.5),
			bar_highlight: Color::from_rgb(0.6, 0.6, 0.6),
			bar_border_radius: 0.0,
			bar_border_width: 1.0,
			bar_border_color: [0.5, 0.5, 0.5].into(),
			handle_color: Color::from_rgb(0.5, 0.5, 0.5),
			handle_highlight: Color::from_rgb(0.6, 0.6, 0.6),
		}
	}
}

pub trait StyleSheet {
	fn active(&self) -> Style;
}

struct Default;

impl StyleSheet for Default {
	fn active(&self) -> Style {
		Style::default()
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
