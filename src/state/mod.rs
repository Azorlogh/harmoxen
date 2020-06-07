use druid::{Data, Lens};
use std::{cell::RefCell, rc::Rc};

pub mod editors;
pub mod history;
pub use history::History;
pub mod project;
pub use project::Project;

#[derive(Clone, Data, Lens)]
pub struct State {
	pub main_window: Option<Rc<druid::WindowId>>,
	pub editors: editors::State,
	pub history: Rc<RefCell<history::History>>,
}

impl State {
	pub fn new() -> State {
		let editors = editors::State::new();
		let project = Project::from_editors(&editors);
		State {
			main_window: None,
			editors,
			history: Rc::new(RefCell::new(History::new(project))),
		}
	}
}
