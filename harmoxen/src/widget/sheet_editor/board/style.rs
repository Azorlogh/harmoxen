use iced::{Background, Color};
use iced_style::menu;

#[derive(Debug, Clone, Copy)]
pub struct Style {
	pub note_color: Color,
	pub note_highlight: Color,
	pub background_dark: Background,
	pub background_light: Background,
	pub root_line_color: Color,
}

impl std::default::Default for Style {
	fn default() -> Self {
		Style {
			note_color: Color::from_rgb(0.2, 0.6, 0.9),
			note_highlight: Color::from_rgb(0.3, 0.7, 1.0),
			background_dark: Color::from_rgb(0.1, 0.1, 0.1).into(),
			background_light: Color::from_rgb(0.2, 0.2, 0.2).into(),
			root_line_color: Color::from_rgb(0.1, 0.8, 0.7),
		}
	}
}

pub trait StyleSheet {
	fn menu(&self) -> menu::Style;

	fn active(&self) -> Style;
}

struct Default;

impl StyleSheet for Default {
	fn menu(&self) -> menu::Style {
		menu::Style::default()
	}

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
