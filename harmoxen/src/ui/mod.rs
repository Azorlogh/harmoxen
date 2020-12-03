use crate::state::{CurrentEditor, State};
use crate::widget::*;
use crate::Message;
use iced::{Align, Container, Element, Length};

mod layout_editor;
mod settings_editor;
mod sheet_editor;

pub fn build(state: &mut State) -> Element<Message> {
	let editor_ui = match state.current_editor {
		CurrentEditor::SheetEditor => sheet_editor::build(&mut state.sheet_editor, state.theme),
		CurrentEditor::LayoutEditor => layout_editor::build(&mut state.layout_editor),
		CurrentEditor::SettingsEditor => settings_editor::build(&mut state.settings_editor),
	};

	let ui = match state.current_editor {
		CurrentEditor::SheetEditor | CurrentEditor::SettingsEditor => Column::new()
			.align_items(Align::Start)
			.push(
				Row::new()
					.align_items(Align::Center)
					.push(
						DropDown::new(
							&mut state.wstates.file_dropdown,
							"File",
							vec![
								("New", Message::ProjectNew),
								("Open", Message::ProjectOpen),
								("Save", Message::ProjectSave),
							],
						)
						.style(state.theme),
					)
					.push(
						Tab::new(
							state.current_editor == CurrentEditor::SheetEditor,
							Text::new("Sheet"),
						)
						.on_press(Message::OpenSheet)
						.style(state.theme),
					)
					.push(
						Tab::new(
							state.current_editor == CurrentEditor::SettingsEditor,
							Text::new("Settings"),
						)
						.on_press(Message::OpenSettings)
						.style(state.theme),
					),
			)
			// .push(Space::with_height(Length::Units(4)))
			.push(editor_ui)
			.into(),
		CurrentEditor::LayoutEditor => editor_ui,
	};

	Container::new(ui)
		.style(state.theme)
		.width(Length::Fill)
		.height(Length::Fill)
		.into()
}
