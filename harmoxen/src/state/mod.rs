use crate::{backend, widget, Theme};
use iced::Command;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub mod layout_editor;
pub mod settings_editor;
pub mod sheet_editor;
// pub mod history;
// pub use history::History;
// pub mod project;
// pub use project::Project;

#[derive(Default)]
pub struct WStates {
	pub file_dropdown: widget::dropdown::State<Message>,
	pub settings_button: widget::button::State,
}

#[derive(PartialEq)]
pub enum CurrentEditor {
	SheetEditor,
	SettingsEditor,
	LayoutEditor,
}

pub struct State {
	pub wstates: WStates,
	pub sheet_editor: sheet_editor::State,
	pub layout_editor: layout_editor::State,
	pub settings_editor: settings_editor::State,
	// pub history: Rc<RefCell<history::History>>,
	pub current_editor: CurrentEditor,
	pub save_path: Option<PathBuf>,
	pub up_to_date: bool,
	pub theme: Theme,
	pub to_backend: Sender<backend::Event>,
}

impl State {
	pub fn new(to_backend: Sender<backend::Event>) -> State {
		State {
			wstates: Default::default(),
			sheet_editor: sheet_editor::State::default(),
			layout_editor: layout_editor::State::default(),
			settings_editor: settings_editor::State::default(),
			current_editor: CurrentEditor::SheetEditor,
			save_path: None,
			up_to_date: true,
			theme: Theme::Flux,
			to_backend,
		}
	}

	pub fn apply_layout(&mut self) -> Result<(), layout_editor::LayoutParseError> {
		let curr_marker = self.sheet_editor.curr_marker;
		let layout = &mut self.sheet_editor.layout;
		let pattern = layout_editor::make_pattern(&self.layout_editor)?;
		layout.set_marker_pattern(curr_marker, pattern);
		Ok(())
	}
}

impl State {
	pub fn update(self: &mut State, msg: Message) -> Command<Message> {
		match msg {
			Message::SheetEditor(msg) => {
				self.sheet_editor.update(msg, &mut self.to_backend);
			}
			Message::LayoutEditor(msg) => {
				self.layout_editor.update(msg);
			}
			Message::ProjectNew => {
				println!("new project");
			}
			Message::ProjectOpen => {
				println!("open project");
			}
			Message::ProjectSave => {
				println!("save project");
			}
			Message::OpenSheet => {
				self.current_editor = CurrentEditor::SheetEditor;
			}
			Message::OpenSettings => {
				self.current_editor = CurrentEditor::SettingsEditor;
			}
			Message::OpenLayout => {
				self.current_editor = CurrentEditor::LayoutEditor;
			}
			Message::Backend(evt) => {
				self.to_backend.send(evt).unwrap();
			}
			Message::ApplyLayout => {
				self.apply_layout().ok();
			}
		};
		Command::none()
	}
}

#[derive(Clone, Debug)]
pub enum Message {
	ProjectNew,
	ProjectOpen,
	ProjectSave,
	OpenSheet,
	OpenSettings,
	OpenLayout,
	ApplyLayout,
	SheetEditor(sheet_editor::Message),
	LayoutEditor(layout_editor::Message),
	// Icp(data::icp::Event),
	Backend(crate::backend::Event),
}
