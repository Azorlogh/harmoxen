use druid::kurbo::Line;

use druid::{
	BoxConstraints, Color, Command, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, MouseEvent,
	PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget,
};

use crate::theme;

use crate::commands;
use crate::state::sheet_editor::State;

pub struct Cursor {
	origin: Option<f64>,
}

impl Cursor {
	pub fn new() -> Cursor {
		Cursor { origin: None }
	}
}

impl Widget<State> for Cursor {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, _env: &Env) {
		let position = &mut data.cursor;
		let view = data.frame.x.view;
		let size = ctx.size();
		match event {
			Event::MouseDown(MouseEvent {
				button: MouseButton::Left,
				pos,
				..
			}) => {
				ctx.request_paint();
				ctx.set_active(true);
				let new_position = (pos.x / size.width) * view.size() + view.0;
				*position = new_position;
			}
			Event::MouseMove(MouseEvent { pos, .. }) => {
				if ctx.is_active() {
					ctx.request_paint();
					let new_position = (pos.x / size.width) * view.size() + view.0;
					*position = new_position.max(0.0);
				}
			}
			Event::MouseUp(MouseEvent {
				button: MouseButton::Left,
				..
			}) => ctx.set_active(false),
			Event::Command(ref cmd) if cmd.is(commands::PLAY_START) => {
				self.origin = Some(*position);
				ctx.request_anim_frame();
			}
			Event::Command(ref cmd) if cmd.is(commands::PLAY_STOP) => {
				*position = self.origin.unwrap();
				self.origin = None;
				ctx.request_paint();
			}
			Event::Command(ref cmd) if cmd.is(commands::CURSOR_ADVANCE) => {
				let delta = *cmd.get_unchecked(commands::CURSOR_ADVANCE);
				*position = (*position + delta * (data.tempo / 60.0)) % data.sheet.borrow().get_size();
				ctx.request_paint();
			}
			_ => {}
		}
	}

	fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &State, _env: &Env) {
		if let LifeCycle::AnimFrame(delta) = event {
			if let Some(_) = self.origin {
				ctx.submit_command(
					Command::new(commands::CURSOR_ADVANCE, (*delta as f64) / 1000000000.0),
					ctx.widget_id(),
				);
				ctx.request_anim_frame();
			}
		}
	}

	fn update(&mut self, ctx: &mut UpdateCtx, old_data: &State, data: &State, _env: &Env) {
		if old_data.tempo != data.tempo {
			ctx.submit_command(Command::new(commands::TEMPO_CHANGED, data.tempo), ctx.window_id());
		}
	}

	fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &State, _env: &Env) -> Size {
		bc.max()
	}

	fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &State, env: &Env) {
		let size = paint_ctx.size();

		let rect = Rect::from_origin_size(Point::ORIGIN, size);

		paint_ctx.clip(rect);

		paint_ctx.fill(rect, &env.get(theme::NEUTRAL_COLOR_1));

		let (pos, view) = (data.cursor, data.frame.x.view);
		let screen_pos = ((pos - view.0) / view.size()) * size.width;

		let p0 = Point::new(screen_pos, 0.0);
		let p1 = Point::new(screen_pos, size.height);
		let line = Line::new(p0, p1);
		paint_ctx.stroke(line, &Color::rgb8(0xF0, 0xF0, 0xF0), 1.0);
	}
}
