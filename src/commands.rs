use crate::icp;
use crate::util::Frame2;
use druid::Selector;
use generational_arena::Index;

pub const PLAY_START: Selector = Selector::new("play-start");
pub const PLAY_STOP: Selector = Selector::new("play-stop");
pub const SHEET_CHANGED: Selector = Selector::new("sheet-changed");
pub const ICP: Selector<icp::Event> = Selector::new("icp");

pub const TEMPO_CHANGED: Selector<f64> = Selector::new("tempo-changed");

pub const SCROLL_VIEW_MOVE: Selector<Frame2> = Selector::new("scroll-view-move");

// editor
pub const SHEET_EDITOR_REDRAW: Selector = Selector::new("sheet-editor-redraw");
pub const SHEET_EDITOR_ADD_RELATIVE_NOTE: Selector<(Index, f64)> = Selector::new("sheet-editor-add-relative-note");
pub const SHEET_EDITOR_DELETE_NOTE: Selector<Index> = Selector::new("sheet-editor-delete-note");

// marker editor
pub const MARKER_ADD: Selector<f64> = Selector::new("marker-add");
pub const MARKER_DELETE: Selector<usize> = Selector::new("marker-delete");

// layout
pub const OPEN_LAYOUT_EDITOR: Selector = Selector::new("open-layout-editor");
pub const LAYOUT_APPLY: Selector = Selector::new("layout-apply");
pub const LAYOUT_CHANGED: Selector = Selector::new("layout-changed");

// cursor
pub const CURSOR_ADVANCE: Selector<f64> = Selector::new("cursor-advance");
