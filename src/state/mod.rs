use druid::{Data, Lens};
use std::rc::Rc;

pub mod layout_input;
pub mod sheet_editor;

#[derive(Default, Clone, Data, Lens)]
pub struct State {
	pub main_window: Option<Rc<druid::WindowId>>,
	pub sheet_editor: sheet_editor::State,
	pub layout_input: layout_input::State,
}

pub fn apply_layout(data: &State) -> Result<(), layout_input::LayoutParseError> {
	let curr_marker = data.sheet_editor.curr_marker;
	let mut layout = data.sheet_editor.layout.borrow_mut();
	let pattern = layout_input::make_pattern(&data.layout_input)?;
	layout.set_marker_pattern(curr_marker, pattern);
	Ok(())
}
