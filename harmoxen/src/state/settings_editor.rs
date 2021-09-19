use iced_audio::{IntRange, knob};

use crate::{widget::*, BackendId, Message, Theme};

pub struct WStates {
	pub backend_dropdown: dropdown::State<Message>,
	pub midi_channel_knob: knob::State,
}

impl Default for WStates {
	fn default() -> Self {
		Self {
			backend_dropdown: Default::default(),
			midi_channel_knob: knob::State::new(IntRange::new(0, 15).default_normal_param()),
		}
	}
}

#[derive(Default)]
pub struct State {
	pub wstates: WStates,
	pub backend_id: BackendId,
}
