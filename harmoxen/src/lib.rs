#![feature(const_fn_floating_point_arithmetic)]

// use iced_winit::
// use iced::{button, widget, Align, Button, Column, Command, Element, PickList, Row, Text};
use iced::{time, Command, Element, Subscription};
use std::sync::mpsc::Sender;
use std::time::Duration;

pub mod backend;
pub mod consts;
pub mod data;
pub mod state;
mod style;
mod ui;
mod util;
pub mod widget;

pub use backend::Event as BackendEvent;
pub use state::State;
pub use style::Theme;

pub use state::Message;

use iced_baseview::{Application, Color, WindowSubs};

#[derive(Debug, Clone)]
pub enum BackendId {
	Audio,
	Midi(usize),
}
impl Default for BackendId {
	fn default() -> Self {
		BackendId::Audio
	}
}

pub trait Backend {
	fn send(&mut self, evt: BackendEvent);
}

pub enum Event {
	ChangeBackend(BackendId),
	ToBackend(backend::Event),
}

pub struct Flags {
	pub to_server: Sender<Event>,
}

impl Application for State {
	type Flags = Flags;
	type Message = Message;
	type Executor = iced_futures::executor::Tokio;

	fn new(flags: Self::Flags) -> (State, Command<Self::Message>) {
		(State::new(flags.to_server), Command::none())
	}

	fn update(&mut self, msg: Message) -> Command<Message> {
		self.update(msg)
	}

	fn view(&mut self) -> Element<Message> {
		ui::build(self)
	}

	fn subscription(&self, _: &mut WindowSubs<Self::Message>) -> Subscription<Self::Message> {
		let is_playing = self.sheet_editor.playing_state.is_playing();
		let is_scrolling = self.sheet_editor.is_scrolling;
		let mut subscriptions = vec![];
		if is_playing {
			subscriptions.push(
				time::every(Duration::from_millis(16))
					.map(|_| state::sheet_editor::Message::CursorTick(std::time::Instant::now()).into()),
			)
		}
		if is_scrolling {
			subscriptions
				.push(time::every(Duration::from_millis(16)).map(|_| state::sheet_editor::Message::ScrollTick(16.0).into()))
		}
		Subscription::batch(subscriptions)
	}

	fn background_color(&self) -> Color {
		Color::WHITE
	}

	fn renderer_settings() -> iced_baseview::renderer::Settings {
		iced_baseview::renderer::Settings {
			default_font: None,
			default_text_size: 20,
			antialiasing: Some(iced_baseview::renderer::Antialiasing::MSAAx4),
			..iced_baseview::renderer::Settings::default()
		}
	}
}
