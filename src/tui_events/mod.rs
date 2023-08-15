use crate::tui_keys::TuiKeys;

pub enum TuiEvent {
    KeyEvent(bool, TuiKeys, u16),
    BufferSizeEvent,
    Other,
    Error,
}
