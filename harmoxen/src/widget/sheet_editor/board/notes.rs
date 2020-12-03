use super::{Action, Board, Hover, Style, NOTE_HEIGHT, NOTE_SCALE_KNOB};
use crate::data::{sheet::Pitch, Point, Rect};
use crate::util::coord::Coord;
use iced_graphics::{Background, Color, Primitive};

impl<'a> Board<'a> {
	pub fn draw_notes(&self, coord: &Coord, style: Style) -> Primitive {
		let mut primitives = vec![];

		let sheet = &self.sheet;
		for (index, note) in sheet.get_notes() {
			let pos = sheet.get_y(note.pitch);
			let spos = coord.to_screen_y(pos);

			let s_start = coord.to_screen_x(note.start);
			let s_length = coord.to_screen_w(note.length);

			if let Pitch::Relative(root, _) = note.pitch {
				let root = sheet.get_note(root).unwrap();
				let root_sfreq = coord.to_screen_y(sheet.get_y(root.pitch));
				if note.start < root.start || note.start > root.end() {
					let endpoint = coord.to_screen_x(if note.start < root.start {
						root.start
					} else {
						root.end()
					});
					primitives.push(Primitive::Quad {
						bounds: Rect::new(s_start - 0.5, spos, s_start + 0.5, root_sfreq).into(),
						background: style.note_color.into(),
						border_radius: 0,
						border_width: 0,
						border_color: Color::TRANSPARENT,
					});
					primitives.push(Primitive::Quad {
						bounds: Rect::new(endpoint, root_sfreq - 0.5, s_start, root_sfreq + 0.5)
							.into(),
						background: style.note_color.into(),
						border_radius: 0,
						border_width: 0,
						border_color: Color::TRANSPARENT,
					});
				} else {
					primitives.push(Primitive::Quad {
						bounds: Rect::new(
							s_start - 0.5,
							spos,
							s_start + 0.5,
							root_sfreq - NOTE_HEIGHT / 2.0,
						)
						.into(),
						background: style.note_color.into(),
						border_radius: 0,
						border_width: 0,
						border_color: Color::TRANSPARENT,
					});
				}
			}

			// draw note
			let p0 = Point::new(s_start, spos);
			let p1 = Point::new(s_start + s_length, spos);
			let mut color = style.note_color;

			if let Hover::Move(idx) = self.state.hover {
				if idx == index {
					color = style.note_highlight;
				}
			}
			match self.state.action {
				Action::Scale(id, _) if id == index => {
					color = style.note_highlight;
				}
				Action::Move(id, _, _) if id == index => {
					color = style.note_highlight;
				}
				_ => {}
			}
			primitives.push(Primitive::Quad {
				bounds: Rect::new(
					p0.x,
					p0.y - NOTE_HEIGHT / 2.0,
					p1.x,
					p1.y + NOTE_HEIGHT / 2.0,
				)
				.into(),
				background: Background::Color(color),
				border_radius: 0,
				border_width: 0,
				border_color: Color::TRANSPARENT,
			});

			let mut resizing = self.state.hover == Hover::Scale(index);
			if let Action::Scale(idx, _) = self.state.action {
				resizing = resizing || idx == index;
			}
			if resizing {
				primitives.push(Primitive::Quad {
					bounds: Rect::new(
						(s_start + s_length - NOTE_SCALE_KNOB).max(s_start + s_length * 0.60),
						spos - NOTE_HEIGHT / 2.0,
						p1.x,
						p1.y + NOTE_HEIGHT / 2.0,
					)
					.into(),
					background: style.note_highlight.into(),
					border_radius: 0,
					border_width: 0,
					border_color: Color::TRANSPARENT,
				})
			}
		}
		Primitive::Group { primitives }
	}
}
