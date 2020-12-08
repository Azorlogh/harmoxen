#![feature(const_fn_floating_point_arithmetic)]

// use iced_winit::
// use iced::{button, widget, Align, Button, Column, Command, Element, PickList, Row, Text};
use iced::{time, Command, Element, Subscription};
use std::sync::mpsc::Sender;
use std::time::Duration;

pub mod backend;
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

use iced_winit::{application::Application, Color, Mode, Program};

impl Program for State {
	type Renderer = iced_wgpu::Renderer;
	type Message = Message;

	fn update(&mut self, msg: Message) -> Command<Message> {
		self.update(msg)
	}

	fn view(&mut self) -> Element<Message> {
		ui::build(self)
	}
}

impl Application for State {
	type Flags = Sender<backend::Event>;

	fn new(to_server: Self::Flags) -> (State, Command<Self::Message>) {
		let a: State = State::new(to_server);
		(a, Command::none())
	}

	fn title(&self) -> String {
		String::from("Harmoxen on Iced")
	}

	fn subscription(&self) -> Subscription<Self::Message> {
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

	fn mode(&self) -> Mode {
		Mode::Windowed
	}

	fn background_color(&self) -> Color {
		Color::WHITE
	}

	fn scale_factor(&self) -> f64 {
		1.0
	}
}
