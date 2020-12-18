use super::color;
use crate::widget::*;
use iced::Color;

const NORD_0: Color = color(0x2e3440);
const NORD_1: Color = color(0x3b4252);
const NORD_2: Color = color(0x434c5e);
const NORD_3: Color = color(0x4c566a);
const NORD_4: Color = color(0xd8dee9);
const NORD_5: Color = color(0xe5e9f0);
const NORD_6: Color = color(0xeceff4);
const NORD_7: Color = color(0x8fbcbb);
const NORD_8: Color = color(0x88c0d0);
const NORD_9: Color = color(0x81a1c1);
const NORD_10: Color = color(0x5e81ac);
const NORD_11: Color = color(0xbf616a);
const NORD_12: Color = color(0xd08770);
const NORD_13: Color = color(0xebcb8b);
const NORD_14: Color = color(0xa3be8c);
const NORD_15: Color = color(0xb48ead);

const SURFACE: Color = Color::from_rgb(0x40 as f32 / 255.0, 0x44 as f32 / 255.0, 0x4B as f32 / 255.0);

const ACCENT: Color = Color::from_rgb(0x6F as f32 / 255.0, 0xFF as f32 / 255.0, 0xE9 as f32 / 255.0);

const ACTIVE: Color = Color::from_rgb(0x72 as f32 / 255.0, 0x89 as f32 / 255.0, 0xDA as f32 / 255.0);

const HOVERED: Color = Color::from_rgb(0x67 as f32 / 255.0, 0x7B as f32 / 255.0, 0xC4 as f32 / 255.0);

pub struct Container;
impl container::StyleSheet for Container {
	fn style(&self) -> container::Style {
		container::Style {
			text_color: Some(NORD_6),
			background: NORD_0.into(),
			border_width: 1,
			border_color: NORD_1.into(),
			border_radius: 0,
		}
	}
}

pub struct Button;
impl button::StyleSheet for Button {
	fn active(&self) -> button::Style {
		button::Style {
			background: NORD_0.into(),
			border_radius: 3,
			border_width: 1,
			border_color: NORD_1,
			text_color: NORD_6,
			..button::Style::default()
		}
	}

	fn hovered(&self) -> button::Style {
		button::Style {
			background: NORD_2.into(),
			..self.active()
		}
	}

	fn pressed(&self) -> button::Style {
		button::Style {
			border_color: NORD_4.into(),
			..self.hovered()
		}
	}
}

pub struct PickList;
impl pick_list::StyleSheet for PickList {
	fn menu(&self) -> pick_list::Menu {
		pick_list::Menu {
			text_color: NORD_6,
			background: NORD_0.into(),
			border_width: 1,
			border_color: NORD_1.into(),
			selected_background: NORD_1.into(),
			selected_text_color: NORD_1.into(),
		}
	}

	fn active(&self) -> pick_list::Style {
		pick_list::Style {
			text_color: NORD_6,
			background: NORD_0.into(),
			border_width: 1,
			border_color: NORD_1.into(),
			border_radius: 0,
			icon_size: 1.0,
		}
	}

	fn hovered(&self) -> pick_list::Style {
		pick_list::Style {
			background: NORD_2.into(),
			..self.active()
		}
	}
}

pub struct RangeSlider;
impl range_slider::StyleSheet for RangeSlider {
	fn active(&self) -> range_slider::Style {
		range_slider::Style {
			background: NORD_0.into(),
			border_radius: 1,
			border_width: 1,
			border_color: NORD_2,
			bar_color: NORD_3,
			bar_highlight: NORD_4,
			bar_border_radius: 0,
			bar_border_width: 1,
			bar_border_color: NORD_1,
			handle_color: NORD_3,
			handle_highlight: NORD_2,
		}
	}
}

pub struct Tab;
impl tab::StyleSheet for Tab {
	fn active(&self) -> tab::Style {
		tab::Style {
			background: NORD_0.into(),
			border_radius: 1,
			border_width: 1,
			border_color: NORD_2,
		}
	}

	fn hovered(&self) -> tab::Style {
		tab::Style {
			background: NORD_1.into(),
			..self.active()
		}
	}

	fn selected(&self) -> tab::Style {
		tab::Style {
			background: NORD_1.into(),
			..self.active()
		}
	}
}

pub mod sheet_editor {
	use super::*;
	use crate::widget::sheet_editor::*;
	pub struct Board;
	impl board::StyleSheet for Board {
		fn active(&self) -> board::Style {
			board::Style {
				note_color: NORD_9,
				note_highlight: NORD_8,
				background_dark: NORD_0.into(),
				background_light: NORD_1.into(),
				root_line_color: NORD_8,
			}
		}
	}

	pub struct MarkerEditor;
	impl marker_editor::StyleSheet for MarkerEditor {
		fn menu(&self) -> pick_list::Menu {
			pick_list::Menu {
				text_color: NORD_6,
				background: NORD_0.into(),
				border_width: 1,
				border_color: NORD_1.into(),
				selected_background: NORD_1.into(),
				selected_text_color: NORD_1.into(),
			}
		}

		fn active(&self) -> marker_editor::Style {
			marker_editor::Style {}
		}
	}
}
