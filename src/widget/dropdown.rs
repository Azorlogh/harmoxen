//! A dropdown widget.
use druid::theme;
use druid::widget::prelude::*;
use druid::widget::{Flex, Label, LabelText};
use druid::{
	Affine, Command, Data, Insets, LinearGradient, Point, Rect, RenderContext, Selector, UnitPoint, Widget, WidgetExt,
};
use std::{cell::RefCell, rc::Rc};

use crate::widget::common::Button;

use super::overlay;

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

const CLICK_ITEM: Selector<usize> = Selector::new("dropdown.click-item");

pub struct Item<T> {
	text: String,
	action: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
}

impl<T> Item<T> {
	pub fn new(text: &str, action: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Item<T> {
		Item {
			text: text.to_owned(),
			action: Box::new(action),
		}
	}
}

pub struct DropDown<T> {
	label: Label<T>,
	label_size: Size,
	items: Vec<Item<T>>,
}

impl<T: Data> DropDown<T> {
	pub fn new(text: impl Into<LabelText<T>>) -> DropDown<T> {
		DropDown {
			label: Label::new(text),
			label_size: Size::ZERO,
			items: vec![],
		}
	}

	pub fn add_item(&mut self, item: Item<T>) {
		self.items.push(item)
	}
	pub fn with_item(mut self, item: Item<T>) -> DropDown<T> {
		self.add_item(item);
		self
	}
}

impl<T: Data> Widget<T> for DropDown<T> {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
		match event {
			Event::MouseDown(_) => {
				ctx.set_active(true);
				ctx.request_paint();
			}
			Event::MouseUp(mouse) => {
				if ctx.is_active() {
					ctx.set_active(false);
					ctx.request_paint();
					let size = ctx.size();
					let pos = Point::new(0.0, size.height) + (mouse.window_pos - mouse.pos);
					let bc = BoxConstraints::new(Size::new(size.width, 0.0), Size::new(size.width, 1000.0));
					let mut texts = vec![];
					for Item { text, .. } in &self.items {
						texts.push(text.clone());
					}
					let widget_id = ctx.widget_id();
					ctx.submit_command(
						Command::new(
							overlay::SHOW_AT,
							(
								pos,
								bc,
								Box::new(move |_| {
									let mut flex = Flex::column();
									for i in 0..texts.len() {
										flex.add_child(
											Button::new(texts[i].as_str())
												.on_click(move |ctx, _, _| {
													ctx.submit_command(Command::new(CLICK_ITEM, i), widget_id)
												})
												.expand_width(),
										);
									}
									Box::new(flex)
								}),
							),
						),
						ctx.window_id(),
					);
				}
			}
			Event::Command(cmd) if cmd.is(CLICK_ITEM) => {
				let idx = *cmd.get_unchecked(CLICK_ITEM);
				(self.items[idx].action)(ctx, data, env);
				ctx.submit_command(overlay::HIDE, ctx.window_id());
			}
			_ => (),
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
		if let LifeCycle::HotChanged(_) = event {
			ctx.request_paint();
		}
		self.label.lifecycle(ctx, event, data, env)
	}

	fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
		self.label.update(ctx, old_data, data, env)
	}

	fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
		bc.debug_check("Button");
		let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
		let label_bc = bc.shrink(padding).loosen();
		self.label_size = self.label.layout(layout_ctx, &label_bc, data, env);
		// HACK: to make sure we look okay at default sizes when beside a textbox,
		// we make sure we will have at least the same height as the default textbox.
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
		let stroke_width = env.get(theme::BUTTON_BORDER_WIDTH);

		let rounded_rect = Rect::from_origin_size(Point::ORIGIN, size)
			.inset(-stroke_width / 2.0)
			.to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

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

		let border_color = if is_hot {
			env.get(theme::BORDER_LIGHT)
		} else {
			env.get(theme::BORDER_DARK)
		};

		ctx.stroke(rounded_rect, &border_color, stroke_width);

		ctx.fill(rounded_rect, &bg_gradient);

		let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

		ctx.with_save(|ctx| {
			ctx.transform(Affine::translate(label_offset));
			self.label.paint(ctx, data, env);
		});
	}
}
