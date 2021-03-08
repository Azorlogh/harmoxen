use serde::{Deserialize, Serialize};

use crate::data::layout::Layout;
use crate::data::sheet::Sheet;
use crate::state::sheet_editor;
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
	pub fn from_state(sheet_editor: &sheet_editor::State) -> Project {
		let layout = sheet_editor.layout.clone();
		let sheet = sheet_editor.sheet.clone();
		let selection = sheet_editor.selection.clone();
		let tempo = sheet_editor.tempo;
		Project {
			sheet,
			layout,
			selection,
			tempo,
		}
	}

	pub fn open(self, sheet_editor: &mut sheet_editor::State) {
		sheet_editor.layout = self.layout;
		sheet_editor.sheet = self.sheet;
		sheet_editor.selection = self.selection;
		sheet_editor.tempo = self.tempo;
	}
}
