use druid::{
	BoxConstraints, Command, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, Size,
	UpdateCtx, Widget, WidgetPod,
};

use crate::commands;
use crate::util::{coord::Coord, Frame2};

#[derive(Default)]
pub struct ScrollState {
	xmove: f64,
	ymove: f64,
	xzoom: f64,
	yzoom: f64,
}

pub struct ScrollView<T, L> {
	child: WidgetPod<T, Box<dyn Widget<T>>>,
	frame_lens: L,
	scroll: ScrollState,
	mouse: Point,
}

impl<T, L> ScrollView<T, L> {
	pub fn new(child: impl Widget<T> + 'static, frame_lens: L) -> ScrollView<T, L> {
		ScrollView {
			child: WidgetPod::new(child).boxed(),
			frame_lens,
			scroll: ScrollState::default(),
			mouse: Point::ORIGIN,
		}
	}
}

impl<T, L> Widget<T> for ScrollView<T, L>
where
	T: Data,
	L: Lens<T, Frame2>,
{
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		match event {
			Event::Wheel(mouse) if mouse.wheel_delta.y != 0.0 => {
				let delta = mouse.wheel_delta.y;
				let scroll = &mut self.scroll;
				let mouse_pos = &mut self.mouse;
				self.frame_lens.with(data, |frame| {
					let coord = Coord::new(frame.clone(), ctx.size());
					*mouse_pos = coord.to_board_p(mouse.pos);
					if mouse.mods.alt {
						scroll.xzoom += (delta / 120.0) * 0.1;
					} else if mouse.mods.ctrl {
						scroll.yzoom += (delta / 120.0) * 0.1;
					} else if mouse.mods.shift {
						let factor = frame.x.view.size();
						scroll.xmove += (-delta * factor * 0.002).max(-frame.x.view.0 - scroll.xmove);
					} else {
						let factor = frame.y.view.size();
						scroll.ymove += -delta * factor * 0.002;
					}
				});
				ctx.request_anim_frame();
			}
			Event::Wheel(mouse) if mouse.wheel_delta.x != 0.0 => {
				let delta = mouse.wheel_delta.x;
				let scroll = &mut self.scroll;
				self.frame_lens.with(data, |frame| {
					let factor = frame.x.view.size();
					scroll.xmove += (delta * factor * 0.002).max(-frame.x.view.0 - scroll.xmove);
				});
				ctx.request_anim_frame();
			}
			Event::Command(ref cmd) if cmd.is(commands::SCROLL_VIEW_MOVE) => {
				let new_view = cmd.get_unchecked(commands::SCROLL_VIEW_MOVE).clone();
				self.frame_lens.with_mut(data, move |view| *view = new_view);
				ctx.request_layout();
				ctx.request_paint();
			}
			_ => {}
		}
		self.child.event(ctx, event, data, env);
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		match event {
			LifeCycle::AnimFrame(_) => {
				let scroll = &mut self.scroll;
				let mouse = self.mouse;
				self.frame_lens.with(data, |view| {
					let mut view = view.clone();
					let mut moving = false;
					if scroll.xmove.abs() > 1e-3 {
						view.x.view += scroll.xmove * 0.2;
						scroll.xmove *= 0.8;
						moving = true;
					}
					if scroll.ymove.abs() > 1e-3 {
						view.y.view += scroll.ymove * 0.2;
						scroll.ymove *= 0.8;
						moving = true;
					}
					if scroll.xzoom.abs() > 1e-3 {
						view.x.view.scale_around(2f64.powf(scroll.xzoom * 0.2), mouse.x);
						view.x.view -= view.x.view.0.min(0.0);
						scroll.xzoom *= 0.8;
						moving = true;
					}
					if scroll.yzoom.abs() > 1e-3 {
						view.y.view.scale_around(2f64.powf(scroll.yzoom * 0.2), mouse.y);
						view.y.view -= view.y.view.0.min(0.0);
						scroll.yzoom *= 0.8;
						moving = true;
					}
					if moving {
						ctx.request_anim_frame();
						ctx.submit_command(Command::new(commands::SCROLL_VIEW_MOVE, view), ctx.widget_id());
					}
				});
				ctx.request_paint();
			}
			_ => {}
		}
		self.child.lifecycle(ctx, event, data, env);
	}

	fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
		self.child.update(ctx, data, env);
	}

	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		let size = self.child.layout(ctx, bc, data, env);
		self.child
			.set_layout_rect(ctx, data, env, Rect::from_origin_size(Point::ORIGIN, size));
		size
	}

	fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
		self.child.paint(paint_ctx, data, env);
	}
}
