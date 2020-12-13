use crate::data::{Frame2, Layout, Point};
use crate::state::{sheet_editor::Message, Message as RootMessage};
use crate::util::coord::Coord;
use iced_graphics::{
	triangle::{Mesh2D, Vertex2D},
	Backend, Defaults, Primitive, Renderer,
};
use iced_native::{
	event, keyboard, layout as iced_layout, mouse, overlay, Clipboard, Element, Event, Hasher, Layout as IcedLayout, Length,
	Rectangle, Vector, Widget,
};

use crate::widget::common::{context_menu, ContextMenu};

#[derive(PartialEq)]
pub enum Action {
	Idle,
	Move,
	Context,
}

pub struct State {
	action: Action,
	ctrl: bool,
	action_effective: bool,
	context_menu: context_menu::State<RootMessage>,
	context_pos: Option<iced::Point>,
}

impl Default for State {
	fn default() -> State {
		State {
			action: Action::Idle,
			ctrl: false,
			action_effective: false,
			context_menu: context_menu::State::default(),
			context_pos: None,
		}
	}
}

mod style;
pub use style::{Style, StyleSheet};

pub struct MarkerEditor<'a> {
	state: &'a mut State,
	layout: &'a Layout,
	selected_marker: usize,
	frame: Frame2,
	style: Box<dyn StyleSheet>,
}

impl<'a> MarkerEditor<'a> {
	pub fn new(state: &'a mut State, frame: Frame2, layout: &'a Layout, selected_marker: usize) -> Self {
		Self {
			state,
			frame,
			layout,
			selected_marker,
			style: Default::default(),
		}
	}

	pub fn style(mut self, style: impl Into<Box<dyn StyleSheet>>) -> Self {
		self.style = style.into();
		self
	}
}

pub fn get_hover(x: f32, coord: Coord, layout: &Layout) -> Option<usize> {
	let extent = coord.to_board_w(8.0);
	let offset = coord.to_board_w(4.0);
	for (i, marker) in layout.markers.iter().enumerate() {
		if x > marker.at - offset && x < marker.at + extent + offset {
			return Some(i);
		}
	}
	None
}

impl<'a, B> Widget<RootMessage, Renderer<B>> for MarkerEditor<'a>
where
	B: Backend + iced_graphics::backend::Text + 'static,
{
	fn width(&self) -> Length {
		Length::Fill
	}

	fn height(&self) -> Length {
		Length::Fill
	}

	fn layout(&self, _renderer: &Renderer<B>, limits: &iced_layout::Limits) -> iced_layout::Node {
		iced_layout::Node::new(limits.max())
	}

	fn hash_layout(&self, _action: &mut Hasher) {}

	fn on_event(
		&mut self,
		event: Event,
		layout: iced_native::Layout,
		cursor_position: iced::Point,
		messages: &mut Vec<RootMessage>,
		_renderer: &Renderer<B>,
		_clipboard: Option<&dyn Clipboard>,
	) -> event::Status {
		let lbounds = layout.bounds();
		let lposition: Point = lbounds.position().into();
		let mouse_pos = Into::<Point>::into(cursor_position) - lposition.to_vec2();
		let coord = Coord::new(self.frame, lbounds.size());

		let captured = match event {
			Event::Keyboard(keyboard::Event::ModifiersChanged(mods)) => {
				self.state.ctrl = mods.control;
				false
			}
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
				if let Action::Context = self.state.action {
					self.state.action = Action::Idle;
					true
				} else if lbounds.contains(cursor_position) {
					let x = coord.to_board_x(mouse_pos.x);
					if let Some(idx) = get_hover(x, coord, self.layout) {
						messages.push(Message::SelectMarker(idx).into());
						self.state.action = Action::Move;
						true
					} else {
						false
					}
				} else {
					false
				}
			}
			Event::Mouse(mouse::Event::CursorMoved { .. }) => {
				let mut time = coord.to_board_x(mouse_pos.x).max(0.0);
				let idx = self.selected_marker;
				if self.state.action == Action::Move && self.selected_marker != 0 {
					if !self.state.ctrl {
						time = self.layout.quantize_time_exclude(time, false, idx);
					}
					self.state.action_effective = true;
					messages.push(Message::MoveMarker(time).into());
				}
				false
			}
			Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
				self.state.action = Action::Idle;
				false
			}
			Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
				self.state.action = Action::Idle;
				if lbounds.contains(cursor_position) {
					let at = coord.to_board_x(mouse_pos.x);
					match get_hover(at, coord, self.layout) {
						Some(idx) => {
							let mut items = vec![context_menu::Item::new("Edit Layout", RootMessage::OpenLayout)];
							if idx != 0 {
								items.push(context_menu::Item::new("Delete marker", Message::DeleteMarker(idx).into()));
							}
							self.state.context_menu = context_menu::State::new(items);
							self.state.action = Action::Context;
							self.state.context_pos = Some(cursor_position);
						}
						_ => {
							self.state.context_menu = context_menu::State::new(vec![context_menu::Item::new(
								"Create Marker",
								Message::AddMarker(at).into(),
							)]);
							self.state.action = Action::Context;
							self.state.context_pos = Some(cursor_position);
						}
					}
					true
				} else {
					false
				}
			}
			_ => false,
		};
		if captured {
			event::Status::Captured
		} else {
			event::Status::Ignored
		}
	}

	fn draw(
		&self,
		_renderer: &mut Renderer<B>,
		_defaults: &Defaults,
		layout: IcedLayout,
		_cursor_position: iced::Point,
		_viewport: &Rectangle,
	) -> (Primitive, mouse::Interaction) {
		let bounds = layout.bounds();
		let coord = Coord::new(self.frame, layout.bounds().size());

		let size = bounds.width.min(bounds.height);

		const AR: f32 = 0.5; // marker aspect ratio

		let mut cursors = vec![];
		for i in 0..self.layout.markers.len() {
			let color = if self.selected_marker == i {
				[1.0, 1.0, 1.0, 1.0]
			} else {
				[0.3, 0.3, 0.3, 1.0]
			};
			let marker = &self.layout.markers[i];
			let s_pos = coord.to_screen_x(marker.at);
			cursors.push(Primitive::Translate {
				translation: Vector::new(bounds.x + s_pos, bounds.y),
				content: Box::new(Primitive::Mesh2D {
					size: bounds.size(),
					buffers: Mesh2D {
						vertices: vec![
							Vertex2D {
								position: [0.0, 0.0],
								color,
							},
							Vertex2D {
								position: [size * AR, 0.0],
								color,
							},
							Vertex2D {
								position: [size * AR / 2.0, size / 2.0],
								color,
							},
							Vertex2D {
								position: [size * AR, size],
								color,
							},
							Vertex2D {
								position: [0.0, size],
								color,
							},
						],
						indices: vec![
							0, 1, 4, // |-/
							2, 3, 4, //   /_\
						],
					},
				}),
			})
		}

		let cursors_primitives = Primitive::Group { primitives: cursors };
		(
			Primitive::Clip {
				bounds,
				offset: Vector::new(0, 0),
				content: Box::new(cursors_primitives),
			},
			mouse::Interaction::Idle,
		)
	}

	fn overlay(&mut self, _layout: iced_layout::Layout) -> Option<overlay::Element<RootMessage, Renderer<B>>> {
		if let Action::Context = self.state.action {
			Some(
				ContextMenu::new(&mut self.state.context_menu)
					.padding(4)
					.style(self.style.menu())
					.overlay(self.state.context_pos.unwrap()),
			)
		} else {
			None
		}
	}
}

impl<'a, B> Into<Element<'a, RootMessage, Renderer<B>>> for MarkerEditor<'a>
where
	RootMessage: 'a + Clone,
	B: Backend + iced_graphics::backend::Text + 'static,
{
	fn into(self) -> Element<'a, RootMessage, Renderer<B>> {
		Element::new(self)
	}
}
