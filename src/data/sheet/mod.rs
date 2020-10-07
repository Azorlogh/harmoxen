use crate::data::icp;
use crate::util::{intersect, Range};
use druid::{kurbo::Line, Point, Rect};
use generational_arena::{Arena, Index};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

mod interval;
pub use interval::*;

pub mod note;

mod clipboard;
pub use clipboard::Clipboard;

pub type Pitch = note::Pitch<Index>;
pub type Note = note::Note<Index>;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Sheet {
	pub notes: Arena<Note>,
	pub indices: Vec<Index>,
}

#[allow(dead_code)]
impl Sheet {
	pub fn get_freq(&self, pitch: Pitch) -> f64 {
		match pitch {
			Pitch::Absolute(freq) => freq,
			Pitch::Relative(idx, ratio) => self.get_freq(self.notes[idx].pitch) * ratio,
		}
	}

	pub fn get_y(&self, pitch: Pitch) -> f64 {
		self.get_freq(pitch).log2()
	}

	pub fn get_notes(&self) -> Vec<(Index, Note)> {
		self.indices.iter().map(|&idx| (idx, self.notes[idx])).collect()
	}

	pub fn get_note(&self, id: Index) -> Option<Note> {
		self.notes.get(id).copied()
	}

	pub fn get_note_mut(&mut self, id: Index) -> Option<&mut Note> {
		self.notes.get_mut(id)
	}

	pub fn get_note_at(&self, pos: Point, note_height: f64) -> Option<Index> {
		let mut closest = (None, f64::INFINITY);
		for &index in &self.indices {
			let note = self.notes[index];
			let dist = (pos.y - note.y(self)).abs();
			if note.start <= pos.x && pos.x <= note.start + note.length && dist <= note_height / 2.0 && dist <= closest.1 {
				closest = (Some(index), dist);
			}
		}
		closest.0
	}

	// get notes from a point in board coordinates
	pub fn get_notes_at(&self, pos: Point, note_height: f64) -> Vec<Index> {
		let mut out = vec![];
		for (index, note) in &self.notes {
			if note.start <= pos.x && note.start + note.length >= pos.x && (pos.y - note.y(self)).abs() <= note_height / 2.0 {
				out.push(index);
			}
		}
		out
	}

	// get notes intersecting a rect in board coordinates
	pub fn get_notes_rect(&self, rect: Rect, note_height: f64) -> Vec<Index> {
		let mut out = vec![];
		for (index, note) in &self.notes {
			let note_y = note.y(self);
			if intersect::rect_rect(
				Rect::from_points(
					Point::new(note.start, note_y - note_height / 2.0),
					Point::new(note.start + note.length, note_y + note_height / 2.0),
				),
				rect,
			)
			.is_some()
			{
				out.push(index);
			}
		}
		out
	}

	pub fn get_size(&self) -> f64 {
		self.notes.iter().fold(0.0, |tmp, (_, note)| tmp.max(note.end()))
	}

	pub fn get_bounds(&self) -> (Range, Range) {
		let mut x = Range(f64::INFINITY, f64::NEG_INFINITY);
		let mut y = Range(f64::INFINITY, f64::NEG_INFINITY);
		for (_, note) in &self.notes {
			let freq = self.get_freq(note.pitch);
			x.0 = x.0.min(note.start);
			x.1 = x.1.max(note.start + note.length);
			y.0 = y.0.min(freq);
			y.1 = y.1.max(freq);
		}
		(x, y)
	}

	pub fn add_note(&mut self, note: Note) -> Index {
		let index = self.notes.insert(note);
		self.indices.push(index);
		index
	}

	pub fn move_note(&mut self, id: Index, start: f64, y: f64) {
		if let Some(note) = self.notes.get_mut(id) {
			note.start = start;
			if let Pitch::Absolute(_) = note.pitch {
				note.pitch = Pitch::Absolute(2f64.powf(y));
			}
		}
	}

	pub fn resize_note_to(&mut self, id: Index, time: f64) {
		if let Some(note) = self.notes.get_mut(id) {
			note.length = time - note.start;
		}
	}

	pub fn remove_note(&mut self, index: Index) -> Option<Note> {
		let removed = [index].iter().cloned().collect();
		self.disconnect_children(&removed);
		self.indices.retain(|&idx| idx != index);
		self.notes.remove(index)
	}

	pub fn remove_notes_along(&mut self, line: Line, note_height: f64) {
		let mut removed = HashSet::new();
		for (idx, note) in &self.notes {
			let note_y = note.y(self);
			let rect = Rect::from_points(
				Point::new(note.start, note_y - note_height / 2.0),
				Point::new(note.start + note.length, note_y + note_height / 2.0),
			);
			if intersect::line_rect(line, rect) {
				removed.insert(idx);
			}
		}
		self.disconnect_children(&removed);
		self.indices.retain(|idx| !removed.contains(idx));
		self.notes.retain(|idx, _| !removed.contains(&idx));
	}

	fn disconnect_children(&mut self, parents: &HashSet<Index>) {
		let mut children = vec![];
		for (idx, note) in &self.notes {
			if let Pitch::Relative(root, _) = note.pitch {
				if parents.contains(&root) {
					children.push((idx, self.get_freq(note.pitch)))
				}
			}
		}
		for (idx, freq) in children {
			self.notes[idx].pitch = Pitch::Absolute(freq);
		}
	}

	pub fn get_events_at_time(&self, time: f64) -> Vec<icp::Event> {
		let mut events = vec![];
		for (index, note) in &self.notes {
			if note.start < time && note.end() > time {
				let icp_note = icp::Note {
					id: index.into_raw_parts().0,
					freq: self.get_freq(note.pitch),
				};
				events.push(icp::Event::NotePlay(icp_note));
			}
		}
		events
	}

	pub fn get_events(&self, range: crate::util::Range) -> Vec<icp::Event> {
		let mut events = vec![];
		for (index, note) in &self.notes {
			let id = index.into_raw_parts().0;
			if range.contains(note.start) {
				let icp_note = icp::Note {
					id,
					freq: self.get_freq(note.pitch),
				};
				events.push(icp::Event::NotePlay(icp_note));
			}
			if range.contains(note.end()) {
				events.push(icp::Event::NoteStop(id));
			}
		}
		events
	}
}
