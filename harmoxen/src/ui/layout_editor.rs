use crate::data::layout::{
	freq_input::{self, FreqInput},
	time_input::{self, TimeInput},
};
use crate::state::layout_editor::State;
use crate::state::{layout_editor::Message, Message as RootMessage};
use iced::{
	text_input, Button, Column, Container, Element, Length, PickList, Row, Text, TextInput,
};

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
						TimeInput::None => Into::<Element<RootMessage>>::into(Text::new(
							"The time axis will be free",
						)),
						TimeInput::Regular { ndiv, nbeats } => Row::new()
							.push(textbox(state0, "# divisions", &ndiv.to_string(), |text| {
								Message::SetTimeField(0, text).into()
							}))
							.push(textbox(state1, "# beats", &nbeats.to_string(), |text| {
								Message::SetTimeField(1, text).into()
							}))
							.into(),
						TimeInput::Formula {
							ndiv,
							nbeats,
							formula,
						} => Row::new()
							.push(
								textbox(state0, "# divisions", &ndiv.to_string(), |text| {
									Message::SetTimeField(0, text).into()
								})
								.padding(5),
							)
							.push(textbox(state1, "# beats", &nbeats.to_string(), |text| {
								Message::SetTimeField(1, text).into()
							}))
							.push(textbox(state2, "F: i -> x", &formula, |text| {
								Message::SetTimeField(2, text).into()
							}))
							.into(),
						TimeInput::Poly {
							ndiv0,
							ndiv1,
							nbeats,
						} => Row::new()
							.push(textbox(
								state0,
								"# divisions (a)",
								&ndiv0.to_string(),
								|text| Message::SetTimeField(0, text).into(),
							))
							.push(textbox(
								state1,
								"# divisions (b)",
								&ndiv1.to_string(),
								|text| Message::SetTimeField(1, text).into(),
							))
							.push(textbox(state2, "# beats", &nbeats.to_string(), |text| {
								Message::SetTimeField(2, text).into()
							}))
							.into(),
					}
				}),
		)
		.push(
			Button::new(&mut state.apply_btn_state, Text::new("Apply"))
				.on_press(RootMessage::ApplyLayout),
		);

	Row::new()
		.push(Container::new(editor).width(Length::Fill))
		.push(
			Button::new(&mut state.close_btn_state, Text::new("X"))
				.on_press(RootMessage::OpenSheet),
		)
		.padding(5)
		.into()
}
