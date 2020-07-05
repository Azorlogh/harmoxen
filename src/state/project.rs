use serde::{Deserialize, Serialize};

use crate::data::layout::Layout;
use crate::data::sheet::Sheet;
use crate::state::editors::State;
use generational_arena::Index;
use std::{cell::RefCell, collections::HashSet, rc::Rc};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Project {
	pub sheet: Sheet,
	pub layout: Layout,
	pub selection: HashSet<Index>,
	pub tempo: f64,
}

impl Project {
	pub fn from_editors(editors: &State) -> Project {
		let layout = (*editors.sheet_editor.layout.borrow()).clone();
		let sheet = (*editors.sheet_editor.sheet.borrow()).clone();
		let selection = (*editors.sheet_editor.selection.borrow()).clone();
		let tempo = editors.sheet_editor.tempo;
		Project {
			sheet,
			layout,
			selection,
			tempo,
		}
	}

	pub fn open(self, editors: &mut State) {
		editors.sheet_editor.layout = Rc::new(RefCell::new(self.layout));
		editors.sheet_editor.sheet = Rc::new(RefCell::new(self.sheet));
		editors.sheet_editor.selection = Rc::new(RefCell::new(self.selection));
		editors.sheet_editor.tempo = self.tempo;
	}
}
