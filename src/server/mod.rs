use crate::data::{icp, sheet::*};

#[derive(Debug)]
pub enum Event {
	SetTempo(f64),
	PlayStart(Sheet, f64),
	PlayStop,
	SheetChanged(Sheet),
	ICP(icp::Event),
	Shutdown,
}

pub mod audio;
pub mod midi;
