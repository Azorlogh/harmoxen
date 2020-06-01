use druid::kurbo::{Point, Rect, Size};

use druid::{
	BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx, Widget, WidgetPod,
};

pub struct Stack<T: Data> {
	children: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
}

impl<T: Data> Stack<T> {
	pub fn new() -> Self {
		Stack { children: Vec::new() }
	}
	pub fn with_child(mut self, child: impl Widget<T> + 'static) -> Self {
		self.add_child(child);
		self
	}
	pub fn add_child(&mut self, child: impl Widget<T> + 'static) {
		self.children.push(WidgetPod::new(child).boxed());
	}
}

impl<T: Data> Widget<T> for Stack<T> {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		self.children.reverse();
		for child in &mut self.children {
			child.event(ctx, event, data, env);
			if ctx.is_handled() {
				break;
			}
		}
		self.children.reverse()
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		for child in &mut self.children {
			child.lifecycle(ctx, event, data, env);
		}
	}

	fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
		for child in &mut self.children {
			child.update(ctx, data, env);
		}
	}

	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		for child in &mut self.children {
			child.layout(ctx, bc, data, env);
			child.set_layout_rect(ctx, data, env, Rect::from_origin_size(Point::ORIGIN, bc.max()))
		}
		bc.max()
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
		for child in &mut self.children {
			child.paint(ctx, data, env);
		}
	}
}
