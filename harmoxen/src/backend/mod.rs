use crate::data::{icp, sheet::*};

#[derive(Debug, Clone)]
pub enum Event {
	SetTempo(f32),
	PlayStart(Sheet, f32),
	PlayStop,
	SheetChanged(Sheet),
	ICP(icp::Event),
	Shutdown,
}

impl From<icp::Event> for Event {
	fn from(evt: icp::Event) -> Event {
		Event::ICP(evt)
	}
}
