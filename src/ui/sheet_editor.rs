use druid::widget::{Flex, Label, SizedBox, WidgetExt};
use druid::Widget;

use crate::util::{Frame, Frame2};
use crate::widget::{common::*, sheet_editor::*};

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

	let x_range_slider = RangeSlider::horizontal((true, false)).lens(Frame2::x).lens(State::frame);
	let y_range_slider = RangeSlider::vertical((false, false)).lens(Frame2::y).lens(State::frame);
	let timeline = Stack::new().with_child(Cursor::new()).with_child(MarkerEditor::new());

	// editor
	Stack::new()
		.with_child(ScrollView::new().lens(State::frame))
		.with_child(Shortcuts::new())
		.with_child(
			Flex::row().with_child(preview.fix_width(96.0)).with_flex_child(
				Flex::column()
					.with_flex_child(
						Flex::row()
							.with_flex_child(x_range_slider, 1.0)
							.with_flex_child(Label::new("").fix_width(32.0), 0.0)
							.fix_height(SCROLLBAR_THICKNESS),
						0.0,
					)
					.with_flex_child(
						Flex::row()
							.with_flex_child(
								Flex::column()
									.with_child(timeline.fix_height(TIMELINE_THICKNESS))
									.with_flex_child(Stack::new().with_child(Board::new()).with_child(Selection::new()), 23.0),
								1.0,
							)
							.with_flex_child(
								Reversed::new(y_range_slider.fix_width(SCROLLBAR_THICKNESS), (false, true)),
								0.0,
							),
						15.0,
					),
				1.0,
			),
		)
}
