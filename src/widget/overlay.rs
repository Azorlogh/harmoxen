use crate::state::State;
use druid::{
	BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Point, Rect, Selector, Size, UpdateCtx,
	Widget, WidgetPod,
};

type ChildBuilder = Box<dyn Fn(&Env) -> Box<dyn Widget<State>>>;

pub const SHOW_AT: Selector<(Point, BoxConstraints, ChildBuilder)> = Selector::new("dropdown.show-at");
pub const SHOW_MIDDLE: Selector<(BoxConstraints, ChildBuilder)> = Selector::new("dropdown.show-middle");
pub const HIDE: Selector = Selector::new("dropdown.hide");

pub struct Child {
	origin: Option<Point>,
	bc: BoxConstraints,
	widget: WidgetPod<State, Box<dyn Widget<State>>>,
}

pub struct Overlay {
	child: Option<Child>,
}

impl Overlay {
	pub fn new() -> Overlay {
		Overlay { child: None }
	}
}

impl Widget<State> for Overlay {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, env: &Env) {
		let mut remove_child = false;
		if let Some(child) = &mut self.child {
			child.widget.event(ctx, event, data, env);

			match event {
				Event::MouseDown(mouse) => {
					if !child.widget.layout_rect().contains(mouse.pos) {
						ctx.set_active(true);
					}
					ctx.set_handled();
				}
				Event::MouseUp(mouse) => {
					if ctx.is_active() && !child.widget.layout_rect().contains(mouse.pos) {
						remove_child = true;
						ctx.set_active(false);
					}
					ctx.set_handled();
				}
				Event::MouseMove(_) => {
					ctx.set_handled();
				}
				_ => {}
			}
		}
		match event {
			Event::Command(cmd) if cmd.is(SHOW_AT) => {
				let (pos, bc, child_builder) = cmd.get_unchecked(SHOW_AT);
				self.child = Some(Child {
					origin: Some(*pos),
					bc: *bc,
					widget: WidgetPod::new(child_builder(env)),
				});
				ctx.children_changed();
			}
			Event::Command(cmd) if cmd.is(SHOW_MIDDLE) => {
				let (bc, child_builder) = cmd.get_unchecked(SHOW_MIDDLE);
				self.child = Some(Child {
					origin: None,
					bc: *bc,
					widget: WidgetPod::new(child_builder(env)),
				});
				ctx.children_changed();
			}
			Event::Command(cmd) if cmd.is(HIDE) => {
				remove_child = true;
			}
			_ => {}
		}
		if remove_child {
			self.child = None;
			ctx.request_layout();
			ctx.request_paint();
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &State, env: &Env) {
		if let Some(child) = &mut self.child {
			child.widget.lifecycle(ctx, event, data, env);
		}
	}

	fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &State, data: &State, env: &Env) {
		if let Some(child) = &mut self.child {
			child.widget.update(ctx, data, env);
		}
	}

	fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &State, env: &Env) -> Size {
		if let Some(child) = &mut self.child {
			let size = child.widget.layout(ctx, &child.bc, data, env);
			let origin = child
				.origin
				.unwrap_or((bc.max().to_vec2() / 2.0 - size.to_vec2() / 2.0).to_point());
			child
				.widget
				.set_layout_rect(ctx, data, env, Rect::from_origin_size(origin, size));
		}
		bc.max()
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &State, env: &Env) {
		if let Some(child) = &mut self.child {
			child.widget.paint(ctx, data, env);
		}
	}
}
