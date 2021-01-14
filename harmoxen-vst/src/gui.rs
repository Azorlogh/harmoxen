pub struct GUI {
	pub gen: Box<dyn std::marker::Unpin + std::ops::Generator<Yield = (), Return = ()>>,
}

impl GUI {
	fn new(parent: HWND, params: Arc<WhisperParameters>) -> Self {
		let gen = Box::new(move || {
			use iced_wgpu::{wgpu, Backend, Renderer, Settings, Viewport};

			// use iced_winit::{application, executor, Settings};
			use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
			use iced_baseview::*;

			let settings = Settings {
				window: WindowOpenOptions {
					title: "Harmoxen".into(),
					size: Size::new(500.0, 300.0),
					scale: WindowScalePolicy::SystemScaleFactor,
				},
				flags: to_server,
			};
			iced_baseview::IcedWindow::<harmoxen::State>::open_blocking(settings);

			use winit::{
				event::{Event, ModifiersState, WindowEvent},
				event_loop::{ControlFlow, EventLoop},
				platform::desktop::EventLoopExtDesktop,
			};
			let mut event_loop = EventLoop::new();

			let window = winit::window::WindowBuilder::new()
				.with_decorations(false)
				.with_inner_size(winit::dpi::PhysicalSize {
					width: 1280,
					height: 720,
				})
				.build(&event_loop)
				.unwrap();

			let physical_size = window.inner_size();
			let mut viewport =
				Viewport::with_physical_size(Size::new(physical_size.width, physical_size.height), window.scale_factor());
			let modifiers = ModifiersState::default();
		});
		gen
	}
}
