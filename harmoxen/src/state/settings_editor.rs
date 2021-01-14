use derive_more::Display;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct State {
	pub backend: Backend,
	pub mpe_port_names: Arc<Vec<String>>,
}

#[derive(Clone, Display)]
pub enum Backend {
	#[display(fmt = "Synth")]
	Audio,
	#[display(fmt = "MPE")]
	MPE { port: usize },
}
impl Default for Backend {
	fn default() -> Backend {
		Backend::Audio
	}
}
