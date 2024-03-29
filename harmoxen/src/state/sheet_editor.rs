use super::UpdateCtx;
use crate::backend;
use crate::data::{
	self,
	layout::Layout,
	sheet::{self, Clipboard, Pitch, Sheet},
	Frame, Frame2, Point, Range,
};
use crate::state::Message as RootMessage;
use crate::widget;
use generational_arena::Index;
use iced::Command;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Default)]
pub struct WStates {
	pub board: widget::sheet_editor::board::State,
	pub scroll_view: widget::scroll_view::State,
	pub marker_editor: widget::sheet_editor::marker_editor::State,
	pub xrange_slider: widget::range_slider::State,
	pub yrange_slider: widget::range_slider::State,
	pub cursor: widget::sheet_editor::cursor::State,
	pub preview: widget::sheet_editor::preview::State,
	pub interval_input: Option<widget::sheet_editor::interval_input::State>,
	pub selection: widget::sheet_editor::selection::State,
}

pub enum PlayingState {
	Stopped,
	Playing(f32), // origin
}

impl PlayingState {
	pub fn is_playing(&self) -> bool {
		match self {
			PlayingState::Stopped => false,
			PlayingState::Playing(_) => true,
		}
	}
}

pub struct State {
	pub wstates: WStates,
	pub frame: Frame2,
	pub is_scrolling: bool,
	pub sheet: Sheet,
	pub cursor: f32,
	pub playing_state: PlayingState,
	pub last_tick: Instant,
	pub layout: Layout,
	pub curr_marker: usize,
	pub selection: HashSet<Index>,
	pub clipboard: Clipboard,
}
impl Default for State {
	fn default() -> State {
		let mut sheet = Sheet::default();
		sheet.add_note(crate::data::sheet::Note::new(crate::data::Point::new(0.0, 8.8), 1.0));
		State {
			wstates: WStates::default(),
			frame: Frame2 {
				x: Frame {
					view: Range(0.0, 4.0),
					bounds: Range(0.0, 5.0),
				},
				y: Frame {
					view: Range(8.0, 9.0),
					bounds: Range(4.0, 14.0),
				},
			},
			is_scrolling: false,
			sheet,
			cursor: 0.0,
			playing_state: PlayingState::Stopped,
			last_tick: Instant::now(),
			layout: Layout::default(),
			curr_marker: 0,
			selection: HashSet::new(),
			clipboard: Clipboard::new(),
		}
	}
}

impl State {
	pub fn update(&mut self, msg: Message, mut ctx: UpdateCtx) -> Command<Message> {
		// println!("{:?}", msg);
		match msg {
			Message::FrameChanged(frame) => {
				self.frame = frame;
			}
			Message::XViewChanged(range) => {
				self.frame.x.view = range;
			}
			Message::YViewChanged(range) => {
				self.frame.y.view = range;
			}
			Message::SetScrolling => {
				self.is_scrolling = true;
			}
			Message::ScrollTick(dt) => {
				self.is_scrolling = widget::scroll_view::tick(&mut self.wstates.scroll_view, &mut self.frame, dt);
			}
			Message::Play => {
				if let PlayingState::Playing(origin) = self.playing_state {
					ctx.to_backend(backend::Event::PlayStop);
					self.cursor = origin;
					self.playing_state = PlayingState::Stopped;
				} else {
					ctx.to_backend(backend::Event::SetTempo(ctx.tempo));
					ctx.to_backend(backend::Event::PlayStart(self.sheet.clone(), self.cursor));
					self.playing_state = PlayingState::Playing(self.cursor);
					self.last_tick = Instant::now();
				}
			}
			Message::CursorTick(now) => {
				self.cursor += now.duration_since(self.last_tick).as_secs_f32() * (ctx.tempo / 60.0);
				let wrap_size = self.sheet.get_size().ceil();
				self.cursor %= wrap_size;
				self.last_tick = now;
			}
			Message::SetCursor(at) => {
				self.cursor = at;
			}
			Message::NoteAdd(note, mov) => {
				let idx = self.sheet.add_note(note);

				if let Pitch::Relative(_, _) = note.pitch {
					self.wstates.interval_input = Some(widget::sheet_editor::interval_input::State::new(&self.sheet, idx));
				} else if mov {
					let rect = note.rect(&self.sheet, 0.0);
					self.wstates.board.set_action_move(idx, rect);
				}
				ctx.project_changed();
			}
			Message::NoteMove(idx, pos) => {
				self.sheet.move_note(idx, pos.x, pos.y);
				ctx.project_changed();
			}
			Message::NoteResize(idx, len) => {
				let note = self.sheet.get_note_mut(idx).expect("tried to resize dead note");
				note.length = len;
				ctx.project_changed();
			}
			Message::NoteDelete(idx) => {
				self.sheet.remove_note(idx);
				self.wstates.interval_input = None;
				ctx.project_changed();
			}
			Message::NoteSetPitch(idx, pitch) => {
				let note = self.sheet.get_note_mut(idx).expect("tried to change pitch of dead note");
				note.pitch = pitch;
				ctx.project_changed();
			}
			Message::OpenIntervalInput(idx) => {
				self.wstates.interval_input = Some(widget::sheet_editor::interval_input::State::new(&self.sheet, idx));
			}
			Message::CloseIntervalInput => {
				self.wstates.interval_input = None;
			}
			Message::AddMarker(at) => {
				let mut new_marker = self.layout.markers[self.curr_marker].clone();
				new_marker.at = at;
				let idx = self.layout.add_marker(new_marker);
				self.curr_marker = idx;
				ctx.project_changed();
			}
			Message::SelectMarker(idx) => {
				self.curr_marker = idx;
				ctx.project_changed();
			}
			Message::MoveMarker(at) => {
				self.curr_marker = self.layout.set_marker_time(self.curr_marker, at);
				ctx.project_changed();
			}
			Message::DeleteMarker(idx) => {
				self.layout.delete_marker(idx);
				ctx.project_changed();
			}
			Message::SelectAll => {
				self.selection = self.sheet.indices.iter().copied().collect();
				ctx.project_changed();
			}
			Message::SetSelection(selection) => {
				self.selection = selection;
				ctx.project_changed();
			}
			Message::Cut => {
				self.clipboard.cut(&mut self.sheet, &mut self.selection);
				ctx.project_changed();
			}
			Message::Copy => {
				self.clipboard.copy(&mut self.sheet, &mut self.selection);
			}
			Message::Paste => {
				self.clipboard.paste(&mut self.sheet, &mut self.selection);
				ctx.project_changed();
			}
			Message::Delete => {
				for idx in self.selection.drain() {
					self.sheet.remove_note(idx);
				}
				ctx.project_changed();
			}
		}
		Command::none()
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	FrameChanged(data::Frame2),
	XViewChanged(data::Range),
	YViewChanged(data::Range),
	SetScrolling,
	ScrollTick(f32),
	Play,
	CursorTick(Instant),
	SetCursor(f32),
	NoteAdd(sheet::Note, bool), // if true: initiate move action
	NoteMove(sheet::Index, Point),
	NoteResize(sheet::Index, f32),
	NoteDelete(sheet::Index),
	NoteSetPitch(sheet::Index, Pitch),
	OpenIntervalInput(sheet::Index),
	CloseIntervalInput,
	AddMarker(f32),
	SelectMarker(usize),
	MoveMarker(f32),
	DeleteMarker(usize),
	SelectAll,
	SetSelection(HashSet<Index>),
	Cut,
	Copy,
	Paste,
	Delete,
}

impl From<Message> for RootMessage {
	fn from(msg: Message) -> RootMessage {
		RootMessage::SheetEditor(msg)
	}
}
