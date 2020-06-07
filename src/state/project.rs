use serde::{Deserialize, Serialize};

use crate::data::layout::Layout;
use crate::data::sheet::Sheet;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Project {
	pub sheet: Sheet,
	pub layout: Layout,
}

use crate::state::editors::State;
use std::{cell::RefCell, rc::Rc};

impl Project {
	pub fn from_editors(editors: &State) -> Project {
		let sheet = (*editors.sheet_editor.sheet.borrow()).clone();
		let layout = (*editors.sheet_editor.layout.borrow()).clone();
		Project { sheet, layout }
	}

	pub fn open(self, editors: &mut State) {
		editors.sheet_editor.sheet = Rc::new(RefCell::new(self.sheet));
		editors.sheet_editor.layout = Rc::new(RefCell::new(self.layout));
	}
}
