use crate::tui_keys::TuiKeys;

pub enum TuiEvent {
    KeyEvent(TuiKeys),
    BufferSizeEvent,
    Other,
    Error,
}
