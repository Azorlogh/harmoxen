use iced_graphics::{backend, Backend, Defaults, Primitive, Renderer};
use iced_native::{event, layout, mouse, Clipboard, Element, Event, Hasher, Layout as IcedLayout, Length, Rectangle, Widget};

#[derive(Default)]
pub struct State<ChildState, Data> {
	child_state: ChildState,
	child_data: Option<Data>,
}

pub struct Parse<'a, Data, Message, B>
where
	B: Backend + backend::Text + 'static,
{
	child_data: &'a mut Data,
	on_change: Box<dyn Fn(&Data) -> Option<Message>>,
	child: Element<'a, Data, Renderer<B>>,
}

impl<'a, Data, Message, B> Parse<'a, Data, Message, B>
where
	Data: Clone,
	B: Backend + backend::Text + 'static,
{
	pub fn new<ChildState, ChildBuilder, C>(
		state: &'a mut State<ChildState, Data>,
		child: ChildBuilder,
		initial: Data,
		on_change: impl Fn(&Data) -> Option<Message> + 'static,
	) -> Self
	where
		ChildBuilder: FnOnce(&'a mut ChildState, Data) -> C + 'static,
		C: Into<Element<'a, Data, Renderer<B>>>,
	{
		if let None = state.child_data {
			state.child_data = Some(initial);
		}
		let child_data = state.child_data.as_mut().unwrap();
		let child_data_clone = child_data.clone();
		Self {
			child_data,
			child: child(&mut state.child_state, child_data_clone).into(),
			on_change: Box::new(on_change),
		}
	}
}

impl<'a, Data, Message, B> Widget<Message, Renderer<B>> for Parse<'a, Data, Message, B>
where
	B: Backend + backend::Text + 'static,
{
	fn width(&self) -> Length {
		Length::Fill
	}

	fn height(&self) -> Length {
		Length::Fill
	}

	fn layout(&self, renderer: &Renderer<B>, limits: &layout::Limits) -> layout::Node {
		let child_layout = self.child.layout(renderer, &limits.clone());
		let size = limits.resolve(child_layout.size());
		layout::Node::with_children(size, vec![child_layout])
	}

	fn hash_layout(&self, state: &mut Hasher) {
		use std::{any::TypeId, hash::Hash};
		struct Marker;
		TypeId::of::<Marker>().hash(state);
	}

	fn on_event(
		&mut self,
		event: Event,
		layout: iced_native::Layout,
		cursor_position: iced::Point,
		messages: &mut Vec<Message>,
		renderer: &Renderer<B>,
		clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		{
			let layout = layout.children().next().unwrap();
			let mut msgs = vec![];
			let status = self
				.child
				.on_event(event, layout, cursor_position, &mut msgs, renderer, clipboard);

			if let Some(data) = msgs.pop() {
				if let Some(message) = (self.on_change)(&data) {
					messages.push(message);
				}
				*self.child_data = data;
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
		self.child.draw(
			renderer,
			defaults,
			layout.children().next().unwrap(),
			cursor_position,
			viewport,
		)
	}
}

impl<'a, Data, Message, B> Into<Element<'a, Message, Renderer<B>>> for Parse<'a, Data, Message, B>
where
	Data: 'static,
	Message: 'a + Clone,
	B: Backend + iced_graphics::backend::Text + 'static,
{
	fn into(self) -> Element<'a, Message, Renderer<B>> {
		Element::new(self)
	}
}
