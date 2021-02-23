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
use std::time::Instant;
use std::{cell::RefCell, collections::HashSet, sync::Arc};

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

pub struct State {
	pub wstates: WStates,
	pub frame: Frame2,
	pub is_scrolling: bool,
	pub sheet: Sheet,
	pub cursor: f32,
	pub is_playing: bool,
	pub last_tick: Instant,
	pub layout: Layout,
	pub tempo: f32,
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
			is_playing: false,
			last_tick: Instant::now(),
			layout: Layout::default(),
			tempo: 120.0,
			curr_marker: 0,
			selection: HashSet::new(),
			clipboard: Clipboard::new(),
		}
	}
}

use std::sync::mpsc::Sender;

impl State {
	pub fn update(&mut self, msg: Message, to_backend: &mut Sender<backend::Event>) -> Command<Message> {
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
				if self.is_playing {
					to_backend.send(backend::Event::PlayStop).ok();
					self.cursor = 0.0;
				} else {
					to_backend.send(backend::Event::SetTempo(self.tempo)).unwrap();
					to_backend
						.send(backend::Event::PlayStart(self.sheet.clone(), self.cursor))
						.ok();
					self.last_tick = Instant::now();
				}
				self.is_playing ^= true;
			}
			Message::CursorTick(now) => {
				self.cursor += now.duration_since(self.last_tick).as_secs_f32() * (self.tempo / 60.0);
				self.cursor %= self.sheet.get_size();
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
			}
			Message::NoteMove(idx, pos) => {
				self.sheet.move_note(idx, pos.x, pos.y);
			}
			Message::NoteResize(idx, len) => {
				let note = self.sheet.get_note_mut(idx).expect("tried to resize dead note");
				note.length = len;
			}
			Message::NoteDelete(idx) => {
				self.sheet.remove_note(idx);
				self.wstates.interval_input = None;
			}
			Message::NoteSetPitch(idx, pitch) => {
				let note = self.sheet.get_note_mut(idx).expect("tried to change pitch of dead note");
				note.pitch = pitch;
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
			}
			Message::SelectMarker(idx) => {
				self.curr_marker = idx;
			}
			Message::MoveMarker(at) => {
				self.curr_marker = self.layout.set_marker_time(self.curr_marker, at);
			}
			Message::DeleteMarker(idx) => {
				self.layout.delete_marker(idx);
			}
			Message::SetSelection(selection) => {
				self.selection = selection;
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
	SetSelection(HashSet<Index>),
}

impl From<Message> for RootMessage {
	fn from(msg: Message) -> RootMessage {
		RootMessage::SheetEditor(msg)
	}
}
