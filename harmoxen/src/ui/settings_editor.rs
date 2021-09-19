use crate::{
	state::{settings_editor::State, Message},
	widget::*,
	BackendId, Theme,
};
use iced::{Column, Element, Text};
use iced_audio::Knob;

pub fn build<'a>(state: &'a mut State, theme: Theme) -> Element<'a, Message> {
	let mut backend_settings = Row::new()
		.push(DropDown::new(
			&mut state.wstates.backend_dropdown,
			match state.backend_id {
				BackendId::Audio => "Audio",
				BackendId::Midi(_) => "Midi",
			},
			vec![
				("Audio", Message::ChangeBackend(BackendId::Audio)),
				("Midi", Message::ChangeBackend(BackendId::Midi(0))),
			],
		));
	
	if let BackendId::Midi(channel) = state.backend_id {
		// backend_settings.push(Knob::new(
		// 	&mut state.wstates.midi_channel_knob,
		// 	|value| Message::ChangeBackend(BackendId::Midi(value)),
		// ))
	}

	Column::new()
		.push(Text::new("SETTINGS"))
		.push(backend_settings)
		.into()
}
