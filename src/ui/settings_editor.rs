use druid::{
	lens::Map,
	widget::{Button, Controller, Flex, Label, ViewSwitcher, WidgetExt},
	Command, Env, LifeCycle, LifeCycleCtx, Widget,
};

use crate::commands as cmds;
use crate::state::editors::settings::{Backend, State};
use crate::widget::common::*;

struct RequestMPEPorts;

impl<T, W: Widget<T>> Controller<T, W> for RequestMPEPorts {
	fn lifecycle(&mut self, child: &mut W, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		if let LifeCycle::WidgetAdded = event {
			ctx.submit_command(Command::new(cmds::BACKEND_MPE_REQUEST_PORTS, ctx.widget_id()), None);
		}
		child.lifecycle(ctx, event, data, env);
	}
}

pub fn build() -> impl Widget<State> {
	let backend_input = Flex::row()
		.with_child(
			ValueSelector::new(vec![Backend::Audio, Backend::MPE { port: 0 }])
				.fix_width(100.0)
				.padding(10.0),
		)
		.with_flex_child(
			ViewSwitcher::new(
				|data: &Backend, _| std::mem::discriminant(data),
				|_, data, _| {
					Box::new(match data {
						Backend::Audio => Flex::row().with_child(Label::new("Use integrated synth")),
						Backend::MPE { .. } => Flex::row().with_child(
							IndexSelector::new(vec!["waiting for ports...".into()])
								.fix_width(300.0)
								.controller(RequestMPEPorts)
								.lens(enum_lens!(Backend::MPE, port)),
						),
					})
				},
			),
			1.0,
		)
		.lens(State::backend);

	Flex::column()
		.with_flex_spacer(1.0)
		.with_flex_child(backend_input, 1.0)
		.with_flex_spacer(1.0)
		.with_flex_child(
			Button::new("Apply").on_click(|ctx, _, _| ctx.submit_command(cmds::SETTINGS_APPLY, ctx.window_id())),
			1.0,
		)
}
