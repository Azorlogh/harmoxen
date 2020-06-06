use druid::{
	lens::Map,
	widget::{Button, Flex, Label, TextBox, ViewSwitcher, WidgetExt},
	Data, Lens, Widget,
};
use std::str::FromStr;

use crate::commands;
use crate::state::layout_input::{
	freq_input::{self, FreqInput},
	time_input::TimeInput,
	State,
};
use crate::widget::common::*;

macro_rules! enum_lens (
	($variant:path,$field:ident) => {
		Map::new(
			|data| match data {
				$variant{ $field, ..} => $field.clone(),
				_ => unreachable!(),
			},
			|data, new_data|
			if let $variant{ $field, .. } = data {
					*$field = new_data
			},
		)
	};
);

fn make_field<S: Data, F: Data + std::str::FromStr + std::fmt::Display>(name: &str, lens: impl Lens<S, F>) -> impl Widget<S> {
	Flex::row()
		.with_child(Label::new(format!("{}: ", name)))
		.with_child(ParseLazy::new(TextBox::new()))
		.padding(10.0)
		.lens(lens)
}

pub fn build() -> impl Widget<State> {
	let time_input = Flex::row()
		.with_child(
			Selector::new(vec![
				TimeInput::None,
				TimeInput::Regular { ndiv: 4, nbeats: 4 },
				TimeInput::Formula {
					ndiv: 4,
					nbeats: 4,
					formula: "i/4 + (i%2)*0.2".into(),
				},
				TimeInput::Poly {
					ndiv0: 4,
					ndiv1: 5,
					nbeats: 4,
				},
			])
			.fix_width(100.0)
			.padding(10.0),
		)
		.with_flex_child(
			ViewSwitcher::new(
				|data: &TimeInput, _| std::mem::discriminant(data),
				|_, data, _| {
					Box::new(match data {
						TimeInput::None => Flex::row().with_child(Label::new("The time axis will be free")),
						TimeInput::Regular { .. } => Flex::row()
							.with_child(make_field("#divisions", enum_lens!(TimeInput::Regular, ndiv)))
							.with_child(make_field("#repetitions", enum_lens!(TimeInput::Regular, nbeats))),
						TimeInput::Poly { .. } => Flex::row()
							.with_child(make_field("#divisions (a)", enum_lens!(TimeInput::Poly, ndiv0)))
							.with_child(make_field("#divisions (b)", enum_lens!(TimeInput::Poly, ndiv1)))
							.with_child(make_field("#repetitions", enum_lens!(TimeInput::Poly, nbeats))),
						TimeInput::Formula { .. } => Flex::row()
							.with_child(make_field("#divisions", enum_lens!(TimeInput::Formula, ndiv)))
							.with_child(make_field("#repetitions", enum_lens!(TimeInput::Formula, nbeats)))
							.with_child(make_field("#formula", enum_lens!(TimeInput::Formula, formula))),
					})
				},
			),
			1.0,
		)
		.lens(State::time);

	let freq_input = Flex::row()
		.with_child(
			Selector::new(vec![
				FreqInput::None,
				FreqInput::Equal {
					ndiv: 12,
					interval: 2.0,
					base: 440.0,
				},
				FreqInput::Enumeration {
					base: 440.0,
					enumeration: freq_input::Enumeration::from_str("38:40:43:46:48:51:54:57:61:64:68:72:76").unwrap(),
				},
				FreqInput::HarmonicSegment {
					base: 440.0,
					from: 8,
					to: 16,
				},
			])
			.fix_width(150.0)
			.padding(10.0),
		)
		.with_flex_child(
			ViewSwitcher::new(
				|data: &FreqInput, _| std::mem::discriminant(data),
				|_, data, _| {
					Box::new(match data {
						FreqInput::None => Flex::row().with_child(Label::new("The frequency axis will be free")),
						FreqInput::Equal { .. } => Flex::row()
							.with_child(make_field("base", enum_lens!(FreqInput::Equal, base)))
							.with_child(make_field("interval", enum_lens!(FreqInput::Equal, interval)))
							.with_child(make_field("#divisions", enum_lens!(FreqInput::Equal, ndiv))),
						FreqInput::Enumeration { .. } => Flex::row()
							.with_child(make_field("base", enum_lens!(FreqInput::Enumeration, base)))
							.with_child(make_field("enum", enum_lens!(FreqInput::Enumeration, enumeration))),
						FreqInput::HarmonicSegment { .. } => Flex::row()
							.with_child(make_field("base", enum_lens!(FreqInput::HarmonicSegment, base)))
							.with_child(make_field("from", enum_lens!(FreqInput::HarmonicSegment, from)))
							.with_child(make_field("to", enum_lens!(FreqInput::HarmonicSegment, to))),
					})
				},
			),
			1.0,
		)
		.lens(State::freq);

	Flex::column()
		.with_flex_spacer(1.0)
		.with_flex_child(time_input, 1.0)
		.with_flex_spacer(1.0)
		.with_flex_child(freq_input, 1.0)
		.with_flex_spacer(1.0)
		.with_flex_child(
			Button::new("Apply").on_click(|ctx, _, _| ctx.submit_command(commands::LAYOUT_APPLY, ctx.window_id())),
			1.0,
		)
}
