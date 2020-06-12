use druid::kurbo::Line;

use druid::{
	BoxConstraints, Color, Command, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, MouseEvent,
	PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget,
};

use crate::commands;
use crate::data::icp;
use crate::util::Range;

pub struct Preview {
	playing: Option<f64>,
}

impl Preview {
	pub fn new() -> Preview {
		Preview { playing: None }
	}
}

type State = Range;

impl Widget<State> for Preview {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, _env: &Env) {
		let range = &*data;
		let size = ctx.size();
		match event {
			Event::MouseDown(MouseEvent {
				pos,
				button: MouseButton::Left,
				..
			}) => {
				let freq = 2f64.powf((pos.y / size.height) * range.size() + range.0);
				self.playing = Some(freq);
				ctx.set_active(true);
				ctx.request_paint();
				let cmd = Command::new(commands::ICP, icp::Event::NotePlay(icp::Note { id: 1000, freq }));
				ctx.submit_command(cmd, ctx.window_id());
			}
			Event::MouseMove(mouse) if mouse.buttons.has_left() => {
				if let Some(prev_freq) = &mut self.playing {
					let freq = 2f64.powf((mouse.pos.y / size.height) * range.size() + range.0);
					if range.contains((mouse.pos.y / size.height) * range.size() + range.0) {
						*prev_freq = freq;
						ctx.request_paint();
						let cmd = Command::new(commands::ICP, icp::Event::NoteChangeFreq(1000, freq));
						ctx.submit_command(cmd, ctx.window_id());
					}
				}
			}
			Event::MouseUp(MouseEvent {
				button: MouseButton::Left,
				..
			}) => {
				self.playing = None;
				ctx.set_active(false);
				ctx.request_paint();
				let cmd = Command::new(commands::ICP, icp::Event::NoteStop(1000));
				ctx.submit_command(cmd, ctx.window_id());
			}
			_ => {}
		}
	}

	fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &State, _env: &Env) {}

	fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {}

	fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &State, _env: &Env) -> Size {
		bc.max()
	}

	fn paint(&mut self, ctx: &mut PaintCtx, data: &State, _env: &Env) {
		let range = &*data;
		let size = ctx.size();

		let rect = Rect::from_origin_size(Point::ORIGIN, size);

		ctx.clip(rect);

		ctx.fill(rect, &Color::rgb8(0x70, 0x70, 0x70));

		if let Some(freq) = self.playing {
			let vpos = (freq.log2() - range.0) / range.size();
			let spos = vpos * size.height;
			let p0 = Point::new(0.0, spos);
			let p1 = Point::new(size.width, spos);
			let line = Line::new(p0, p1);
			ctx.stroke(line, &Color::rgb8(0xE0, 0xE0, 0xE0), 1.0 / range.size());
		}
	}
}
