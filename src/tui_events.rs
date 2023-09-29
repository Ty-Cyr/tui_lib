#[derive(Clone, Copy, PartialEq, Debug)]
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
    LeftClick((u16, u16)),
    MiddleClick((u16, u16)),
    RightClick((u16, u16)),
    MouseMove((u16, u16)),
    LeftDrag((u16, u16)),
    MiddleDrag((u16, u16)),
    RightDrag((u16, u16)),
    ScrollUp((u16, u16)),
    ScrollDown((u16, u16)),
    Ignore,
    Error,
}

impl TuiEvents {
    pub fn filter_keyboard_events(self) -> TuiEvents {
        return match self {
            TuiEvents::LeftClick(_) => TuiEvents::Ignore,
            TuiEvents::MiddleClick(_) => TuiEvents::Ignore,
            TuiEvents::RightClick(_) => TuiEvents::Ignore,

            TuiEvents::LeftDrag(_) => TuiEvents::Ignore,
            TuiEvents::MiddleDrag(_) => TuiEvents::Ignore,
            TuiEvents::RightDrag(_) => TuiEvents::Ignore,

            TuiEvents::MouseMove(_) => TuiEvents::Ignore,
            TuiEvents::ScrollUp(_) => TuiEvents::Ignore,
            TuiEvents::ScrollDown(_) => TuiEvents::Ignore,
            _ => self,
        };
    }
}
