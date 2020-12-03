pub use crate::data::layout::LayoutParseError;
use crate::data::layout::{
	freq_input::{self, FreqInput},
	time_input::{self, TimeInput},
};
use crate::state::Message as RootMessage;
use derive_more::Display;
use iced::{button, text_input, Command};
use std::error::Error;

use crate::data::layout::Pattern;

#[derive(Clone, Default)]
pub struct State {
	pub close_btn_state: button::State,
	pub apply_btn_state: button::State,
	pub time_pick_list: iced::pick_list::State<time_input::Mode>,
	pub wstates_time: [text_input::State; 3],
	pub time: TimeInput,
	pub freq: FreqInput,
}

impl State {
	pub fn update(&mut self, msg: Message) -> Command<Message> {
		match msg {
			Message::SetTimeMode(input) => {
				self.time = match input {
					time_input::Mode::None => TimeInput::None,
					time_input::Mode::Regular => {
						time_input::TimeInput::Regular { ndiv: 4, nbeats: 4 }
					}
					time_input::Mode::Formula => time_input::TimeInput::Formula {
						ndiv: 4,
						nbeats: 4,
						formula: "i/4 + (i%2)*0.2".into(),
					},
					time_input::Mode::Poly => time_input::TimeInput::Poly {
						ndiv0: 4,
						ndiv1: 5,
						nbeats: 4,
					},
				}
			}
			Message::SetTimeField(idx, text) => match &mut self.time {
				// TODO: improve when RFC 2294 lands
				TimeInput::None => {}
				TimeInput::Regular { ndiv, nbeats } => match idx {
					0 => *ndiv = text.parse::<usize>().unwrap_or(*ndiv),
					1 => *nbeats = text.parse::<usize>().unwrap_or(*nbeats),
					_ => {}
				},
				TimeInput::Formula {
					ndiv,
					nbeats,
					formula,
				} => match idx {
					0 => *ndiv = text.parse::<usize>().unwrap_or(*ndiv),
					1 => *nbeats = text.parse::<usize>().unwrap_or(*nbeats),
					2 => *formula = text,
					_ => {}
				},
				TimeInput::Poly {
					ndiv0,
					ndiv1,
					nbeats,
				} => match idx {
					0 => *ndiv0 = text.parse::<usize>().unwrap_or(*ndiv0),
					1 => *ndiv1 = text.parse::<usize>().unwrap_or(*ndiv1),
					2 => *nbeats = text.parse::<usize>().unwrap_or(*nbeats),
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
