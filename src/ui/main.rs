use druid::{
	widget::{Button, Flex, Label},
	Widget, WidgetExt,
};

use crate::commands;
use crate::state::*;
use crate::widget::{common::*, *};

use super::sheet_editor;

pub fn build() -> impl Widget<State> {
	let menu = {
		Flex::row()
			.with_child(
				DropDown::new("File")
					.with_item(dropdown::Item::new("New", |ctx, _, _| {
						ctx.submit_command(commands::PROJECT_NEW);
					}))
					.with_item(dropdown::Item::new("Open", |ctx, _, _| {
						ctx.submit_command(commands::PROJECT_OPEN);
					}))
					.with_item(dropdown::Item::new("Save", |ctx, _, _| {
						ctx.submit_command(commands::PROJECT_SAVE)
					}))
					.with_item(dropdown::Item::new("Save As", |ctx, _, _| {
						ctx.submit_command(commands::PROJECT_SAVE_AS)
					}))
					.fix_width(80.0)
					.padding(3.0),
			)
			.with_child(
				Button::new("Settings")
					.on_click(|ctx, _, _| ctx.submit_command(commands::OPEN_SETTINGS))
					.fix_width(100.0)
					.padding(3.0),
			)
			.with_child(
				Button::new("Layout")
					.on_click(|ctx, _, _| ctx.submit_command(commands::OPEN_LAYOUT_EDITOR))
					.fix_width(80.0)
					.padding(3.0),
			)
			.with_flex_spacer(1.0)
			.with_child(Label::new("BPM:"))
			.with_child(
				ParseLazy::new(TextBox::new())
					.lens(editors::sheet_editor::State::tempo)
					.lens(editors::State::sheet_editor)
					.lens(State::editors)
					.padding(3.0),
			)
			.expand_width()
	};

	Stack::new()
		.with_child(Flex::column().with_child(menu.fix_height(40.0)).with_flex_child(
			sheet_editor::build().lens(editors::State::sheet_editor).lens(State::editors),
			1.0,
		))
		.with_child(Overlay::new())
}
