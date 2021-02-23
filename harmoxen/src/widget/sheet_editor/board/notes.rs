use super::{Action, Board, Hover, Style, NOTE_HEIGHT, NOTE_SCALE_KNOB};
use crate::data::{sheet::Pitch, Point, Rect, Size};
use crate::util::coord::Coord;
use iced::canvas::{Frame, Path, Stroke};
use iced_graphics::Primitive;

impl<'a> Board<'a> {
	pub fn draw_notes(&self, size: Size, coord: &Coord, style: Style) -> Primitive {
		let mut frame = Frame::new(size);
		let sheet = &self.sheet;
		for (index, note) in sheet.get_notes() {
			let pos = sheet.get_y(note.pitch);
			let s_pos = coord.to_screen_y(pos);

			let s_start = coord.to_screen_x(note.start);
			let s_length = coord.to_screen_w(note.length);

			if let Pitch::Relative(root, _) = note.pitch {
				let root = sheet.get_note(root).unwrap();
				let root_s_freq = coord.to_screen_y(sheet.get_y(root.pitch));
				let path = if note.start < root.start || note.start > root.end() {
					let endpoint = coord.to_screen_x(if note.start < root.start { root.start } else { root.end() });
					Path::new(|b| {
						b.move_to([s_start, s_pos].into());
						b.line_to([s_start, root_s_freq].into());
						b.line_to([endpoint, root_s_freq].into());
					})
				} else {
					Path::line([s_start, s_pos].into(), [s_start, root_s_freq - NOTE_HEIGHT / 2.0].into())
				};
				frame.stroke(
					&path,
					Stroke {
						width: 1.0,
						color: style.note_color.into(),
						..Default::default()
					},
				);
			}

			// draw note
			let p0 = Point::new(s_start, s_pos);
			let p1 = Point::new(s_start + s_length, s_pos);
			let mut color = style.note_color;

			if let Hover::Move(idx) = self.state.hover {
				if idx == index {
					color = style.note_highlight;
				}
			}
			if self.selection.contains(&index) {
				color = style.note_selected;
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
			let path = Path::rectangle([p0.x, p0.y - NOTE_HEIGHT / 2.0].into(), [s_length, NOTE_HEIGHT].into());
			frame.fill(&path, color);

			let mut resizing = self.state.hover == Hover::Scale(index);
			if let Action::Scale(idx, _) = self.state.action {
				resizing = resizing || idx == index;
			}
			if resizing {
				let rect = Rect::new(
					(s_start + s_length - NOTE_SCALE_KNOB).max(s_start + s_length * 0.60),
					s_pos - NOTE_HEIGHT / 2.0,
					p1.x,
					p1.y + NOTE_HEIGHT / 2.0,
				);
				let path = Path::rectangle(rect.position().into(), rect.size());
				frame.fill(&path, style.note_highlight);
			}
		}
		frame.into_geometry().into_primitive()
	}
}
