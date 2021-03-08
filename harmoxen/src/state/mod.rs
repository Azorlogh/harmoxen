use crate::{backend, widget, Theme};
use iced::Command;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub mod history;
pub mod layout_editor;
pub mod project;
pub mod settings_editor;
pub mod sheet_editor;
pub use history::History;
pub use project::Project;

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
	pub current_editor: CurrentEditor,
	pub history: History,
	pub save_path: Option<PathBuf>,
	pub up_to_date: bool,
	pub theme: Theme,
	pub to_backend: Sender<backend::Event>,
}

impl State {
	pub fn new(to_backend: Sender<backend::Event>) -> State {
		let sheet_editor = sheet_editor::State::default();
		let project = Project::from_state(&sheet_editor);
		State {
			wstates: Default::default(),
			sheet_editor: sheet_editor::State::default(),
			layout_editor: layout_editor::State::default(),
			settings_editor: settings_editor::State::default(),
			current_editor: CurrentEditor::SheetEditor,
			history: History::new(project),
			save_path: None,
			up_to_date: true,
			theme: Theme::default(),
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
				self.sheet_editor = sheet_editor::State::default();
			}
			Message::ProjectOpen => {
				if let Some(path) = rfd::FileDialog::new().add_filter("hxp", &["hxp"]).pick_file() {
					println!("open location: {:?}", path);
					if let Ok(project_str) = std::fs::read_to_string(path) {
						let project = ron::from_str::<Project>(&project_str);
						if let Ok(project) = project {
							project.open(&mut self.sheet_editor);
						}
					}
				}
			}
			Message::ProjectSaveAs => {
				if let Some(path) = self.open_save_dialog() {
					self.save_to(&path);
				}
			}
			Message::ProjectSave => {
				if let Some(path) = self.save_path.clone().or_else(|| self.open_save_dialog()) {
					self.save_to(&path)
				}
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

	fn open_save_dialog(&self) -> Option<PathBuf> {
		rfd::FileDialog::new().add_filter("hxp", &["hxp"]).save_file()
	}

	fn save_to<P>(&self, path: &P)
	where
		P: AsRef<std::path::Path>,
	{
		let project = Project::from_state(&self.sheet_editor);
		let project_str = ron::to_string(&project).unwrap();
		std::fs::write(path, project_str).ok();
	}
}

#[derive(Clone, Debug)]
pub enum Message {
	ProjectNew,
	ProjectOpen,
	ProjectSave,
	ProjectSaveAs,
	OpenSheet,
	OpenSettings,
	OpenLayout,
	ApplyLayout,
	SheetEditor(sheet_editor::Message),
	LayoutEditor(layout_editor::Message),
	// Icp(data::icp::Event),
	Backend(crate::backend::Event),
}
