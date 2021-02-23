#![feature(const_fn_floating_point_arithmetic)]

// use iced_winit::
// use iced::{button, widget, Align, Button, Column, Command, Element, PickList, Row, Text};
use iced::{time, Command, Element, Subscription};
use std::sync::mpsc::Sender;
use std::time::Duration;

pub mod backend;
pub mod consts;
pub mod data;
mod state;
mod style;
mod ui;
mod util;
mod widget;

pub use backend::Event as BackendEvent;
pub use state::State;
pub use style::Theme;

pub use state::Message;

use iced_baseview::{Application, Color, WindowSubs};

impl Application for State {
	type Flags = Sender<backend::Event>;
	type Message = Message;
	type Executor = iced_futures::executor::Tokio;

	fn new(to_server: Self::Flags) -> (State, Command<Self::Message>) {
		let a: State = State::new(to_server);
		(a, Command::none())
	}

	fn update(&mut self, msg: Message) -> Command<Message> {
		self.update(msg)
	}

	fn view(&mut self) -> Element<Message> {
		ui::build(self)
	}

	fn subscription(&self, _: &mut WindowSubs<Self::Message>) -> Subscription<Self::Message> {
		let is_playing = self.sheet_editor.is_playing;
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
}
