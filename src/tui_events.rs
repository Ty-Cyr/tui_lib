#[derive(Clone, Copy, PartialEq)]
pub enum TuiEvents {
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

impl TuiEvents {
    pub fn eq_or_none(&self, expected: &TuiEvents) -> Option<()> {
        if self == expected {
            return Some(());
        } else {
            return None;
        }
    }
    pub fn get_digit(&self) -> Option<u8> {
        if let TuiEvents::AsciiReadable(value) = self {
            match value.clone() as u8 {
                0x30..=0x39 => return Some((value.clone() as u8) - 0x30),
                _ => return None,
            }
        }
        return None;
    }
}
