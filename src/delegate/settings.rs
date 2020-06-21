use super::Delegate;
use crate::commands as cmds;
use crate::server;
use crate::state::State;
use crate::ui;
use crate::widget::common::*;
use druid::{Command, DelegateCtx, LocalizedString, WindowDesc};

impl Delegate {
	pub fn handle_settings(
		&mut self,
		ctx: &mut DelegateCtx,
		cmd: &Command,
		data: &mut State,
		_project_changed: &mut bool,
	) -> bool {
		match cmd {
			_ if cmd.is(cmds::OPEN_SETTINGS) => {
				let new_win = WindowDesc::new(|| ui::build_settings())
					.title(LocalizedString::new("Settings"))
					.window_size((700.0, 200.0));
				ctx.new_window(new_win);
				false
			}
			_ if cmd.is(cmds::SETTINGS_APPLY) => {
				data.editors.apply_settings(ctx);
				false
			}
			_ if cmd.is(cmds::BACKEND_SET_AUDIO) => {
				self.to_server.send(server::Event::Shutdown).unwrap();
				std::thread::sleep(std::time::Duration::from_secs(1)); // not great, should handle server thread failures instead
				self.to_server = server::audio::launch().unwrap();
				false
			}
			_ if cmd.is(cmds::BACKEND_SET_MPE) => {
				self.to_server.send(server::Event::Shutdown).unwrap();
				std::thread::sleep(std::time::Duration::from_secs(1));
				let port = cmd.get_unchecked(cmds::BACKEND_SET_MPE);
				self.to_server = server::midi::launch(*port).unwrap();
				false
			}
			_ if cmd.is(cmds::BACKEND_MPE_REQUEST_PORTS) => {
				let target = *cmd.get_unchecked(cmds::BACKEND_MPE_REQUEST_PORTS);
				let ports = server::midi::get_port_names().unwrap();
				ctx.submit_command(Command::new(index_selector::SET_CHOICES, ports), Some(target.into()));
				false
			}
			_ => true,
		}
	}
}
