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
    AsciiReadable(char),
    Control(char),
    Other(char),
    Ignore,
    Error,
}
