pub use crate::data::layout::LayoutParseError;
use crate::data::layout::{
	freq_input::{self, FreqInput},
	time_input::{self, TimeInput},
};
use crate::state::Message as RootMessage;
use iced::{button, text_input, Command};

use crate::data::layout::Pattern;

#[derive(Clone, Default)]
pub struct State {
	pub close_btn_state: button::State,
	pub apply_btn_state: button::State,
	pub time_pick_list: iced::pick_list::State<time_input::Mode>,
	pub wstates_time: [text_input::State; 3],
	pub time: TimeInput,
	pub freq_pick_list: iced::pick_list::State<freq_input::Mode>,
	pub wstates_freq: [text_input::State; 3],
	pub freq: FreqInput,
}

impl State {
	pub fn update(&mut self, msg: Message) -> Command<Message> {
		match msg {
			Message::SetTimeMode(input) => {
				self.time = match input {
					time_input::Mode::None => TimeInput::default_none(),
					time_input::Mode::Regular => TimeInput::default_regular(),
					time_input::Mode::Poly => TimeInput::default_poly(),
					time_input::Mode::Formula => TimeInput::default_formula(),
				}
			}
			Message::SetTimeField(idx, text) => match &mut self.time {
				// TODO: improve when RFC 2294 lands
				TimeInput::None => {}
				TimeInput::Regular { ndiv, nbeats } => match idx {
					0 => *ndiv = text,
					1 => *nbeats = text,
					_ => {}
				},
				TimeInput::Formula { ndiv, nbeats, formula } => match idx {
					0 => *ndiv = text,
					1 => *nbeats = text,
					2 => *formula = text,
					_ => {}
				},
				TimeInput::Poly { ndiv0, ndiv1, nbeats } => match idx {
					0 => *ndiv0 = text,
					1 => *ndiv1 = text,
					2 => *nbeats = text,
					_ => {}
				},
			},
			Message::SetFreqMode(input) => {
				self.freq = match input {
					freq_input::Mode::None => FreqInput::default_none(),
					freq_input::Mode::Equal => FreqInput::default_equal(),
					freq_input::Mode::Enumeration => FreqInput::default_enumeration(),
					freq_input::Mode::HarmonicSegment => FreqInput::default_harmonic_segment(),
				}
			}
			Message::SetFreqField(idx, text) => match &mut self.freq {
				// TODO: improve when RFC 2294 lands
				FreqInput::None => {}
				FreqInput::Equal { base, interval, ndiv } => match idx {
					0 => *base = text,
					1 => *interval = text,
					2 => *ndiv = text,
					_ => {}
				},
				FreqInput::Enumeration { base, values } => match idx {
					0 => *base = text,
					1 => *values = text,
					_ => {}
				},
				FreqInput::HarmonicSegment { base, from, to } => match idx {
					0 => *base = text,
					1 => *from = text,
					2 => *to = text,
					_ => {}
				},
			},
		}
		Command::none()
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	SetTimeMode(time_input::Mode),
	SetTimeField(usize, String),
	SetFreqMode(freq_input::Mode),
	SetFreqField(usize, String),
}

impl From<Message> for RootMessage {
	fn from(msg: Message) -> RootMessage {
		RootMessage::LayoutEditor(msg)
	}
}

pub fn make_pattern(input: &State) -> Result<Pattern, LayoutParseError> {
	Ok(Pattern {
		time: input.time.build()?,
		freq: input.freq.build()?,
	})
}
