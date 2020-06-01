use druid::widget::WidgetExt;
use druid::Widget;

use crate::state::State;

pub mod layout_editor;
pub mod sheet_editor;

pub fn build() -> impl Widget<State> {
	sheet_editor::build().lens(State::sheet_editor)
}

pub fn build_layout_editor() -> impl Widget<State> {
	layout_editor::build().lens(State::layout_input)
}
