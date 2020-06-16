// used to set the data to a specific value

#![allow(unused)]

use druid::kurbo::{Point, RoundedRect, Size};
use druid::theme;
use druid::widget::{Label, LabelText};
use druid::{
	Affine, BoxConstraints, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, LinearGradient, PaintCtx,
	RenderContext, UnitPoint, UpdateCtx, Widget,
};

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct Setter<T> {
	label: Label<T>,
	label_size: Size,
	value: T,
	is_set: bool,
}

impl<T: PartialEq + Copy + Data + 'static> Setter<T> {
	pub fn new(text: impl Into<LabelText<T>>, value: T) -> Setter<T> {
		Setter {
			label: Label::new(text),
			label_size: Size::ZERO,
			value,
			is_set: false,
		}
	}
}

impl<T: PartialEq + Copy + Data> Widget<T> for Setter<T> {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, _env: &Env) {
		match event {
			Event::MouseDown(_) => {
				ctx.set_active(true);
				ctx.request_paint();
			}
			Event::MouseUp(_) => {
				if ctx.is_active() {
					ctx.set_active(false);
					ctx.request_paint();
					if ctx.is_hot() {
						*data = self.value;
						self.is_set = true;
					}
				}
			}
			_ => (),
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		match event {
			LifeCycle::WidgetAdded => {
				self.is_set = self.value == *data;
			}
			LifeCycle::HotChanged(_) => {
				ctx.request_paint();
			}
			_ => {}
		}
		self.label.lifecycle(ctx, event, data, env);
	}

	fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
		self.is_set = self.value == *data;
		self.label.update(ctx, old_data, data, env);
	}

	fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		bc.debug_check("Setter");
		let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
		let label_bc = bc.shrink(padding).loosen();
		self.label_size = self.label.layout(layout_ctx, &label_bc, data, env);

		let min_height = env.get(theme::BORDERED_WIDGET_HEIGHT);
		bc.constrain(Size::new(
			self.label_size.width + padding.width,
			(self.label_size.height + padding.height).max(min_height),
		))
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
		let is_active = ctx.is_active();
		let is_hot = ctx.is_hot();
		let size = ctx.size();

		let rounded_rect = RoundedRect::from_origin_size(Point::ORIGIN, size, 4.);
		let bg_gradient = if self.is_set || is_active {
			LinearGradient::new(
				UnitPoint::TOP,
				UnitPoint::BOTTOM,
				(env.get(theme::BUTTON_DARK), env.get(theme::BUTTON_LIGHT)),
			)
		} else {
			LinearGradient::new(
				UnitPoint::TOP,
				UnitPoint::BOTTOM,
				(env.get(theme::BUTTON_LIGHT), env.get(theme::BUTTON_DARK)),
			)
		};

		let border_color = if is_hot || is_active {
			env.get(theme::BORDER_LIGHT)
		} else {
			env.get(theme::BORDER_DARK)
		};

		ctx.stroke(rounded_rect, &border_color, env.get(theme::BUTTON_BORDER_WIDTH));

		ctx.fill(rounded_rect, &bg_gradient);

		let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

		ctx.with_save(|ctx| {
			ctx.transform(Affine::translate(label_offset));
			self.label.paint(ctx, data, env);
		});
	}
}
