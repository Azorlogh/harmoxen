use super::color;
use crate::widget::*;
use iced::Color;

const BG_0: Color = color(0x21252B);
const BG_1: Color = color(0x282C34);
const BG_2: Color = color(0x373C47);
const BG_3: Color = color(0x414855);
const BG_4: Color = color(0x5D5D5D);
const BG_5: Color = color(0x707070);

const BG_FEATURE: Color = color(0x356160);

const FEATURE: Color = color(0x02BC71);
const FEATURE_HL: Color = color(0x69FFA2);

const FG: Color = color(0xEEEEEE);

pub struct Container;
impl container::StyleSheet for Container {
	fn style(&self) -> container::Style {
		container::Style {
			text_color: Some(FG),
			background: BG_0.into(),
			border_width: 0,
			border_color: Color::TRANSPARENT.into(),
			border_radius: 0,
		}
	}
}

pub struct Button;
impl button::StyleSheet for Button {
	fn active(&self) -> button::Style {
		button::Style {
			background: BG_1.into(),
			border_radius: 0,
			border_width: 0,
			border_color: BG_1,
			text_color: FG,
			..button::Style::default()
		}
	}

	fn hovered(&self) -> button::Style {
		button::Style {
			background: BG_1.into(),
			..self.active()
		}
	}

	fn pressed(&self) -> button::Style {
		button::Style {
			border_color: BG_1.into(),
			..self.hovered()
		}
	}
}

pub struct PickList;
impl pick_list::StyleSheet for PickList {
	fn menu(&self) -> pick_list::Menu {
		pick_list::Menu {
			text_color: FG,
			background: BG_0.into(),
			border_width: 0,
			border_color: BG_1.into(),
			selected_background: BG_3.into(),
			selected_text_color: FG.into(),
		}
	}

	fn active(&self) -> pick_list::Style {
		pick_list::Style {
			text_color: FG,
			background: BG_0.into(),
			border_width: 0,
			border_color: BG_1.into(),

			border_radius: 0,
			icon_size: 1.0,
		}
	}

	fn hovered(&self) -> pick_list::Style {
		pick_list::Style {
			background: BG_3.into(),
			..self.active()
		}
	}
}

pub struct RangeSlider;
impl range_slider::StyleSheet for RangeSlider {
	fn active(&self) -> range_slider::Style {
		range_slider::Style {
			background: BG_1.into(),
			border_radius: 0,
			border_width: 1,
			border_color: BG_2,
			bar_color: BG_2,
			bar_highlight: BG_3,
			bar_border_radius: 0,
			bar_border_width: 0,
			bar_border_color: BG_5,
			handle_color: BG_2,
			handle_highlight: BG_3,
		}
	}
}

pub struct Tab;
impl tab::StyleSheet for Tab {
	fn active(&self) -> tab::Style {
		tab::Style {
			background: BG_0.into(),
			border_radius: 0,
			border_width: 0,
			border_color: BG_1,
		}
	}

	fn hovered(&self) -> tab::Style {
		tab::Style {
			background: BG_3.into(),
			..self.active()
		}
	}

	fn selected(&self) -> tab::Style {
		tab::Style {
			background: BG_3.into(),
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
				note_color: FEATURE,
				note_highlight: FEATURE_HL,
				background_dark: BG_1.into(),
				background_light: BG_0.into(),
				root_line_color: BG_FEATURE,
			}
		}
	}

	pub struct MarkerEditor;
	impl marker_editor::StyleSheet for MarkerEditor {
		fn menu(&self) -> pick_list::Menu {
			pick_list::Menu {
				text_color: FG,
				background: BG_0.into(),
				border_width: 1,
				border_color: BG_1.into(),
				selected_background: BG_3.into(),
				selected_text_color: FG.into(),
			}
		}

		fn active(&self) -> marker_editor::Style {
			marker_editor::Style { background: BG_2.into() }
		}
	}

	pub struct Preview;
	impl preview::StyleSheet for Preview {
		fn active(&self) -> preview::Style {
			preview::Style { background: BG_1.into() }
		}
	}
}
