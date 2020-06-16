//! Instrument control protocol
//! Interface between the piano roll and the audio server

pub type NoteId = usize;

#[derive(Debug, Clone, Copy)]
pub struct Note {
	pub id: NoteId,
	pub freq: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
	NotePlay(Note),
	NoteStop(NoteId),
	NoteStopAll,
	NoteChangeFreq(NoteId, f64),
}
