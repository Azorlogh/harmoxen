use druid::kurbo::{Point, RoundedRect, Size};
use druid::theme;
use druid::widget::Label;
use druid::{
	Affine, BoxConstraints, Color, Data, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, LinearGradient,
	MouseEvent, PaintCtx, Rect, RenderContext, UnitPoint, UpdateCtx, Widget,
};
use std::fmt::Display;

// TODO: Make this less ugly

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

#[derive(Debug, PartialEq)]
enum Side {
	None,
	Left,
	Right,
}
use Side::*;

pub struct Selector<T> {
	label: Label<String>,
	label_size: Size,
	choices: Vec<T>,
	curr_idx: usize,
	hover: Side,
}

impl<T: Data + Clone> Selector<T> {
	pub fn new(choices: Vec<T>) -> Selector<T> {
		Selector {
			label: Label::new(|data: &String, _env: &Env| data.clone().into()),
			label_size: Size::ZERO,
			choices,
			curr_idx: 0,
			hover: None,
		}
	}
}

impl<T: Data + Display> Widget<T> for Selector<T> {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, _env: &Env) {
		let width = ctx.size().width;
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
						match self.hover {
							Left => self.curr_idx = self.curr_idx.saturating_sub(1),
							Right => self.curr_idx = self.curr_idx.saturating_add(1).min(self.choices.len() - 1),
							None => (),
						}
						*data = self.choices[self.curr_idx].clone();
					}
				}
			}
			Event::MouseMove(MouseEvent { pos, .. }) => {
				if !ctx.is_active() && ctx.is_hot() {
					let hover = if pos.x < width / 2.0 { Left } else { Right };
					if self.hover != hover {
						self.hover = hover;
						ctx.request_paint();
					}
				}
			}
			_ => (),
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		match event {
			LifeCycle::WidgetAdded => {
				self.label.lifecycle(ctx, event, &data.to_string(), env);
			}
			LifeCycle::HotChanged(hot) => {
				if !hot {
					self.hover = None;
					ctx.request_paint();
				}
			}
			_ => {}
		}
	}

	fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
		self.label.update(ctx, &old_data.to_string(), &data.to_string(), env);
	}

	fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
		let label_bc = bc.shrink(padding).loosen();
		self.label_size = self.label.layout(layout_ctx, &label_bc, &data.to_string(), env);

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
		let bg_gradient = if is_active {
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

		match self.hover {
			Side::Left => {
				ctx.fill(
					Rect::from_points((0.0, 0.0), (size.width / 2.0, size.height)),
					&LinearGradient::new(
						UnitPoint::LEFT,
						UnitPoint::RIGHT,
						(Color::rgba8(255, 255, 255, 255), Color::rgba8(255, 255, 255, 0)),
					),
				);
			}
			Side::Right => {
				ctx.fill(
					Rect::from_points((size.width / 2.0, 0.0), (size.width, size.height)),
					&LinearGradient::new(
						UnitPoint::LEFT,
						UnitPoint::RIGHT,
						(Color::rgba8(255, 255, 255, 0), Color::rgba8(255, 255, 255, 255)),
					),
				);
			}
			_ => {}
		}

		let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

		ctx.with_save(|ctx| {
			ctx.transform(Affine::translate(label_offset));
			self.label.paint(ctx, &data.to_string(), env);
		});
	}
}
