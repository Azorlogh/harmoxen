use crate::data::{
	sheet::{Index, Interval, Pitch},
	Frame2, Point, Sheet,
};
use crate::state::{sheet_editor::Message, Message as RootMessage};
use crate::util::coord::Coord;
use iced_graphics::{backend, Backend, Defaults, Primitive, Renderer};
use iced_native::widget::*;
use iced_native::{
	event, layout as iced_layout, mouse, Clipboard, Element, Event, Hasher, Layout as IcedLayout, Length, Rectangle, Widget,
};

pub struct State {
	internal: InternalState,
	text_input: text_input::State,
}
impl State {
	pub fn new(sheet: &Sheet, idx: Index) -> Self {
		let note = sheet.get_note(idx).expect("tried to input interval for dead note");
		match note.pitch {
			Pitch::Relative(_, interval) => Self {
				internal: InternalState {
					text: interval.to_string(),
					idx,
				},
				text_input: Default::default(),
			},
			Pitch::Absolute(_) => panic!("tried to input interval for absolute note"),
		}
	}
}

struct InternalState {
	text: String,
	idx: Index,
}

#[derive(Clone)]
struct IntervalChange(String);

pub struct IntervalInput<'a, B>
where
	B: Backend + backend::Text + 'static,
{
	state: &'a mut InternalState,
	text_input: TextInput<'a, IntervalChange, Renderer<B>>,
	sheet: &'a Sheet,
	frame: &'a Frame2,
}

impl<'a, B> IntervalInput<'a, B>
where
	B: Backend + backend::Text + 'static,
{
	pub fn new(state: &'a mut State, sheet: &'a Sheet, frame: &'a Frame2) -> Self {
		let internal = &mut state.internal;
		let text_input = TextInput::new(&mut state.text_input, "2/1", &internal.text, |v| IntervalChange(v));
		Self {
			state: internal,
			text_input,
			sheet,
			frame,
		}
	}
}

impl<'a, B> Widget<RootMessage, Renderer<B>> for IntervalInput<'a, B>
where
	B: Backend + backend::Text + 'static,
{
	fn width(&self) -> Length {
		Length::Fill
	}

	fn height(&self) -> Length {
		Length::Fill
	}

	fn layout(&self, renderer: &Renderer<B>, limits: &iced_layout::Limits) -> iced_layout::Node {
		let mut children = vec![];
		let state = &*self.state;
		let sheet = self.sheet;
		let frame = *self.frame;
		let note = sheet.get_note(state.idx).unwrap();
		if let Pitch::Relative(root, _) = note.pitch {
			let coord = Coord::new(frame, limits.max());
			let root = sheet.get_note(root).unwrap();
			let position = Point::new(note.start, (sheet.get_y(note.pitch) + sheet.get_y(root.pitch)) / 2.0);
			let screen_pos = coord.to_screen_p(position);
			let mut node = self.text_input.layout(renderer, &iced_layout::Limits::NONE.max_width(100));
			node.move_to(screen_pos.into());
			children.push(node)
		}
		iced_layout::Node::with_children(limits.max(), children)
	}

	fn hash_layout(&self, state: &mut Hasher) {
		use std::{any::TypeId, hash::Hash};
		struct Marker;
		TypeId::of::<Marker>().hash(state);

		let note = self.sheet.get_note(self.state.idx).unwrap();
		self.sheet.get_y(note.pitch).to_bits().hash(state);
		if let Pitch::Relative(root_idx, _) = note.pitch {
			let root = self.sheet.get_note(root_idx).unwrap();
			self.sheet.get_y(root.pitch).to_bits().hash(state);
		}

		self.text_input.hash_layout(state);
	}

	fn on_event(
		&mut self,
		event: Event,
		layout: iced_native::Layout,
		cursor_position: iced::Point,
		messages: &mut Vec<RootMessage>,
		renderer: &Renderer<B>,
		clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let state = &mut self.state;
		{
			let layout = layout.children().next().unwrap();
			let mut msgs = vec![];
			let status = self
				.text_input
				.on_event(event, layout, cursor_position, &mut msgs, renderer, clipboard);

			if let Some(IntervalChange(text)) = msgs.pop() {
				state.text = text.clone();
				if let Ok(value) = text.parse::<Interval>() {
					let note = self.sheet.get_note(state.idx).expect("tried to change interval of dead note");
					if let Pitch::Relative(root, _) = note.pitch {
						messages.push(Message::NoteSetPitch(state.idx, Pitch::Relative(root, value)).into());
					} else {
						panic!("tried to change interval of absolute note");
					}
				}
			}
			status
		}
	}

	fn draw(
		&self,
		renderer: &mut Renderer<B>,
		defaults: &Defaults,
		layout: IcedLayout,
		cursor_position: iced::Point,
		viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		Widget::draw(
			&self.text_input,
			renderer,
			defaults,
			layout.children().next().unwrap(),
			cursor_position,
			viewport,
		)
	}
}

impl<'a, B> Into<Element<'a, RootMessage, Renderer<B>>> for IntervalInput<'a, B>
where
	RootMessage: 'a + Clone,
	B: Backend + iced_graphics::backend::Text + 'static,
{
	fn into(self) -> Element<'a, RootMessage, Renderer<B>> {
		Element::new(self)
	}
}
