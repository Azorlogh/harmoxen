use super::{
	note::{Note, Pitch},
	Index as SheetIndex, Sheet,
};
use std::collections::HashSet;

type SheetNote = Note<SheetIndex>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Index {
	SheetIndex(SheetIndex),
	ClipboardIndex(usize),
}

#[derive(Debug)]
pub struct Clipboard(Vec<Note<Index>>);

impl Clipboard {
	pub fn new() -> Clipboard {
		Clipboard(vec![])
	}

	pub fn cut(&mut self, sheet: &mut Sheet, selection: &mut HashSet<SheetIndex>) {
		self.0 = vec![];
		let selection: Vec<SheetIndex> = selection.drain().collect();
		for &idx in &selection {
			let sheet_note = sheet.remove_note(idx).unwrap();
			let note: Note<Index> = Note {
				start: sheet_note.start,
				length: sheet_note.length,
				pitch: match sheet_note.pitch {
					Pitch::Absolute(freq) => Pitch::Absolute(freq),
					Pitch::Relative(idx, interval) => Pitch::Relative(
						if let Some(i) = selection.iter().position(|&i| i == idx) {
							Index::ClipboardIndex(i)
						} else {
							Index::SheetIndex(idx)
						},
						interval,
					),
				},
			};
			self.0.push(note);
		}
	}

	pub fn copy(&mut self, sheet: &Sheet, selection: &HashSet<SheetIndex>) {
		self.0 = vec![];
		let selection: Vec<SheetIndex> = selection.iter().cloned().collect();
		for &idx in &selection {
			let sheet_note = sheet.get_note(idx).unwrap();
			let note: Note<Index> = Note {
				start: sheet_note.start,
				length: sheet_note.length,
				pitch: match sheet_note.pitch {
					Pitch::Absolute(freq) => Pitch::Absolute(freq),
					Pitch::Relative(idx, interval) => Pitch::Relative(
						if let Some(i) = selection.iter().position(|&i| i == idx) {
							Index::ClipboardIndex(i)
						} else {
							Index::SheetIndex(idx)
						},
						interval,
					),
				},
			};
			self.0.push(note);
		}
	}

	pub fn paste(&self, sheet: &mut Sheet, selection: &mut HashSet<SheetIndex>) {
		let mut entries: Vec<(usize, Note<Index>)> = (0..self.0.len()).map(|i| (i, self.0[i].clone())).collect();
		let mut sheet_indices: Vec<Option<SheetIndex>> = vec![None; self.0.len()];

		selection.clear();
		while entries.len() > 0 {
			let entry @ (i, note) = entries.pop().unwrap();
			let pitch = match note.pitch {
				Pitch::Relative(idx, interval) => match idx {
					Index::ClipboardIndex(idx) => {
						if let Some(sheet_idx) = sheet_indices[idx] {
							Pitch::<SheetIndex>::Relative(sheet_idx, interval)
						} else {
							entries.push(entry);
							let parent = entries.swap_remove(idx);
							entries.push(parent);
							continue;
						}
					}
					Index::SheetIndex(sheet_idx) => Pitch::<SheetIndex>::Relative(sheet_idx, interval),
				},
				Pitch::Absolute(freq) => Pitch::<SheetIndex>::Absolute(freq),
			};

			let sheet_note = SheetNote {
				start: note.start,
				length: note.length,
				pitch,
			};
			let index = sheet.add_note(sheet_note);
			sheet_indices[i] = Some(index);
			selection.insert(index);
		}
	}
}
