use druid::widget::{Button, Flex, Label, SizedBox, TextBox, ViewSwitcher, WidgetExt};
use druid::Widget;

use crate::commands;
use crate::util::{Frame, Frame2};
use crate::widget::{common::*, *};

use crate::state::editors::sheet_editor::{Menu, State};

const SCROLLBAR_THICKNESS: f64 = 32.0;
const TIMELINE_THICKNESS: f64 = 16.0;

pub fn build() -> impl Widget<State> {
	let menu_selector = {
		Flex::row()
			.with_child(
				Setter::new("File", Menu::File)
					.lens(State::open_menu)
					.fix_width(80.0)
					.padding(3.0),
			)
			.with_child(
				Setter::new("Edit", Menu::Edit)
					.lens(State::open_menu)
					.fix_width(80.0)
					.padding(3.0),
			)
			.with_child(
				Button::new("Layout")
					.on_click(|ctx, _, _| ctx.submit_command(commands::OPEN_LAYOUT_EDITOR, ctx.window_id()))
					.lens(State::open_menu)
					.fix_width(120.0)
					.padding(3.0),
			)
			.with_flex_spacer(1.0)
			.with_child(Label::new("BPM:"))
			.with_child(ParseLazy::new(TextBox::new()).lens(State::tempo).padding(3.0))
			.expand_width()
	};

	let menu = ViewSwitcher::new(
		|data: &State, _| std::mem::discriminant(&data.open_menu),
		|_, data, _| {
			Box::new(match data.open_menu {
				Menu::File => Flex::row().with_child(Label::new("File placeholder")).with_flex_spacer(1.0),
				Menu::Edit => Flex::row().with_child(Label::new("Edit placeholder")).with_flex_spacer(1.0),
			})
		},
	)
	.padding(3.0);

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
		let editor = ScrollView::new(SheetEditor::new(), State::frame);

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

	let view = Flex::row().with_child(preview.fix_width(96.0)).with_flex_child(board, 1.0);

	// editor
	Flex::column()
		.with_child(menu_selector.fix_height(40.0))
		.with_child(menu.fix_height(40.0))
		.with_flex_child(view, 1.0)
}
