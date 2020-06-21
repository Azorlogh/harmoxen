use druid::widget::{Flex, Label, SizedBox, WidgetExt};
use druid::Widget;

use crate::util::{Frame, Frame2};
use crate::widget::{common::*, *};

use crate::state::editors::sheet_editor::*;

const SCROLLBAR_THICKNESS: f64 = 32.0;
const TIMELINE_THICKNESS: f64 = 16.0;

pub fn build() -> impl Widget<State> {
	let preview = {
		Flex::column()
			.with_flex_child(SizedBox::empty().height(SCROLLBAR_THICKNESS + TIMELINE_THICKNESS), 0.0)
			.with_flex_child(
				Reversed::new(
					Preview::new().lens(Frame::view).lens(Frame2::y).lens(State::frame),
					(false, true),
				),
				1.0,
			)
	};

	let board = {
		let xrange = RangeSlider::horizontal((true, false)).lens(Frame2::x).lens(State::frame);
		let yrange = RangeSlider::vertical((false, false)).lens(Frame2::y).lens(State::frame);
		let timeline = Stack::new().with_child(Cursor::new()).with_child(MarkerEditor::new());
		let editor = SheetEditor::new();

		Flex::column()
			.with_flex_child(
				Flex::row()
					.with_flex_child(xrange, 1.0)
					.with_flex_child(Label::new("").fix_width(32.0), 0.0)
					.fix_height(SCROLLBAR_THICKNESS),
				0.0,
			)
			.with_flex_child(
				Flex::row()
					.with_flex_child(
						Flex::column()
							.with_child(timeline.fix_height(TIMELINE_THICKNESS))
							.with_flex_child(editor, 23.0),
						1.0,
					)
					.with_flex_child(Reversed::new(yrange.fix_width(SCROLLBAR_THICKNESS), (false, true)), 0.0),
				15.0,
			)
	};

	let view = ScrollView::new(
		Flex::row().with_child(preview.fix_width(96.0)).with_flex_child(board, 1.0),
		State::frame,
	);

	// editor
	Flex::column().with_flex_child(view, 1.0)
}
