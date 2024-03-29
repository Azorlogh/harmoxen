#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(min_specialization)]

// use alloc::raw_vec::RawVec;
use baseplug::{Plugin, ProcessContext, WindowOpenResult};
use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use harmoxen::BackendEvent;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use serde::{Deserialize, Serialize};

baseplug::model! {
	#[derive(Debug, Serialize, Deserialize)]
	struct GainModel {
		#[model(min = -90.0, max = 3.0)]
		#[parameter(name = "gain left", unit = "Decibels",
			gradient = "Power(0.15)")]
		gain_left: f32,

		#[model(min = -90.0, max = 3.0)]
		#[parameter(name = "gain right", unit = "Decibels",
			gradient = "Power(0.15)")]
		gain_right: f32,

		#[model(min = -90.0, max = 3.0)]
		#[parameter(name = "gain master", unit = "Decibels",
			gradient = "Power(0.15)")]
		gain_master: f32,
	}
}

impl Default for GainModel {
	fn default() -> Self {
		Self {
			// "gain" is converted from dB to coefficient in the parameter handling code,
			// so in the model here it's a coeff.
			// -0dB == 1.0
			gain_left: 1.0,
			gain_right: 1.0,
			gain_master: 1.0,
		}
	}
}

struct Gain {}

impl Plugin for Gain {
	const NAME: &'static str = "harmoxen";
	const PRODUCT: &'static str = "harmoxen";
	const VENDOR: &'static str = "azorlogh";

	const INPUT_CHANNELS: usize = 2;
	const OUTPUT_CHANNELS: usize = 2;

	type Model = GainModel;

	#[inline]
	fn new(_sample_rate: f32, _model: &GainModel) -> Self {
		Self {}
	}

	#[inline]
	fn process(&mut self, model: &GainModelProcess, ctx: &mut ProcessContext<Self>) {
		let input = &ctx.inputs[0].buffers;
		let output = &mut ctx.outputs[0].buffers;

		for i in 0..ctx.nframes {
			output[0][i] = input[0][i] * model.gain_left[i] * model.gain_master[i];
			output[1][i] = input[1][i] * model.gain_right[i] * model.gain_master[i];
		}
	}
}

struct ValidRawWindowHandle {
	thing: RawWindowHandle,
}

unsafe impl HasRawWindowHandle for ValidRawWindowHandle {
	fn raw_window_handle(&self) -> RawWindowHandle {
		self.thing
	}
}

impl baseplug::PluginUI for Gain {
	type Handle = ();

	fn ui_size() -> (i16, i16) {
		(230, 130)
	}

	fn ui_open(parent: RawWindowHandle) -> WindowOpenResult<Self::Handle> {
		let (to_server, from_frontend) = std::sync::mpsc::channel::<harmoxen::Event>();
		let settings = iced_baseview::Settings {
			window: WindowOpenOptions {
				title: String::from("iced-baseplug-examples harmoxen"),
				size: Size::new(Self::ui_size().0 as f64, Self::ui_size().1 as f64),
				scale: WindowScalePolicy::SystemScaleFactor,
			},
			flags: harmoxen::Flags { to_server },
		};

		iced_baseview::IcedWindow::<harmoxen::State>::open_parented(&ValidRawWindowHandle { thing: parent }, settings);

		Ok(())
	}

	fn ui_close(_handle: Self::Handle) {}

	fn ui_param_notify(
		handle: &Self::Handle,
		param: &'static baseplug::Param<Self, <Self::Model as baseplug::Model<Self>>::Smooth>,
		val: f32,
	) {
	}
}

baseplug::vst2!(Gain, b"tAnE");
