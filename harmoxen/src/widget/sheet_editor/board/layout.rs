use super::{Board, Style};
use crate::data::{layout::*, Rect, Size};
use crate::util::coord::Coord;
use iced::{Color, Vector};
use iced_graphics::Primitive;

impl<'a> Board<'a> {
	pub fn draw_layout(&self, size: Size, coord: &Coord, layout: &Layout, style: Style) -> Primitive {
		let view_width = coord.frame.x.view.size();
		let view_height = coord.frame.y.view.size();

		// get visible markers
		let mut markers = vec![&Layout::INITIAL_MARKER];
		for marker in &layout.markers {
			let s_origin = coord.to_screen_x(marker.at);
			if s_origin <= 0.0 {
				markers[0] = marker
			} else if s_origin > 0.0 || s_origin < size.width {
				markers.push(marker)
			} else {
				break;
			}
		}

		let mut primitives = vec![];

		// draw each patterns
		for i in 0..markers.len() {
			let s_start = coord.to_screen_x(markers[i].at);
			let s_end = markers.get(i + 1).map(|x| coord.to_screen_x(x.at)).unwrap_or(size.width);
			let pattern = &markers[i].pattern;

			if let Some(TimePattern {
				values: positions,
				nbeats, /* beats per bar */
			}) = &pattern.time
			{
				let s_bar_size = coord.to_screen_w(*nbeats as f32);
				let s_bars_start = if s_start < 0.0 {
					-((-s_start) % s_bar_size) // start drawing the bars just at the left of the view
				} else {
					s_start
				};
				let bar_color_offset = ((-s_start).max(0.0) / s_bar_size) as usize;
				let s_pattern_width = s_end - s_bars_start;
				let nbars = (s_pattern_width / s_bar_size).ceil() as usize;
				for bar in 0..nbars {
					let s_bar_start = s_bars_start + (bar as f32 * s_bar_size);
					let s_bar_end = s_end.min(size.width);

					let background = if (bar + bar_color_offset) % 2 == 0 {
						style.background_dark
					} else {
						style.background_light
					};
					let bounds = Rect::new(s_bar_start, 0.0, s_bar_end, size.height);

					let mut pattern_primitives = vec![];

					pattern_primitives.push(Primitive::Quad {
						bounds: bounds.into(),
						background,
						border_color: Color::TRANSPARENT,
						border_radius: 0.0,
						border_width: 0.0,
					});

					if view_width < 64.0 {
						for beat in 0..*nbeats {
							let s_beat_start = s_bar_start + coord.to_screen_w(beat as f32);
							if view_width < 24.0 {
								for pos in positions {
									let s_div = s_beat_start + coord.to_screen_w(*pos);
									pattern_primitives.push(Primitive::Quad {
										bounds: Rect::new(
											s_div - (4.0 / view_width).min(1.0),
											0.0,
											s_div + (4.0 / view_width).min(1.0),
											size.height,
										)
										.into(),
										background: Color::from_rgba(0.4, 0.4, 0.4, 0.3).into(),
										border_color: Color::TRANSPARENT,
										border_radius: 0.0,
										border_width: 0.0,
									});
								}
							}
							pattern_primitives.push(Primitive::Quad {
								bounds: Rect::new(
									s_beat_start - (16.0 / view_width).min(2.0),
									0.0,
									s_beat_start + (16.0 / view_width).min(2.0),
									size.height,
								)
								.into(),
								background: Color::from_rgba(0.4, 0.4, 0.4, 0.5).into(),
								border_color: Color::TRANSPARENT,
								border_radius: 0.0,
								border_width: 0.0,
							});
						}
					}
					primitives.push(Primitive::Clip {
						bounds: bounds.into(),
						offset: Vector::new(0, 0),
						content: Box::new(Primitive::Group {
							primitives: pattern_primitives,
						}),
					});
				}
			}

			// draw frequency snap lines
			if let Some(pattern) = &pattern.freq {
				let period = pattern.period();
				let min_freq = 2f32.powf(coord.frame.y.view.0);
				let max_freq = 2f32.powf(coord.frame.y.view.1);
				let min = (min_freq / pattern.base).log(period).floor() as isize;
				let max = (max_freq / pattern.base).log(period).ceil() as isize;
				for i in min..max {
					let base = pattern.base * period.powf(i as f32);
					let s_base = coord.to_screen_y(base.log2());

					// draw base line
					let mut root_color = style.root_line_color;
					root_color.a = 1.0;
					if s_base > 0.0 && s_base < size.height {
						primitives.push(Primitive::Quad {
							bounds: Rect::new(
								s_start.max(0.0),
								s_base - 2.0 / view_height,
								s_end.min(size.width),
								s_base + 2.0 / view_height,
							)
							.into(),
							background: root_color.into(),
							border_color: Color::TRANSPARENT,
							border_radius: 0.0,
							border_width: 0.0,
						});
					}

					// draw scale lines
					if view_height < 4.0 {
						for val in pattern.values.iter().skip(1) {
							let s_pos = coord.to_screen_y((base * val).log2());
							if s_pos > 0.0 && s_pos < size.height {
								primitives.push(Primitive::Quad {
									bounds: Rect::new(
										s_start.max(0.0),
										s_pos - 1.0 / view_height,
										s_end.min(size.width),
										s_pos + 1.0 / view_height,
									)
									.into(),
									background: Color::from_rgba(0.4, 0.4, 0.4, 0.5).into(),
									border_color: Color::TRANSPARENT,
									border_radius: 0.0,
									border_width: 0.0,
								});
							}
						}
					}
				}
			}
		}
		Primitive::Group { primitives }
	}
}
