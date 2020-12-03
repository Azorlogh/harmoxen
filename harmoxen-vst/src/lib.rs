// #![feature(generator_trait)]

// use vst::{
// 	api::{Events, Supported},
// 	editor::Editor,
// 	event::Event,
// 	plugin::{CanDo, Category, Info, Plugin},
// };

// #[derive(Default)]
// struct Harmoxen;

// impl Plugin for Harmoxen {
// 	fn get_info(&self) -> Info {
// 		Info {
// 			name: "Harmoxen".to_string(),
// 			unique_id: 969809320,
// 			inputs: 0,
// 			outputs: 2,
// 			category: Category::Synth,
// 			parameters: 1,
// 			..Default::default()
// 		}
// 	}

// 	fn can_do(&self, can_do: CanDo) -> Supported {
// 		match can_do {
// 			CanDo::SendMidiEvent => Supported::Yes,
// 			_ => Supported::Maybe,
// 		}
// 	}

// 	// fn process(&mut self, buffer: &mut AudioBuffer<f32>) {}

// 	fn process_events(&mut self, events: &Events) {
// 		for event in events.events() {
// 			match event {
// 				Event::Midi(ev) => match ev.data[0] {
// 					_ => {}
// 				},
// 				_ => {}
// 			}
// 		}
// 	}

// 	fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
// 		log_panics::init();
// 		Some(Box::new(GUIWrapper { inner: None }))
// 	}
// }

// use std::ffi::c_void;

// mod gui;
// use gui::GUI;

// const WIDTH: u32 = 600;
// const HEIGHT: u32 = 300;

// struct GUIWrapper {
// 	inner: Option<GUI>,
// }

// impl Editor for GUIWrapper {
// 	fn size(&self) -> (i32, i32) {
// 		log::info!("GUI size");
// 		(WIDTH as i32, HEIGHT as i32)
// 	}

// 	fn position(&self) -> (i32, i32) {
// 		log::info!("GUI position");
// 		(0, 0)
// 	}

// 	fn idle(&mut self) {
// 		log::info!("GUI idle");

// 		if let Some(inner) = self.inner.as_mut() {
// 			log::info!("GUI idle run");
// 			if let std::ops::GeneratorState::Complete(_) =
// 				std::ops::Generator::resume(std::pin::Pin::new(&mut inner.gen), ())
// 			{
// 				self.inner = None;
// 			}
// 		}
// 	}

// 	fn close(&mut self) {
// 		log::info!("GUI close");
// 		self.inner = None;
// 		log::info!("GUI closed");
// 	}

// 	fn open(&mut self, parent: *mut c_void) -> bool {
// 		log::info!("GUI open");
// 		let gui: GUI = todo!();
// 		self.inner = Some(gui);

// 		log::info!("GUI opened");
// 		true
// 	}

// 	fn is_open(&mut self) -> bool {
// 		log::info!("GUI is_open");
// 		self.inner.is_some()
// 	}
// }
