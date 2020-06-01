//! This is useful to convert between coordinates systems in the app

#![allow(unused)]
use crate::util::Frame2;
use druid::{Point, Size};

pub struct Coord {
	pub frame: Frame2,
	pub size: Size,
}

impl Coord {
	pub fn new(frame: Frame2, size: Size) -> Coord {
		Coord { frame, size }
	}

	// SCREEN TO BOARD

	pub fn to_board_x(&self, x: f64) -> f64 {
		let range = self.frame.x.view;
		(x / self.size.width) * range.size() + range.0
	}

	pub fn to_board_y(&self, y: f64) -> f64 {
		let range = self.frame.y.view;
		let height = self.size.height;
		((height - y) / height) * range.size() + range.0
	}

	pub fn to_board_p(&self, p: Point) -> Point {
		Point::new(self.to_board_x(p.x), self.to_board_y(p.y))
	}

	pub fn to_board_w(&self, w: f64) -> f64 {
		(w / self.size.width) * self.frame.x.view.size()
	}

	pub fn to_board_h(&self, h: f64) -> f64 {
		(h / self.size.height) * self.frame.y.view.size()
	}

	// BOARD TO SCREEN

	pub fn to_screen_x(&self, x: f64) -> f64 {
		let range = self.frame.x.view;
		((x - range.0) / range.size()) * self.size.width
	}

	pub fn to_screen_y(&self, y: f64) -> f64 {
		let range = self.frame.y.view;
		let height = self.size.height;
		height - ((y - range.0) / range.size()) * height
	}

	pub fn to_screen_w(&self, x: f64) -> f64 {
		let range = self.frame.x.view;
		(x / range.size()) * self.size.width
	}

	pub fn to_screen_h(&self, h: f64) -> f64 {
		let range = self.frame.y.view;
		let height = self.size.height;
		height - (h / range.size()) * height
	}
}
