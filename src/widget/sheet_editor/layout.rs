use druid::{kurbo::Line, Color, Env, PaintCtx, Point, Rect, RenderContext};

use crate::state::sheet_editor::layout::*;
use crate::theme;

use super::SheetEditor;
use crate::util::coord::Coord;

impl SheetEditor {
	pub fn draw_layout(&self, ctx: &mut PaintCtx, coord: &Coord, layout: &Layout, env: &Env) {
		let size = ctx.size();
		let view_width = coord.frame.x.view.size();
		let view_height = coord.frame.y.view.size();
		// get visible markers
		let mut markers = vec![&(-1.0, Pattern::EMPTY)];
		for marker in &layout.markers {
			let s_origin = coord.to_screen_x(marker.0);
			if s_origin <= 0.0 {
				markers[0] = marker
			} else if s_origin > 0.0 || s_origin < size.width {
				markers.push(marker)
			} else {
				break;
			}
		}

		// draw each patterns
		for i in 0..markers.len() {
			let s_start = coord.to_screen_x(markers[i].0);
			let s_end = markers.get(i + 1).map(|x| coord.to_screen_x(x.0)).unwrap_or(size.width);
			let pattern = &markers[i].1;

			if let Some((positions, bpb /* beats per bar */)) = &pattern.time {
				let s_bar_size = coord.to_screen_w(*bpb as f64);
				let s_bars_start = if s_start < 0.0 {
					-((-s_start) % s_bar_size) // start drawing the bars just at the left of the view
				} else {
					s_start
				};
				let bar_color_offset = ((-s_start).max(0.0) / s_bar_size) as usize;
				let s_pattern_width = s_end - s_bars_start;
				let nbars = (s_pattern_width / s_bar_size).ceil() as usize;
				for bar in 0..nbars {
					let s_bar_start = s_bars_start + (bar as f64 * s_bar_size);
					let s_bar_end = s_end.min(size.width);

					let bg_col = &env.get(if (bar + bar_color_offset) % 2 == 0 {
						theme::BACKGROUND_2
					} else {
						theme::BACKGROUND_0
					});
					let bg = Rect::new(s_bar_start, 0.0, s_bar_end, size.height);
					ctx.save().unwrap();
					ctx.clip(bg);
					ctx.fill(bg, bg_col);

					if view_width < 128.0 {
						for beat in 0..*bpb {
							let s_beat_start = s_bar_start + coord.to_screen_w(beat as f64);
							if view_width < 48.0 {
								for pos in positions {
									let s_div = s_beat_start + coord.to_screen_w(*pos);
									ctx.stroke(
										Line::new((s_div, 0.0), (s_div, size.height)),
										&Color::rgb8(0x66, 0x66, 0x66),
										(4.0 / view_width).min(1.0),
									);
								}
							}
							ctx.stroke(
								Line::new((s_beat_start, 0.0), (s_beat_start, size.height)),
								&Color::rgb8(0x66, 0x66, 0x66),
								(16.0 / view_width).min(2.0),
							);
						}
					}
					ctx.restore().unwrap();
				}
			}

			// draw frequency snap lines
			if let Some(positions) = &pattern.freq {
				for pos in positions {
					let s_pos = coord.to_screen_y(pos.log2());
					if s_pos > 0.0 && s_pos < size.height {
						let p0 = Point::new(s_start.max(0.0), s_pos);
						let p1 = Point::new(s_end.min(size.width), s_pos);
						ctx.stroke(Line::new(p0, p1), &env.get(theme::BACKGROUND_1), 2.0 / view_height);
					}
				}
			}
		}
	}
}
