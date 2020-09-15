use super::sheet::{Note, Pitch};
use druid::Point;
use serde::{Deserialize, Serialize};

mod pattern;
pub use pattern::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layout {
	pub markers: Vec<(f64, Pattern)>,
}

impl Default for Layout {
	fn default() -> Layout {
		Layout {
			markers: vec![(0.0, Default::default())],
		}
	}
}

impl Layout {
	pub fn add_marker(&mut self, at: f64, pattern: Pattern) -> usize {
		self.markers.push((at, pattern));
		self.markers.len() - 1
	}
	pub fn delete_marker(&mut self, idx: usize) {
		self.markers.remove(idx);
	}
	pub fn set_marker_time(&mut self, idx: usize, at: f64) -> usize {
		// this is very inefficient
		self.markers[idx].0 = at;
		let mut indices: Vec<usize> = (0..self.markers.len()).collect();
		indices.sort_by(|&a, &b| self.markers[a].0.partial_cmp(&self.markers[b].0).unwrap());
		let new_idx = indices.iter().position(|&x| x == idx).unwrap();
		let new_markers = indices.into_iter().map(|i| self.markers[i].clone()).collect();
		self.markers = new_markers;
		new_idx
	}
	pub fn set_marker_pattern(&mut self, idx: usize, pattern: Pattern) {
		self.markers[idx].1 = pattern;
	}

	pub fn get_marker_at(&self, at: f64, exclude: Option<usize>) -> &(f64, Pattern) {
		let mut closest = &(-1.0, Pattern::EMPTY);
		for (i, marker) in self.markers.iter().enumerate() {
			if Some(i) != exclude && marker.0 <= at && marker.0 > closest.0 {
				closest = marker;
			}
		}
		closest
	}

	pub fn quantize_time(&self, time: f64, floor: bool) -> f64 {
		self.quantize_time_impl(time, floor, None, None)
	}

	pub fn quantize_time_after(&self, time: f64, after: f64) -> f64 {
		self.quantize_time_impl(time, false, None, Some(after))
	}

	pub fn quantize_time_exclude(&self, time: f64, floor: bool, exclude: usize) -> f64 {
		self.quantize_time_impl(time, floor, Some(exclude), None)
	}

	pub fn quantize_time_impl(&self, time: f64, floor: bool, exclude: Option<usize>, after: Option<f64>) -> f64 {
		let (start, pattern) = self.get_marker_at(time, exclude);
		if let Some(pattern) = &pattern.time {
			let layout_time = time - start;
			let beat_time = layout_time.fract();
			let beat = layout_time.floor();
			let min = after.map_or(std::f64::NEG_INFINITY, |x| x - start - beat);
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
				}) + beat + start
		} else {
			time
		}
	}

	pub fn quantize_freq(&self, at: f64, mut freq: f64) -> f64 {
		let pattern = &self.get_marker_at(at, None).1;
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
		let y = self.quantize_freq(pos.x, 2f64.powf(pos.y));
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
