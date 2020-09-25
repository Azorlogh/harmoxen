use crate::util::{coord::Coord, Frame2};
use druid::{
	BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Selector, Size, UpdateCtx,
	Widget,
};

pub const SCROLL_VIEW_MOVE: Selector<Frame2> = Selector::new("scroll-view.move");

#[derive(Default)]
pub struct ScrollState {
	xmove: f64,
	ymove: f64,
	xzoom: f64,
	yzoom: f64,
}

pub struct ScrollView {
	scroll: ScrollState,
	mouse: Point,
}

impl ScrollView {
	pub fn new() -> ScrollView {
		ScrollView {
			scroll: ScrollState::default(),
			mouse: Point::ORIGIN,
		}
	}
}

impl Widget<Frame2> for ScrollView {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, frame: &mut Frame2, _env: &Env) {
		match event {
			Event::Wheel(mouse) if mouse.wheel_delta.y != 0.0 => {
				let delta = mouse.wheel_delta.y;
				let scroll = &mut self.scroll;
				let mouse_pos = &mut self.mouse;
				let coord = Coord::new(frame.clone(), ctx.size());
				*mouse_pos = coord.to_board_p(mouse.pos);
				if mouse.mods.alt() {
					scroll.xzoom += (delta / 120.0) * 0.1;
				} else if mouse.mods.ctrl() {
					scroll.yzoom += (delta / 120.0) * 0.1;
				} else if mouse.mods.shift() {
					let factor = frame.x.view.size();
					scroll.xmove += (-delta * factor * 0.002).max(-frame.x.view.0 - scroll.xmove);
				} else {
					let factor = frame.y.view.size();
					scroll.ymove += -delta * factor * 0.002;
				};
				ctx.request_anim_frame();
			}
			Event::Wheel(mouse) if mouse.wheel_delta.x != 0.0 => {
				let delta = mouse.wheel_delta.x;
				let scroll = &mut self.scroll;
				let factor = frame.x.view.size();
				scroll.xmove += (delta * factor * 0.002).max(-frame.x.view.0 - scroll.xmove);
				ctx.request_anim_frame();
			}
			Event::Command(ref cmd) if cmd.is(SCROLL_VIEW_MOVE) => {
				let new_frame = cmd.get_unchecked(SCROLL_VIEW_MOVE).clone();
				*frame = new_frame;
				ctx.request_layout();
				ctx.request_paint();
			}
			Event::AnimFrame(_) => {
				let scroll = &mut self.scroll;
				let mouse = self.mouse;
				let mut moving = false;
				let mut frame = frame.clone();
				if scroll.xmove.abs() > 1e-3 {
					frame.x.view += scroll.xmove * 0.2;
					scroll.xmove *= 0.8;
					moving = true;
				}
				if scroll.ymove.abs() > 1e-3 {
					frame.y.view += scroll.ymove * 0.2;
					scroll.ymove *= 0.8;
					moving = true;
				}
				if scroll.xzoom.abs() > 1e-3 {
					frame.x.view.scale_around(2f64.powf(scroll.xzoom * 0.2), mouse.x);
					frame.x.view -= frame.x.view.0.min(0.0);
					scroll.xzoom *= 0.8;
					moving = true;
				}
				if scroll.yzoom.abs() > 1e-3 {
					frame.y.view.scale_around(2f64.powf(scroll.yzoom * 0.2), mouse.y);
					frame.y.view -= frame.y.view.0.min(0.0);
					scroll.yzoom *= 0.8;
					moving = true;
				}
				if moving {
					ctx.request_anim_frame();
					ctx.submit_command(SCROLL_VIEW_MOVE.with(frame).to(ctx.widget_id()));
				}
				ctx.request_paint();
			}
			_ => {}
		}
	}

	fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &Frame2, _env: &Env) {}

	fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &Frame2, _data: &Frame2, _env: &Env) {}

	fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &Frame2, _env: &Env) -> Size {
		bc.max()
	}

	fn paint(&mut self, _ctx: &mut PaintCtx, _data: &Frame2, _env: &Env) {}
}
