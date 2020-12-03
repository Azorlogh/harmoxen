use crate::widget::*;

pub const fn color(code: u32) -> iced::Color {
	iced::Color {
		r: ((code >> 16) & 255) as f32 / 255.0,
		g: ((code >> 8) & 255) as f32 / 255.0,
		b: ((code >> 0) & 255) as f32 / 255.0,
		a: 1.0,
	}
}

mod flux;
mod nord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
	Flux,
	Nord,
}
use Theme::*;

impl Default for Theme {
	fn default() -> Theme {
		Flux
	}
}

impl From<Theme> for Box<dyn container::StyleSheet> {
	fn from(theme: Theme) -> Self {
		match theme {
			Flux => flux::Container.into(),
			Nord => nord::Container.into(),
		}
	}
}

impl From<Theme> for Box<dyn button::StyleSheet> {
	fn from(theme: Theme) -> Self {
		match theme {
			Flux => flux::Button.into(),
			Nord => nord::Button.into(),
		}
	}
}

impl From<Theme> for Box<dyn pick_list::StyleSheet> {
	fn from(theme: Theme) -> Self {
		match theme {
			Flux => flux::PickList.into(),
			Nord => nord::PickList.into(),
		}
	}
}

impl From<Theme> for Box<dyn range_slider::StyleSheet> {
	fn from(theme: Theme) -> Self {
		match theme {
			Flux => flux::RangeSlider.into(),
			Nord => nord::RangeSlider.into(),
		}
	}
}

impl From<Theme> for Box<dyn tab::StyleSheet> {
	fn from(theme: Theme) -> Self {
		match theme {
			Flux => flux::Tab.into(),
			Nord => nord::Tab.into(),
		}
	}
}

mod sheet_editor {
	use super::{
		flux, nord,
		Theme::{self, *},
	};
	use crate::widget::sheet_editor::board;

	impl From<Theme> for Box<dyn board::StyleSheet> {
		fn from(theme: Theme) -> Self {
			match theme {
				Flux => flux::sheet_editor::Board.into(),
				Nord => nord::sheet_editor::Board.into(),
			}
		}
	}
}
