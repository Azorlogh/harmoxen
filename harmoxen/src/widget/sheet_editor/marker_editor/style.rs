use iced::Color;
use iced_style::menu;

#[derive(Debug, Clone, Copy)]
pub struct Style {
	pub background: iced::Background,
}

impl std::default::Default for Style {
	fn default() -> Self {
		Style {
			background: Color::TRANSPARENT.into(),
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
