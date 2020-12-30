use crate::widget::*;

pub const fn color(code: u32) -> iced::Color {
	iced::Color {
		r: ((code >> 16) & 255) as f32 / 255.0,
		g: ((code >> 8) & 255) as f32 / 255.0,
		b: ((code >> 0) & 255) as f32 / 255.0,
		a: 1.0,
	}
}

// mod flux;
// mod nord;
mod one_dark;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
	// Flux,
	// Nord,
	OneDark,
}
use Theme::*;

macro_rules! impl_style {
	($widget:ident, $style:ident) => {
		impl From<Theme> for Box<dyn $widget::StyleSheet> {
			fn from(theme: Theme) -> Self {
				match theme {
					// Flux => flux::$style.into(),
					// Nord => nord::$style.into(),
					OneDark => one_dark::$style.into(),
				}
			}
		}
	};
	($widget:ident, $prefix:ident, $style:ident) => {
		impl From<Theme> for Box<dyn $widget::StyleSheet> {
			fn from(theme: Theme) -> Self {
				match theme {
					// Flux => flux::$prefix::$style.into(),
					// Nord => nord::$prefix::$style.into(),
					OneDark => one_dark::$prefix::$style.into(),
				}
			}
		}
	};
}

impl Default for Theme {
	fn default() -> Theme {
		OneDark
	}
}

impl_style!(container, Container);
impl_style!(button, Button);
impl_style!(pick_list, PickList);
impl_style!(range_slider, RangeSlider);
impl_style!(tab, Tab);

mod sheet_editor {
	use super::{
		// flux,
		// nord,
		one_dark,
		Theme::{self, *},
	};
	use crate::widget::sheet_editor::*;

	impl_style!(board, sheet_editor, Board);
	impl_style!(marker_editor, sheet_editor, MarkerEditor);
	impl_style!(preview, sheet_editor, Preview);
}
