use druid::{kurbo::Line, Color, Env, PaintCtx, Point, RenderContext};

use crate::data::sheet::*;
use crate::theme;
use crate::util::coord::Coord;

use super::SheetEditor;

use super::{EditState, HoverState};

impl SheetEditor {
	pub fn draw_notes(&self, ctx: &mut PaintCtx, coord: &Coord, sheet: &Sheet, env: &Env) {
		let note_height = env.get(theme::NOTE_HEIGHT);
		let note_scale_knob = env.get(theme::NOTE_SCALE_KNOB);
		for (index, note) in sheet.notes.iter() {
			let pos = sheet.get_y(note.pitch);
			let spos = coord.to_screen_y(pos);

			let s_start = coord.to_screen_x(note.start);
			let s_length = coord.to_screen_w(note.length);

			// draw link to root
			if let Pitch::Relative(root, _) = note.pitch {
				let root = sheet.get_note(root).unwrap();
				let root_sfreq = coord.to_screen_y(sheet.get_y(root.pitch));
				if note.start < root.start || note.start > root.end() {
					let endpoint = coord.to_screen_x(if note.start < root.start { root.start } else { root.end() });
					ctx.stroke(
						Line::new((s_start, spos), (s_start, root_sfreq)),
						&env.get(theme::BG_FEATURE_COLOR),
						1.0,
					);
					ctx.stroke(
						Line::new((endpoint, root_sfreq), (s_start, root_sfreq)),
						&env.get(theme::BG_FEATURE_COLOR),
						1.0,
					);
				} else {
					ctx.stroke(
						Line::new((s_start, spos), (s_start, root_sfreq - note_height / 2.0)),
						&env.get(theme::BG_FEATURE_COLOR),
						1.0,
					);
				}
			}
			// draw note
			let p0 = Point::new(s_start, spos);
			let p1 = Point::new(s_start + s_length, spos);
			let line = Line::new(p0, p1);
			let mut color = env.get(theme::FEATURE_COLOR);
			match self.hover {
				HoverState::Move(id, _) if id == index => {
					color = env.get(theme::HIGHLIGHTED_COLOR);
				}
				_ => {}
			}
			match self.action {
				EditState::Scale(id) if id == index => {
					color = env.get(theme::HIGHLIGHTED_COLOR);
				}
				EditState::Move(id, _) if id == index => {
					color = env.get(theme::HIGHLIGHTED_COLOR);
				}
				_ => {}
			}
			ctx.stroke(line, &color, note_height);

			if self.hover == HoverState::Scale(index) || self.action == EditState::Scale(index) {
				let line = Line::new(
					Point::new((s_start + s_length - note_scale_knob).max(s_start + s_length * 0.60), spos),
					p1,
				);
				ctx.stroke(line, &Color::WHITE, note_height);
			}
		}
	}
}
