use druid::kurbo::BezPath;
use druid::{
	BoxConstraints, Color, Command, ContextMenu, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
	LocalizedString, MenuDesc, MenuItem, MouseEvent, PaintCtx, Point, Rect, RenderContext, Size, UpdateCtx, Widget,
};
use std::cell::RefMut;

use crate::commands;
use crate::state::sheet_editor::{
	layout::{Layout, Pattern},
	State,
};
use crate::util::coord::Coord;

pub struct MarkerEditor;

pub fn get_hover(x: f64, coord: Coord, layout: RefMut<Layout>) -> Option<usize> {
	let extent = coord.to_board_w(8.0);
	let offset = coord.to_board_w(4.0);
	for (i, (marker_x, _)) in layout.markers.iter().enumerate() {
		if x > marker_x - offset && x < marker_x + extent + offset {
			return Some(i);
		}
	}
	None
}

impl Widget<State> for MarkerEditor {
	fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut State, _env: &Env) {
		let mut layout = data.layout.borrow_mut();
		let coord = Coord::new(data.frame.clone(), ctx.size());
		match event {
			Event::MouseDown(mouse) if mouse.button.is_left() => {
				let board_x = coord.to_board_x(mouse.pos.x);
				if let Some(id) = get_hover(board_x, coord, layout) {
					data.curr_marker = id;
					ctx.set_handled();
					ctx.set_active(true);
					ctx.submit_command(commands::SHEET_EDITOR_REDRAW, ctx.window_id());
					ctx.request_paint();
				}
			}
			Event::MouseDown(mouse) if mouse.button.is_right() => {
				let board_x = coord.to_board_x(mouse.pos.x);
				let menu = if let Some(id) = get_hover(board_x, coord, layout) {
					ContextMenu::new(make_marker_context_menu::<crate::state::State>(id), mouse.window_pos)
				} else {
					ContextMenu::new(make_context_menu::<crate::state::State>(board_x), mouse.window_pos)
				};
				ctx.show_context_menu(menu);
			}
			Event::MouseMove(MouseEvent { pos, mods, .. }) => {
				let mut time = coord.to_board_x(pos.x).max(0.0);
				let idx = data.curr_marker;
				if ctx.is_active() && idx != 0 {
					if !mods.ctrl {
						time = layout.quantize_time_exclude(time, false, idx);
					}
					let new_idx = layout.set_marker_time(idx, time);
					data.curr_marker = new_idx;
					ctx.request_paint();
					ctx.submit_command(commands::SHEET_EDITOR_REDRAW, ctx.window_id());
				}
			}
			Event::MouseUp(_) => {
				ctx.set_active(false);
				ctx.request_paint();
			}
			Event::Command(ref cmd) if cmd.is(commands::MARKER_ADD) => {
				let pos = *cmd.get_unchecked(commands::MARKER_ADD);
				let idx = layout.add_marker(pos, Pattern::default());
				data.curr_marker = idx;
				ctx.submit_command(commands::LAYOUT_APPLY, ctx.window_id());
				ctx.request_paint();
			}
			Event::Command(ref cmd) if cmd.is(commands::MARKER_DELETE) => {
				let id = *cmd.get_unchecked(commands::MARKER_DELETE);
				layout.delete_marker(id);
				ctx.request_paint();
				ctx.submit_command(commands::LAYOUT_CHANGED, ctx.window_id());
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
		let size = ctx.size();
		let Size { width, height } = size;

		ctx.clip(Rect::from_origin_size(Point::ORIGIN, size));

		let xrange = data.frame.x.view;
		for (i, (x, _)) in data.layout.borrow().markers.iter().enumerate() {
			let screen_x = ((x - xrange.0) / xrange.size()) * width;

			let mut curve = BezPath::new();
			curve.move_to((screen_x, 0.0));
			curve.line_to((screen_x, height));
			curve.line_to((screen_x + 8.0, height));
			curve.quad_to((screen_x + 6.0, height / 2.0), (screen_x + 8.0, 0.0));
			curve.line_to((screen_x, 0.0));

			let color = if data.curr_marker == i {
				Color::rgb8(0xFF, 0xFF, 0xFF)
			} else {
				Color::rgb8(0xCC, 0xCC, 0xCC)
			};
			ctx.fill(curve, &color);
		}
	}
}

fn make_context_menu<T: Data>(pos: f64) -> MenuDesc<T> {
	MenuDesc::empty().append(MenuItem::new(
		LocalizedString::new("Add Marker"),
		Command::new(commands::MARKER_ADD, pos),
	))
}

fn make_marker_context_menu<T: Data>(id: usize) -> MenuDesc<T> {
	MenuDesc::empty().append(MenuItem::new(
		LocalizedString::new("Delete Marker"),
		Command::new(commands::MARKER_DELETE, id),
	))
}
