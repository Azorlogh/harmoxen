use crate::{backend, widget, Theme};
use iced::{text_input, Command};
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
	pub tempo_input: widget::parse::State<widget::text_input::State, String>,
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
	pub tempo: f32,
	pub history: History,
	pub save_path: Option<PathBuf>,
	pub up_to_date: bool,
	pub theme: Theme,
	pub to_server: Sender<crate::Event>,
}

impl State {
	pub fn new(to_server: Sender<crate::Event>) -> State {
		let sheet_editor = sheet_editor::State::default();
		let tempo = 120.0;
		let project = Project::from_state(&sheet_editor, tempo);
		State {
			wstates: Default::default(),
			sheet_editor: sheet_editor::State::default(),
			layout_editor: layout_editor::State::default(),
			settings_editor: settings_editor::State::default(),
			current_editor: CurrentEditor::SheetEditor,
			tempo,
			history: History::new(project),
			save_path: None,
			up_to_date: true,
			theme: Theme::default(),
			to_server,
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

pub struct UpdateCtx<'a> {
	to_server: &'a mut Sender<crate::Event>,
	tempo: f32,
	project_changed: &'a mut bool,
}
impl<'a> UpdateCtx<'a> {
	fn to_backend(&mut self, evt: backend::Event) {
		self.to_server.send(crate::Event::ToBackend(evt)).ok();
	}
	fn project_changed(&mut self) {
		*self.project_changed = true;
	}
}

impl State {
	pub fn update(&mut self, msg: Message) -> Command<Message> {
		let mut project_changed = false;
		match msg {
			Message::SheetEditor(msg) => {
				let ctx = UpdateCtx {
					to_server: &mut self.to_server,
					tempo: self.tempo,
					project_changed: &mut project_changed,
				};
				self.sheet_editor.update(msg, ctx);
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
							project.open(self);
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
				self.to_server.send(crate::Event::ToBackend(evt)).unwrap();
			}
			Message::ChangeBackend(backend) => {
				self.to_server.send(crate::Event::ChangeBackend(backend));
			}
			Message::ApplyLayout => {
				self.apply_layout().ok();
			}
			Message::SetTempo(tempo) => {
				self.tempo = tempo;
				project_changed = true
			}
			Message::Undo => {
				if let Some(project) = self.history.undo() {
					project.open(self);
					self.up_to_date = false;
				}
			}
			Message::Redo => {
				if let Some(project) = self.history.redo() {
					project.open(self);
					self.up_to_date = false;
				}
			}
		};

		if project_changed {
			let project = Project::from_state(&self.sheet_editor, self.tempo);
			self.history.save(project);
		}

		Command::none()
	}

	fn open_save_dialog(&self) -> Option<PathBuf> {
		rfd::FileDialog::new().add_filter("hxp", &["hxp"]).save_file()
	}

	fn save_to<P>(&self, path: &P)
	where
		P: AsRef<std::path::Path>,
	{
		let project = Project::from_state(&self.sheet_editor, self.tempo);
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
	Undo,
	Redo,
	ApplyLayout,
	SheetEditor(sheet_editor::Message),
	LayoutEditor(layout_editor::Message),
	Backend(crate::backend::Event),
	ChangeBackend(crate::Backend),
	SetTempo(f32),
}
