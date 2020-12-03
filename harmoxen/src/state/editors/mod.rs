pub mod layout_editor;
// pub mod settings;
pub mod sheet_editor;

#[derive(Default)]
pub struct State {
	pub sheet_editor: sheet_editor::State,
	pub layout_editor: layout_editor::State,
	// pub settings: settings::State,
}

// use crate::commands as cmds;
// use druid::{Data, DelegateCtx, Lens};

impl State {
	pub fn new() -> State {
		let mut state = State::default();
		state.apply_layout().unwrap();
		state
	}

	pub fn apply_layout(&mut self) -> Result<(), layout_editor::LayoutParseError> {
		let curr_marker = self.sheet_editor.curr_marker;
		let layout = &mut self.sheet_editor.layout;
		let pattern = layout_editor::make_pattern(&self.layout_editor)?;
		layout.set_marker_pattern(curr_marker, pattern);
		Ok(())
	}

	// pub fn apply_settings(&mut self, ctx: &mut DelegateCtx) {
	// 	match self.settings.backend {
	// 		settings::Backend::Audio => {
	// 			ctx.submit_command(cmds::BACKEND_SET_AUDIO);
	// 		}
	// 		settings::Backend::MPE { port } => {
	// 			ctx.submit_command(cmds::BACKEND_SET_MPE.with(port));
	// 		}
	// 	}
	// }
}
