use druid::{
	BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, MouseEvent, PaintCtx, Point, Rect,
	RenderContext, Size, UpdateCtx, Widget, WidgetPod,
};

pub struct Reversed<T: Data> {
	child: WidgetPod<T, Box<dyn Widget<T>>>,
	axes: (bool, bool),
}

impl<T: Data> Reversed<T> {
	pub fn new(child: impl Widget<T> + 'static, axes: (bool, bool)) -> Reversed<T> {
		Reversed {
			child: WidgetPod::new(child).boxed(),
			axes,
		}
	}
}

impl<T: Data> Widget<T> for Reversed<T> {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		let axes = self.axes;
		let size = ctx.size();
		let event = match event.clone() {
			Event::Wheel(e) => Event::Wheel(mouse_event(e, size, axes)),
			Event::MouseDown(e) => Event::MouseDown(mouse_event(e, size, axes)),
			Event::MouseMove(e) => Event::MouseMove(mouse_event(e, size, axes)),
			Event::MouseUp(e) => Event::MouseUp(mouse_event(e, size, axes)),
			other => other,
		};
		self.child.event(ctx, &event, data, env);
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		self.child.lifecycle(ctx, event, data, env);
	}

	fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
		self.child.update(ctx, data, env);
	}

	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		let size = self.child.layout(ctx, bc, data, env);
		// let origin = Point::new(
		// 	if self.axes.0 { child_size.width } else { 0.0 },
		// 	if self.axes.1 { child_size.height } else { 0.0 },
		// );
		// let size = Size::new(
		// 	if self.axes.0 { -child_size.width } else { child_size.width },
		// 	if self.axes.1 { -child_size.height } else { child_size.height },
		// );
		self.child
			.set_layout_rect(ctx, data, env, Rect::from_origin_size(Point::ORIGIN, size));
		size
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
		let transform = druid::Affine::translate((0.0, ctx.size().height)) * druid::Affine::FLIP_Y;
		// ctx.render_ctx.transform(transform);
		ctx.with_save(|ctx| {
			ctx.transform(transform);
			// let visible = Vec2::new(0.0, size.height) + ctx.region().to_rect() * Vec2::new(1.0, -1.0);
			// let visible = ctx.region().to_rect();
			// let visible = Rect::from_points((visible.x0, visible.y1), (visible.x1, visible.y0));
			// ctx.with_child_ctx(visible, |ctx| self.child.paint(ctx, data, env));
			ctx.with_child_ctx(ctx.region().clone(), |ctx| self.child.paint(ctx, data, env));
		})
	}
}

fn mouse_event(mut e: MouseEvent, size: Size, axes: (bool, bool)) -> MouseEvent {
	if axes.0 {
		e.pos.x *= -1.0;
		e.pos.x += size.width;
		e.wheel_delta.x *= -1.0;
	}
	if axes.1 {
		e.pos.y *= -1.0;
		e.pos.y += size.height;
		e.wheel_delta.y *= -1.0;
	}
	e
}
