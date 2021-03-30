use crate::state::{CurrentEditor, State};
use crate::widget::*;
use crate::Message;
use iced::{Align, Container, Element, Length, Space, TextInput};
use iced_native::Widget;

mod layout_editor;
mod settings_editor;
mod sheet_editor;

pub fn build(state: &mut State) -> Element<Message> {
	let editor_ui = match state.current_editor {
		CurrentEditor::SheetEditor => sheet_editor::build(&mut state.sheet_editor, state.theme),
		CurrentEditor::LayoutEditor => layout_editor::build(&mut state.layout_editor, state.theme),
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
								("Save As", Message::ProjectSaveAs),
								("Save", Message::ProjectSave),
							],
						)
						.style(state.theme),
					)
					.push(
						Tab::new(state.current_editor == CurrentEditor::SheetEditor, Text::new("Sheet"))
							.on_press(Message::OpenSheet)
							.style(state.theme),
					)
					.push(
						Tab::new(state.current_editor == CurrentEditor::SettingsEditor, Text::new("Settings"))
							.on_press(Message::OpenSettings)
							.style(state.theme),
					)
					.push(Space::new(Length::Fill, Length::Shrink))
					.push({
						let theme = state.theme.clone();
						Container::new(Parse::new(
							&mut state.wstates.tempo_input,
							move |wstate, data| TextInput::new(wstate, "tempo", &data, |s| s).style(theme).padding(5),
							"120.0".to_string(),
							|s| s.parse::<f32>().ok().map(|tempo| Message::SetTempo(tempo)),
						))
						.height(Length::Shrink)
						.width(Length::Units(128))
					}),
			)
			.push(Space::new(Length::Fill, Length::Units(5)))
			.push(editor_ui)
			.into(),
		CurrentEditor::LayoutEditor => editor_ui,
	};

	Container::new(Stack::new().push(ui).push(Shortcuts))
		.style(state.theme)
		.width(Length::Fill)
		.height(Length::Fill)
		.into()
}
