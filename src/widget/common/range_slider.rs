use druid::kurbo::Rect;

use druid::{
	theme, BoxConstraints, Color, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, MouseEvent, PaintCtx,
	Point, RenderContext, Size, UpdateCtx, Widget,
};

use crate::util::Frame;

#[derive(Clone, Copy)]
enum Axis {
	X,
	Y,
}

impl Axis {
	fn major<T: Into<(f64, f64)>>(&self, coords: T) -> f64 {
		match *self {
			Axis::X => coords.into().0,
			Axis::Y => coords.into().1,
		}
	}

	fn with_major<T: Into<(f64, f64)> + From<(f64, f64)>>(&self, coords: T, value: f64) -> T {
		let mut t = coords.into();
		match *self {
			Axis::X => t.0 = value,
			Axis::Y => t.1 = value,
		}
		t.into()
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
	Idle,
	Moving(f64),
	Scaling(bool),
}

pub struct RangeSlider {
	direction: Axis,
	state: State,
	bounds: (bool, bool),
}

const HANDLE_OFFSET: f64 = 8.0;

impl RangeSlider {
	pub fn horizontal(bounds: (bool, bool)) -> RangeSlider {
		RangeSlider {
			state: State::Idle,
			direction: Axis::X,
			bounds,
		}
	}

	pub fn vertical(bounds: (bool, bool)) -> RangeSlider {
		RangeSlider {
			state: State::Idle,
			direction: Axis::Y,
			bounds,
		}
	}
}

impl Widget<Frame> for RangeSlider {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, frame: &mut Frame, _env: &Env) {
		let dir = self.direction;
		let active_bounds = self.bounds;
		let bounds = &mut frame.bounds;
		let view = &mut frame.view;
		match event {
			Event::MouseDown(MouseEvent {
				button: MouseButton::Left,
				pos,
				..
			}) => {
				let size = dir.major(ctx.size());
				let pos = dir.major(*pos);
				let gpos = pos * (bounds.size() / size) + bounds.0;
				let screen_view = (*view - bounds.0) * (size / bounds.size());

				if (screen_view.0 + HANDLE_OFFSET - pos).abs() < HANDLE_OFFSET {
					ctx.set_active(true);
					self.state = State::Scaling(false);
				} else if (screen_view.1 - HANDLE_OFFSET - pos).abs() < HANDLE_OFFSET {
					ctx.set_active(true);
					self.state = State::Scaling(true);
				} else if pos < screen_view.0 {
					*view -= (0.1 * view.size()).min(view.0 - gpos);
				} else if pos > screen_view.1 {
					*view += (0.1 * view.size()).min(gpos - view.1);
				} else {
					ctx.set_active(true);
					self.state = State::Moving(gpos - view.0);
				}
				ctx.request_paint();
			}
			Event::MouseMove(MouseEvent { pos, .. }) => {
				let size = dir.major(ctx.size());
				let pos = dir.major(*pos);
				let gpos = pos * (bounds.size() / size) + bounds.0;
				let screen_view = (*view - bounds.0) * (size / bounds.size());
				if ctx.is_active() {
					match self.state {
						State::Moving(anchor) => {
							let mut new = gpos - anchor;
							if active_bounds.0 {
								new = new.max(bounds.0);
							}
							if active_bounds.1 {
								new = new.min(bounds.1 - view.size());
							}
							*view = *view - view.0 + new;
							ctx.request_paint();
						}
						State::Scaling(false) => {
							if pos < screen_view.1 - HANDLE_OFFSET * 6.0 {
								if !active_bounds.0 || pos > 3.0 + HANDLE_OFFSET {
									view.0 = gpos - HANDLE_OFFSET * (bounds.size() / size);
								} else {
									view.0 = bounds.0;
								}
							} else {
								view.0 = view.1 - HANDLE_OFFSET * 6.0 * (bounds.size() / size);
							}
							ctx.request_paint();
						}
						State::Scaling(true) => {
							if pos > screen_view.0 + HANDLE_OFFSET * 6.0 {
								if !active_bounds.1 || pos < bounds.1 - 3.0 - HANDLE_OFFSET {
									view.1 = gpos + HANDLE_OFFSET * (bounds.size() / size);
								} else {
									view.1 = bounds.1;
								}
							} else {
								view.1 = view.0 + HANDLE_OFFSET * 6.0 * (bounds.size() / size);
							}
							ctx.request_paint();
						}
						_ => {}
					}
				} else if !ctx.is_hot() {
					ctx.request_paint();
					self.state = State::Idle;
				} else {
					let new_state = if (screen_view.0 + HANDLE_OFFSET - pos).abs() < HANDLE_OFFSET {
						State::Scaling(false)
					} else if (screen_view.1 - HANDLE_OFFSET - pos).abs() < HANDLE_OFFSET {
						State::Scaling(true)
					} else if screen_view.0 < pos && pos < screen_view.1 {
						State::Moving(gpos * (bounds.size() / size) - view.0)
					} else {
						State::Idle
					};
					if std::mem::discriminant(&self.state) != std::mem::discriminant(&new_state) {
						self.state = new_state;
						ctx.request_paint();
					}
				}
			}
			Event::MouseUp(_) => {
				if ctx.is_active() {
					ctx.set_active(false);
					ctx.request_paint();
				}
				self.state = State::Idle;
			}
			_ => {}
		}
	}

	fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &Frame, _env: &Env) {}

	fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Frame, data: &Frame, _env: &Env) {
		if old_data.bounds != data.bounds {
			ctx.request_paint();
		}
	}

	fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &Frame, _env: &Env) -> Size {
		bc.max()
	}

	fn paint(&mut self, ctx: &mut PaintCtx, view: &Frame, env: &Env) {
		let dir = self.direction;
		let bounds = view.bounds;
		let view = view.view;
		let size = ctx.size();
		let size_major = dir.major(size);
		let ctx_rect = Rect::from_origin_size(Point::ORIGIN, size);

		ctx.clip(ctx_rect);

		ctx.fill(ctx_rect, &Color::rgb8(0x40, 0x40, 0x40));

		let bar_origin = dir.with_major(
			Point::new(3.0, 3.0),
			3.0 + (size_major - 6.0) * (view.0 - bounds.0) / bounds.size(),
		);

		let bar_size = dir.with_major(
			Size::new(size.width - 6.0, size.height - 6.0),
			(size_major - 6.0) * (view.size() / bounds.size()),
		);

		let rect = Rect::from_origin_size(bar_origin, bar_size);

		ctx.fill(rect, &Color::rgb8(0x80, 0x80, 0x80));

		let mut border_color = env.get(theme::BORDER_DARK);
		let mut color_handle_right = env.get(theme::BORDER_DARK);
		let mut color_handle_left = env.get(theme::BORDER_DARK);

		match self.state {
			State::Moving(_) => border_color = env.get(theme::BORDER_LIGHT),
			State::Scaling(false) => color_handle_left = env.get(theme::BORDER_LIGHT),
			State::Scaling(true) => color_handle_right = env.get(theme::BORDER_LIGHT),
			_ => {}
		}

		// border
		ctx.stroke(rect, &border_color, 1.0);

		// handles
		ctx.fill(
			Rect::from_origin_size(
				dir.with_major(
					Point::new(4.0, 4.0),
					3.0 + (size_major - 6.0) * (view.0 - bounds.0) / bounds.size() + HANDLE_OFFSET - 4.0,
				),
				dir.with_major(Size::new(size.width - 8.0, size.height - 8.0), 8.0),
			),
			&color_handle_left,
		);
		ctx.fill(
			Rect::from_origin_size(
				dir.with_major(
					Point::new(4.0, 4.0),
					3.0 + (size_major - 6.0) * (view.1 - bounds.0) / bounds.size() - HANDLE_OFFSET - 4.0,
				),
				dir.with_major(Size::new(size.width - 8.0, size.height - 8.0), 8.0),
			),
			&color_handle_right,
		);
	}
}
