use super::{Action, Hover, SheetEditor};
use crate::data::sheet::*;
use crate::theme;
use crate::util::{coord::Coord, intersect};
use druid::{kurbo::Line, Color, Env, PaintCtx, Point, Rect, RenderContext};
use generational_arena::Index;
use std::collections::HashSet;

impl SheetEditor {
	pub fn draw_notes(&self, ctx: &mut PaintCtx, coord: &Coord, sheet: &Sheet, selection: &HashSet<Index>, env: &Env) {
		let note_height = env.get(theme::NOTE_HEIGHT);
		let b_note_height = coord.to_board_h(note_height);
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

			if let Hover::Move(idx) = self.hover {
				if idx == index {
					color = env.get(theme::HIGHLIGHTED_COLOR);
				}
			}
			if selection.contains(&index) {
				color = env.get(theme::SELECTED_COLOR);
			}
			match self.action {
				Action::Scale(id, _) if id == index => {
					color = env.get(theme::HIGHLIGHTED_COLOR);
				}
				Action::Move(id, _, _) if id == index => {
					color = env.get(theme::HIGHLIGHTED_COLOR);
				}
				Action::SelectionAdd(p0, p1) => {
					let note_y = sheet.get_y(note.pitch);
					if intersect::rect_rect(
						Rect::from_points(
							Point::new(note.start, note_y - b_note_height / 2.0),
							Point::new(note.start + note.length, note_y + b_note_height / 2.0),
						),
						Rect::from_points(p0, p1),
					)
					.is_some()
					{
						color = env.get(theme::SELECTED_COLOR);
					}
				}
				Action::SelectionRemove(p0, p1) => {
					let note_y = sheet.get_y(note.pitch);
					if intersect::rect_rect(
						Rect::from_points(
							Point::new(note.start, note_y - b_note_height / 2.0),
							Point::new(note.start + note.length, note_y + b_note_height / 2.0),
						),
						Rect::from_points(p0, p1),
					)
					.is_some()
					{
						color = env.get(theme::FEATURE_COLOR);
					}
				}
				_ => {}
			}
			ctx.stroke(line, &color, note_height);

			// resizing handle
			let mut resizing = self.hover == Hover::Scale(index);
			if let Action::Scale(idx, _) = self.action {
				resizing = resizing || idx == index;
			}
			if resizing {
				let line = Line::new(
					Point::new((s_start + s_length - note_scale_knob).max(s_start + s_length * 0.60), spos),
					p1,
				);
				ctx.stroke(line, &Color::WHITE, note_height);
			}
		}
	}
}
