#[derive(Clone, Copy)]
pub enum TuiMode {
    FullScreen,
    Standard,
}
#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum ThreeBool {
    True,
    False,
    Default,
}

#[derive(Clone, Copy)]
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
