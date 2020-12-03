//! A frame is a view (range) enclosed in boundaries (another range)

use crate::data::Range;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Frame2 {
	pub x: Frame,
	pub y: Frame,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Frame {
	pub view: Range,
	pub bounds: Range,
}
