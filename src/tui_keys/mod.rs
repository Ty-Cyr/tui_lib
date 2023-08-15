#[derive(Clone, Copy)]
pub enum TuiKeys {
    Enter,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    Escape,
    Backspace,
    Delete,
    Tab,
    Space,
    Shift,
    Other(char),
}
