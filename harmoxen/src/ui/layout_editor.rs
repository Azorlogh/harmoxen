use crate::data::layout::{
	freq_input::{self, FreqInput},
	time_input::{self, TimeInput},
};
use crate::state::layout_editor::State;
use crate::state::{layout_editor::Message, Message as RootMessage};
use iced::{text_input, Button, Column, Container, Element, Length, PickList, Row, Text, TextInput};

fn textbox<'a, F>(
	state: &'a mut text_input::State,
	placeholder: &'static str,
	text: &str,
	on_change: F,
) -> TextInput<'a, RootMessage>
where
	F: Fn(String) -> RootMessage + 'static,
{
	TextInput::new(state, placeholder, text, on_change).padding(5)
}

pub fn build(state: &mut State) -> Element<RootMessage> {
	let editor = Column::new()
		.push(
			Row::new()
				.push(PickList::new(
					&mut state.time_pick_list,
					&[
						time_input::Mode::None,
						time_input::Mode::Regular,
						time_input::Mode::Formula,
						time_input::Mode::Poly,
					][..],
					Some(state.time.mode()),
					|mode| Message::SetTimeMode(mode).into(),
				))
				.push({
					let [state0, state1, state2] = &mut state.wstates_time;
					match &state.time {
						TimeInput::None => Into::<Element<RootMessage>>::into(Text::new("The time axis will be free")),
						TimeInput::Regular { ndiv, nbeats } => Row::new()
							.push(textbox(state0, "# divisions", &ndiv, |text| {
								Message::SetTimeField(0, text).into()
							}))
							.push(textbox(state1, "# beats", &nbeats, |text| {
								Message::SetTimeField(1, text).into()
							}))
							.into(),
						TimeInput::Formula { ndiv, nbeats, formula } => Row::new()
							.push(textbox(state0, "# divisions", &ndiv, |text| {
								Message::SetTimeField(0, text).into()
							}))
							.push(textbox(state1, "# beats", &nbeats, |text| {
								Message::SetTimeField(1, text).into()
							}))
							.push(textbox(state2, "F: i -> x", &formula, |text| {
								Message::SetTimeField(2, text).into()
							}))
							.into(),
						TimeInput::Poly { ndiv0, ndiv1, nbeats } => Row::new()
							.push(textbox(state0, "# divisions (a)", &ndiv0, |text| {
								Message::SetTimeField(0, text).into()
							}))
							.push(textbox(state1, "# divisions (b)", &ndiv1, |text| {
								Message::SetTimeField(1, text).into()
							}))
							.push(textbox(state2, "# beats", &nbeats, |text| {
								Message::SetTimeField(2, text).into()
							}))
							.into(),
					}
				}),
		)
		.push(
			Row::new()
				.push(PickList::new(
					&mut state.freq_pick_list,
					&[
						freq_input::Mode::None,
						freq_input::Mode::Equal,
						freq_input::Mode::Enumeration,
						freq_input::Mode::HarmonicSegment,
					][..],
					Some(state.freq.mode()),
					|mode| Message::SetFreqMode(mode).into(),
				))
				.push({
					let [state0, state1, state2] = &mut state.wstates_freq;
					match &state.freq {
						FreqInput::None => Into::<Element<RootMessage>>::into(Text::new("The frequency axis will be free")),
						FreqInput::Equal { base, interval, ndiv } => Row::new()
							.push(textbox(state0, "base frequency", &base, |text| {
								Message::SetFreqField(0, text).into()
							}))
							.push(textbox(state1, "interval", &interval, |text| {
								Message::SetFreqField(1, text).into()
							}))
							.push(textbox(state2, "# divisions", &ndiv, |text| {
								Message::SetFreqField(2, text).into()
							}))
							.into(),
						FreqInput::Enumeration { base, values } => Row::new()
							.push(textbox(state0, "base frequency", &base, |text| {
								Message::SetFreqField(0, text).into()
							}))
							.push(textbox(state1, "values", &values, |text| {
								Message::SetFreqField(1, text).into()
							}))
							.into(),
						FreqInput::HarmonicSegment { base, from, to } => Row::new()
							.push(textbox(state0, "base frequency", &base, |text| {
								Message::SetFreqField(0, text).into()
							}))
							.push(textbox(state1, "from", &from, |text| Message::SetFreqField(1, text).into()))
							.push(textbox(state2, "to", &to, |text| Message::SetFreqField(2, text).into()))
							.into(),
					}
				}),
		)
		.push(Button::new(&mut state.apply_btn_state, Text::new("Apply")).on_press(RootMessage::ApplyLayout));

	Row::new()
		.push(Container::new(editor).width(Length::Fill))
		.push(Button::new(&mut state.close_btn_state, Text::new("X")).on_press(RootMessage::OpenSheet))
		.padding(5)
		.into()
}
