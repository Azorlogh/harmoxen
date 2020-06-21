use super::Delegate;
use crate::commands as cmds;
use crate::state::{self, State};
use druid::{Command, DelegateCtx};

impl Delegate {
	pub fn handle_history(
		&mut self,
		_ctx: &mut DelegateCtx,
		cmd: &Command,
		data: &mut State,
		project_changed: &mut bool,
	) -> bool {
		match cmd {
			_ if cmd.is(cmds::HISTORY_SAVE) => {
				let project = state::Project::from_editors(&data.editors);
				data.history.borrow_mut().save(project);
				data.up_to_date = false;
				false
			}
			_ if cmd.is(cmds::HISTORY_UNDO) => {
				let project = data.history.borrow_mut().undo();
				project.open(&mut data.editors);
				*project_changed = true;
				false
			}
			_ if cmd.is(cmds::HISTORY_REDO) => {
				let project = data.history.borrow_mut().redo();
				project.open(&mut data.editors);
				*project_changed = true;
				false
			}
			_ => true,
		}
	}
}
