use super::Delegate;
use crate::commands as cmds;
use crate::state::State;
use crate::ui;
use druid::{Command, DelegateCtx, LocalizedString, Target, WindowDesc};

impl Delegate {
	pub fn handle_layout(
		&mut self,
		ctx: &mut DelegateCtx,
		cmd: &Command,
		data: &mut State,
		_project_changed: &mut bool,
	) -> bool {
		match cmd {
			_ if cmd.is(cmds::OPEN_LAYOUT_EDITOR) => {
				let new_win = WindowDesc::new(|| ui::build_layout_editor())
					.title(LocalizedString::new("Layout input"))
					.window_size((800.0, 300.0));
				ctx.new_window(new_win);
				false
			}
			_ if cmd.is(cmds::LAYOUT_APPLY) => {
				if let Ok(()) = data.editors.apply_layout() {
					ctx.submit_command(cmds::LAYOUT_CHANGED.to(Target::Global));
				}
				false
			}
			_ => true,
		}
	}
}
