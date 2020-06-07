use druid::{Data, Lens};

pub mod layout_editor;
pub mod sheet_editor;

#[derive(Default, Clone, Data, Lens)]
pub struct State {
	pub sheet_editor: sheet_editor::State,
	pub layout_editor: layout_editor::State,
}

impl State {
	pub fn new() -> State {
		let mut state = State::default();
		state.apply_layout().unwrap();
		state
	}

	pub fn apply_layout(&mut self) -> Result<(), layout_editor::LayoutParseError> {
		let curr_marker = self.sheet_editor.curr_marker;
		let mut layout = self.sheet_editor.layout.borrow_mut();
		let pattern = layout_editor::make_pattern(&self.layout_editor)?;
		layout.set_marker_pattern(curr_marker, pattern);
		Ok(())
	}
}
