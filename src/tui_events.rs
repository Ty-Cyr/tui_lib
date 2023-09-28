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
    LeftClick(u16, u16),
    MidddleClick(u16, u16),
    RightClick(u16, u16),
    MouseMove(u16, u16),
    LeftDrag(u16, u16),
    MiddleDrag(u16, u16),
    RightDrag(u16, u16),
    ScrollUp(u16, u16),
    ScrollDown(u16, u16),
    Ignore,
    Error,
}

impl TuiEvents {
    pub fn filter_no_mouse(self) -> TuiEvents {
        return match self {
            TuiEvents::LeftClick(_, _) => TuiEvents::Ignore,
            TuiEvents::MidddleClick(_, _) => TuiEvents::Ignore,
            TuiEvents::RightClick(_, _) => TuiEvents::Ignore,

            TuiEvents::LeftDrag(_, _) => TuiEvents::Ignore,
            TuiEvents::MiddleDrag(_, _) => TuiEvents::Ignore,
            TuiEvents::RightDrag(_, _) => TuiEvents::Ignore,

            TuiEvents::MouseMove(_, _) => TuiEvents::Ignore,
            TuiEvents::ScrollUp(_, _) => TuiEvents::Ignore,
            TuiEvents::ScrollDown(_, _) => TuiEvents::Ignore,
            _ => self,
        };
    }
}
