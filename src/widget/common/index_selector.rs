use druid::kurbo::{Point, RoundedRect, Size};
use druid::theme;
use druid::widget::Label;
use druid::{
	Affine, BoxConstraints, Color, Env, Event, EventCtx, Insets, LayoutCtx, LifeCycle, LifeCycleCtx, LinearGradient,
	MouseEvent, PaintCtx, Rect, RenderContext, Selector, UnitPoint, UpdateCtx, Widget,
};

// TODO: Make this less ugly

pub const SET_CHOICES: Selector<Vec<String>> = Selector::new("index-selector.set-choices");

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

#[derive(Debug, PartialEq)]
enum Side {
	None,
	Left,
	Right,
}
use Side::*;

pub struct IndexSelector {
	label: Label<()>,
	label_size: Size,
	choices: Vec<String>,
	hover: Side,
}

impl IndexSelector {
	pub fn new(choices: Vec<String>) -> IndexSelector {
		assert!(!choices.is_empty(), "tried to create IndexSelector without an empty Vec");
		IndexSelector {
			label: Label::new(choices[0].clone()),
			label_size: Size::ZERO,
			choices,
			hover: None,
		}
	}
}

impl Widget<usize> for IndexSelector {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut usize, _env: &Env) {
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
					ctx.request_layout();
					if ctx.is_hot() {
						match self.hover {
							Left => *data = data.saturating_sub(1),
							Right => *data = data.saturating_add(1).min(self.choices.len() - 1),
							None => (),
						}
					}
					self.label.set_text(self.choices[*data].clone());
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
			Event::Command(cmd) if cmd.is(SET_CHOICES) => {
				let choices = cmd.get_unchecked(SET_CHOICES).clone();
				self.choices = choices;
				self.label.set_text(self.choices[*data].clone());
				ctx.request_layout();
				ctx.request_paint();
			}
			_ => {}
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &usize, env: &Env) {
		match event {
			LifeCycle::WidgetAdded => {
				self.label.lifecycle(ctx, event, &(), env);
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

	fn update(&mut self, _ctx: &mut UpdateCtx, old_data: &usize, data: &usize, _env: &Env) {
		if old_data != data {
			self.label.set_text(self.choices[*data].clone());
		}
	}

	fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &usize, env: &Env) -> Size {
		let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
		let label_bc = bc.shrink(padding).loosen();
		self.label_size = self.label.layout(layout_ctx, &label_bc, &(), env);

		let min_height = env.get(theme::BORDERED_WIDGET_HEIGHT);
		bc.constrain(Size::new(
			self.label_size.width + padding.width,
			(self.label_size.height + padding.height).max(min_height),
		))
	}

	fn paint(&mut self, ctx: &mut PaintCtx, _data: &usize, env: &Env) {
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
			self.label.paint(ctx, &(), env);
		});
	}
}
