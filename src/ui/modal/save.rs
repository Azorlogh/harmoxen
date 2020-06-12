use druid::{
	theme,
	widget::{Button, Flex, Label},
	BoxConstraints, Command, Env, Selector, Widget, WidgetExt,
};

use crate::commands;
use crate::state::State;
use crate::widget::*;

pub fn build(cmd: Selector) -> (BoxConstraints, Box<dyn Fn(&Env) -> Box<dyn Widget<State>>>) {
	(
		BoxConstraints::tight((300.0, 80.0).into()),
		Box::new(move |env| {
			Box::new(
				Flex::column()
					.with_flex_child(Label::new("Save changes to this project ?"), 1.0)
					.with_child(
						Flex::row()
							.with_flex_child(
								Button::new("Yes").on_click(|ctx, _, _| {
									ctx.submit_command(overlay::HIDE, ctx.window_id());
									ctx.submit_command(commands::PROJECT_SAVE, ctx.window_id())
								}),
								1.0,
							)
							.with_flex_child(
								Button::new("No").on_click(move |ctx, _, _| {
									ctx.submit_command(overlay::HIDE, ctx.window_id());
									ctx.submit_command(cmd, ctx.window_id());
								}),
								1.0,
							)
							.with_flex_child(
								Button::new("Cancel").on_click(|ctx, _, _| ctx.submit_command(overlay::HIDE, ctx.window_id())),
								1.0,
							),
					)
					.background(env.get(theme::BACKGROUND_LIGHT)),
			)
		}),
	)
}
