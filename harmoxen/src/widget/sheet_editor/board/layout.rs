use super::{Board, Style};
use crate::data::{layout::*, Rect, Size};
use crate::util::coord::Coord;
use iced::canvas::{Frame, Path, Stroke};
use iced::{Color, Vector};
use iced_graphics::Primitive;

impl<'a> Board<'a> {
	pub fn draw_layout(&self, size: Size, coord: &Coord, layout: &Layout, style: Style) -> Primitive {
		let mut primitives_bg = vec![];
		let mut primitives_fg = vec![];

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

		// draw each pattern
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

					// draw bar background
					primitives_bg.push(Primitive::Quad {
						bounds: bounds.into(),
						background,
						border_width: 0.0,
						border_color: Color::TRANSPARENT,
						border_radius: 0.0,
					});

					let mut frame = Frame::new(size);

					if view_width < 64.0 {
						for beat in 0..*nbeats {
							let s_beat_start = s_bar_start + coord.to_screen_w(beat as f32);
							if view_width < 24.0 {
								for pos in positions {
									let s_div = s_beat_start + coord.to_screen_w(*pos);
									// draw subdiv
									frame.stroke(
										&Path::line([s_div, 0.0].into(), [s_div, size.height].into()),
										Stroke {
											width: 2f32,
											color: Color::from_rgba(0.4, 0.4, 0.4, 0.5).into(),
											..Default::default()
										},
									);
								}
							}
							// draw beat
							frame.stroke(
								&Path::line([s_beat_start, 0.0].into(), [s_beat_start, size.height].into()),
								Stroke {
									width: 4f32,
									color: Color::from_rgba(0.4, 0.4, 0.4, 0.8).into(),
									..Default::default()
								},
							);
						}
					}

					primitives_fg.push(Primitive::Clip {
						bounds: bounds.into(),
						offset: Vector::new(0, 0),
						content: Box::new(frame.into_geometry().into_primitive()),
					});
				}
			}

			// draw frequency snap lines
			if let Some(pattern) = &pattern.freq {
				let mut frame = Frame::new(size);

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
						frame.stroke(
							&Path::line([s_start.max(0.0), s_base].into(), [s_end.min(size.width), s_base].into()),
							Stroke {
								width: 4.0,
								color: root_color,
								..Default::default()
							},
						);
					}

					// draw scale lines
					if view_height < 4.0 {
						for val in pattern.values.iter().skip(1) {
							let s_pos = coord.to_screen_y((base * val).log2());
							if s_pos > 0.0 && s_pos < size.height {
								frame.stroke(
									&Path::line([s_start.max(0.0), s_pos].into(), [s_end.min(size.width), s_pos].into()),
									Stroke {
										width: 2.0,
										color: Color::from_rgba(0.4, 0.4, 0.4, 0.5),
										..Default::default()
									},
								);
							}
						}
					}
				}
				primitives_fg.push(frame.into_geometry().into_primitive());
			}
		}
		let mut primitives = vec![];
		primitives.extend(primitives_bg);
		primitives.extend(primitives_fg);
		Primitive::Group { primitives }
	}
}
