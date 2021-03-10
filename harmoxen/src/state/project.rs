use serde::{Deserialize, Serialize};

use crate::data::layout::Layout;
use crate::data::sheet::Sheet;
use crate::state::{sheet_editor, State};
use generational_arena::Index;
use std::collections::HashSet;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Project {
	pub sheet: Sheet,
	pub layout: Layout,
	pub selection: HashSet<Index>,
	pub tempo: f32,
}

impl Project {
	pub fn from_state(sheet_editor: &sheet_editor::State, tempo: f32) -> Project {
		let layout = sheet_editor.layout.clone();
		let sheet = sheet_editor.sheet.clone();
		let selection = sheet_editor.selection.clone();
		let tempo = tempo;
		Project {
			sheet,
			layout,
			selection,
			tempo,
		}
	}

	pub fn open(self, state: &mut State) {
		state.sheet_editor.layout = self.layout;
		state.sheet_editor.sheet = self.sheet;
		state.sheet_editor.selection = self.selection;
		state.tempo = self.tempo;
	}
}
