use crate::data::{
	sheet::{Note, Pitch},
	Point,
};
use serde::{Deserialize, Serialize};

mod pattern;
pub use pattern::*;

mod marker;
pub use marker::Marker;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layout {
	pub markers: Vec<Marker>,
}

impl Default for Layout {
	fn default() -> Layout {
		Layout {
			markers: vec![Marker::default()],
		}
	}
}

impl Layout {
	pub const INITIAL_MARKER: Marker = Marker {
		at: -1.0,
		pattern: Pattern::EMPTY,
		pattern_input: PatternInput {
			freq: freq_input::FreqInput::None,
			time: time_input::TimeInput::None,
		},
	};

	pub fn add_marker(&mut self, marker: Marker) -> usize {
		self.markers.push(marker);
		self.markers.len() - 1
	}
	pub fn delete_marker(&mut self, idx: usize) {
		self.markers.remove(idx);
	}
	pub fn set_marker_time(&mut self, idx: usize, at: f32) -> usize {
		// this is very inefficient
		self.markers[idx].at = at;
		let mut indices: Vec<usize> = (0..self.markers.len()).collect();
		indices.sort_by(|&a, &b| self.markers[a].at.partial_cmp(&self.markers[b].at).unwrap());
		let new_idx = indices.iter().position(|&x| x == idx).unwrap();
		let new_markers = indices.into_iter().map(|i| self.markers[i].clone()).collect();
		self.markers = new_markers;
		new_idx
	}
	pub fn set_marker_pattern(&mut self, idx: usize, pattern: Pattern) {
		self.markers[idx].pattern = pattern;
	}

	pub fn get_marker_at(&self, at: f32, exclude: Option<usize>) -> &Marker {
		let mut closest = &Self::INITIAL_MARKER;
		for (i, marker) in self.markers.iter().enumerate() {
			if Some(i) != exclude && marker.at <= at && marker.at > closest.at {
				closest = marker;
			}
		}
		closest
	}

	pub fn quantize_time(&self, time: f32, floor: bool) -> f32 {
		self.quantize_time_impl(time, floor, None, None)
	}

	pub fn quantize_time_after(&self, time: f32, after: f32) -> f32 {
		self.quantize_time_impl(time, false, None, Some(after))
	}

	pub fn quantize_time_exclude(&self, time: f32, floor: bool, exclude: usize) -> f32 {
		self.quantize_time_impl(time, floor, Some(exclude), None)
	}

	pub fn quantize_time_impl(&self, time: f32, floor: bool, exclude: Option<usize>, after: Option<f32>) -> f32 {
		let marker = self.get_marker_at(time, exclude);
		if let Some(pattern) = &marker.pattern.time {
			let layout_time = time - marker.at;
			let beat_time = layout_time.fract();
			let beat = layout_time.floor();
			let min = after.map_or(std::f32::NEG_INFINITY, |x| x - marker.at - beat);
			pattern
				.values
				.iter()
				.map(|&x| vec![x - 1.0, x, x + 1.0])
				.flatten()
				.fold(0.0, |acc, x| {
					if x > min && (!floor || x < beat_time) && (x - beat_time).abs() < (acc - beat_time).abs() {
						x
					} else {
						acc
					}
				}) + beat + marker.at
		} else {
			time
		}
	}

	pub fn quantize_freq(&self, at: f32, mut freq: f32) -> f32 {
		let pattern = &self.get_marker_at(at, None).pattern;
		if let Some(pattern) = &pattern.freq {
			let period = pattern.period();
			let base = pattern.base * period.powf((freq / pattern.base).log(period).floor());
			freq = pattern.values.iter().fold(0.0, |acc, x| {
				if (base * x - freq).abs() < (acc - freq).abs() {
					base * x
				} else {
					acc
				}
			});
		}
		freq
	}

	pub fn quantize_position(&self, pos: Point) -> Point {
		let x = self.quantize_time(pos.x, false);
		let y = self.quantize_freq(pos.x, 2f32.powf(pos.y));
		Point::new(x, y.log2())
	}

	pub fn quantize_note(&self, mut note: Note) -> Note {
		note.start = self.quantize_time(note.start, false);
		note.length = self.quantize_time_after(note.start + note.length, note.start) - note.start;
		if let Pitch::Absolute(freq) = note.pitch {
			note.pitch = Pitch::Absolute(self.quantize_freq(note.start, freq));
		}
		note
	}
}
