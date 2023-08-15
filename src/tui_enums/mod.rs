#[derive(Clone, Copy)]
pub enum TuiMode {
    FullScreen,
    Standard,
}
#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue,
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
pub enum KeyEvent {
    KeyUp,
    KeyDown,
}
