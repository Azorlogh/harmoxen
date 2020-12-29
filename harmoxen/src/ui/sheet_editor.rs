use crate::{
	state::{
		sheet_editor::{Message, State},
		Message as RootMessage,
	},
	widget::{sheet_editor::*, *},
	Theme,
};
use iced::{Element, Length};

const PREVIEW_THICKNESS: u16 = 96;
const SCROLLBAR_THICKNESS: u16 = 32;
const TIMELINE_THICKNESS: u16 = 16;

fn rootmsg(msg: Message) -> RootMessage {
	RootMessage::SheetEditor(msg)
}

pub fn build(state: &mut State, theme: Theme) -> Element<RootMessage> {
	let x_scrollbar = Container::new(
		RangeSlider::horizontal(
			&mut state.wstates.xrange_slider,
			state.frame.x,
			(true, false),
			false,
			|view| rootmsg(Message::XViewChanged(view)),
		)
		.style(theme),
	)
	.height(SCROLLBAR_THICKNESS.into())
	.width(Length::Fill);

	let y_scrollbar = Container::new(
		RangeSlider::vertical(&mut state.wstates.yrange_slider, state.frame.y, (true, true), true, |view| {
			rootmsg(Message::YViewChanged(view))
		})
		.style(theme),
	)
	.width(SCROLLBAR_THICKNESS.into());

	let timeline = Container::new(
		Stack::new()
			.push(Cursor::new(&mut state.wstates.cursor, state.cursor, state.frame))
			.push(
				MarkerEditor::new(
					&mut state.wstates.marker_editor,
					state.frame,
					&state.layout,
					state.curr_marker,
				)
				.style(theme),
			),
	)
	.height(TIMELINE_THICKNESS.into());

	Stack::new()
		.push(
			Row::new()
				.push(
					Column::new()
						.push(Space::with_height((SCROLLBAR_THICKNESS + TIMELINE_THICKNESS).into()))
						.push(Preview::new(&mut state.wstates.preview, state.frame).style(theme))
						.width(PREVIEW_THICKNESS.into()),
				)
				.push(
					Column::new()
						.push(
							Row::new()
								.push(Column::new().push(x_scrollbar).push(timeline).width(Length::Fill))
								.push(Space::with_width(SCROLLBAR_THICKNESS.into())),
						)
						.push(
							Row::new()
								.push(
									Stack::new()
										.push(
											Board::new(
												&mut state.wstates.board,
												state.interval_input.as_mut().map(|d| &mut d.state),
												&state.sheet,
												&state.frame,
												&state.layout,
												&state.cursor,
												&state.selection,
											)
											.style(theme),
										)
										.push(ScrollView::new(
											&mut state.wstates.scroll_view,
											&state.frame,
											[(true, false), (true, true)],
											|| Message::SetScrolling.into(),
										)),
								)
								.push(y_scrollbar),
						),
				),
		)
		.push(Shortcuts)
		.into()
}
