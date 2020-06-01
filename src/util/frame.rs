//! A frame is a view (range) enclosed in boundaries (another range)

use crate::util::Range;
use druid::{Data, Lens};

#[derive(Debug, Default, Clone, Data, Lens, PartialEq)]
pub struct Frame2 {
	pub x: Frame,
	pub y: Frame,
}

#[derive(Debug, Default, Clone, Data, Lens, PartialEq)]
pub struct Frame {
	pub view: Range,
	pub bounds: Range,
}
