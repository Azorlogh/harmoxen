use std::collections::VecDeque;

use super::project::Project;

pub const SIZE: usize = 8;

pub struct History {
	states: VecDeque<Project>,
	cursor: usize,
}

impl History {
	pub fn new(init: Project) -> History {
		let mut states = VecDeque::with_capacity(10);
		states.push_back(init);
		History { states, cursor: 0 }
	}

	pub fn save(&mut self, state: Project) {
		if self.cursor + 1 < SIZE {
			self.states.truncate(self.cursor + 1);
			self.states.push_back(state);
			self.cursor += 1;
		} else {
			self.states.pop_front();
			self.states.push_back(state);
		}
	}

	pub fn undo(&mut self) -> Project {
		if self.cursor > 0 {
			self.cursor -= 1;
		}
		self.states[self.cursor].clone()
	}

	pub fn redo(&mut self) -> Project {
		if self.cursor < self.states.len() {
			self.cursor += 1;
		}
		self.states[self.cursor].clone()
	}
}
