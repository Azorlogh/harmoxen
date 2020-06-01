use std::fmt::Display;
use std::mem;
use std::str::FromStr;

use druid::kurbo::Size;
use druid::{
	BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx, Widget, WidgetId,
};

/// Converts a `Widget<String>` to a `Widget<T>`, not modifying the data if the input is invalid
pub struct ParseLazy<W> {
	widget: W,
	state: String,
}

impl<W> ParseLazy<W> {
	pub fn new(widget: W) -> Self {
		Self {
			widget,
			state: String::new(),
		}
	}
}

impl<T: FromStr + Display + Data, W: Widget<String>> Widget<T> for ParseLazy<W> {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		self.widget.event(ctx, event, &mut self.state, env);
		if let Ok(res) = self.state.parse() {
			*data = res;
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		if let LifeCycle::WidgetAdded = event {
			self.state = data.to_string();
		}
		self.widget.lifecycle(ctx, event, &self.state, env)
	}

	fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
		let old = mem::replace(&mut self.state, data.to_string());
		self.widget.update(ctx, &old, &self.state, env)
	}

	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
		self.widget.layout(ctx, bc, &self.state, env)
	}

	fn paint(&mut self, paint: &mut PaintCtx, _data: &T, env: &Env) {
		self.widget.paint(paint, &self.state, env)
	}

	fn id(&self) -> Option<WidgetId> {
		self.widget.id()
	}
}
