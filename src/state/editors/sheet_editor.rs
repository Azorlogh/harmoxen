use crate::util::{Frame, Frame2, Range};
use druid::{Data, Lens};
use generational_arena::Index;
use std::{cell::RefCell, rc::Rc};

use crate::data::layout::Layout;
use crate::data::sheet::{Interval, Sheet};

#[derive(Clone, Data, Lens)]
pub struct State {
	pub frame: Frame2,
	pub sheet: Rc<RefCell<Sheet>>,
	pub cursor: f64,
	pub playing: bool,
	pub layout: Rc<RefCell<Layout>>,
	pub tempo: f64,
	pub interval_input: Interval,
	pub curr_marker: usize,
	pub selection: Rc<RefCell<Vec<Index>>>,
}
impl Default for State {
	fn default() -> State {
		State {
			frame: Frame2 {
				x: Frame {
					view: Range(0.0, 4.0),
					bounds: Range(0.0, 5.0),
				},
				y: Frame {
					view: Range(8.0, 9.0),
					bounds: Range(4.0, 14.0),
				},
			},
			sheet: Rc::new(RefCell::new(Sheet::default())),
			cursor: 0.0,
			playing: false,
			layout: Rc::new(RefCell::new(Layout::default())),
			tempo: 172.0,
			interval_input: Interval::Ratio(3, 2),
			curr_marker: 0,
			selection: Rc::new(RefCell::new(vec![])),
		}
	}
}
