#[derive(Clone, Copy, Debug)]
pub enum TuiMode {
    FullScreen,
    Standard,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    BrightWhite,
    Black,
    BrightBlack,
    Red,
    BrightRed,
    Green,
    BrightGreen,
    Blue,
    BrightBlue,
    Yellow,
    BrightYellow,
    Magenta,
    BrightMagenta,
    Cyan,
    BrightCyan,
    CC256(u8),
    RGB(u8, u8, u8),
    Default,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreeBool {
    True,
    False,
    Default,
}

#[derive(Clone, Copy, Debug)]
pub enum CursorMode {
    Default,
    BlinkingBlock,
    SteadyBlock,
    BlinkingUnderline,
    StedayUnderline,
    BlinkingBar,
    SteadyBar,
    Hidden,
}

#[derive(Clone, Copy, Debug)]
pub enum CursorNav {
    Up(u16),
    Down(u16),
    Forwards(u16),
    Backwards(u16),
    Next(u16),
    Previous(u16),
}

impl CursorNav {
    pub fn get_code(&self) -> String {
        return match self {
            &CursorNav::Up(num) => "\x1b[".to_string() + &num.to_string() + "A",
            &CursorNav::Down(num) => "\x1b[".to_string() + &num.to_string() + "B",
            &CursorNav::Forwards(num) => "\x1b[".to_string() + &num.to_string() + "C",
            &CursorNav::Backwards(num) => "\x1b[".to_string() + &num.to_string() + "D",
            &CursorNav::Next(num) => "\x1b[".to_string() + &num.to_string() + "E",
            &CursorNav::Previous(num) => "\x1b[".to_string() + &num.to_string() + "F",
        };
    }
}
