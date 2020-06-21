use crate::data::icp;
use crate::util::Frame2;
use druid::{Selector, WidgetId};

// project
pub const PROJECT_NEW: Selector = Selector::new("project.new");
pub const PROJECT_OPEN: Selector = Selector::new("project.open");
pub const PROJECT_SAVE: Selector = Selector::new("project.save");
pub const PROJECT_SAVE_AS: Selector = Selector::new("project.save-as");

// history
pub const HISTORY_SAVE: Selector = Selector::new("history.save");
pub const HISTORY_UNDO: Selector = Selector::new("history.undo");
pub const HISTORY_REDO: Selector = Selector::new("history.redo");

//
pub const OPEN_LAYOUT_EDITOR: Selector = Selector::new("editor.layout.open");
pub const LAYOUT_APPLY: Selector = Selector::new("layout.apply");

pub const OPEN_SETTINGS: Selector = Selector::new("editor.settings");
pub const SETTINGS_APPLY: Selector = Selector::new("settings-apply");

pub const SHEET_CHANGED: Selector = Selector::new("sheet-changed");
pub const LAYOUT_CHANGED: Selector = Selector::new("layout-changed");

pub const REDRAW: Selector = Selector::new("redraw");

pub const BACKEND_SET_AUDIO: Selector = Selector::new("backend.set-audio");
pub const BACKEND_SET_MPE: Selector<usize> = Selector::new("backend.set-mpe");
pub const BACKEND_MPE_REQUEST_PORTS: Selector<WidgetId> = Selector::new("backend.mpe.request-ports");

pub const PLAY_START: Selector = Selector::new("play-start");
pub const PLAY_STOP: Selector = Selector::new("play-stop");
pub const ICP: Selector<icp::Event> = Selector::new("icp");
pub const TEMPO_CHANGED: Selector<f64> = Selector::new("tempo-changed");

// board
pub const SCROLL_VIEW_MOVE: Selector<Frame2> = Selector::new("scroll-view-move");

// marker editor
pub const MARKER_ADD: Selector<f64> = Selector::new("marker-add");
pub const MARKER_DELETE: Selector<usize> = Selector::new("marker-delete");

// cursor
pub const CURSOR_ADVANCE: Selector<f64> = Selector::new("cursor-advance");
