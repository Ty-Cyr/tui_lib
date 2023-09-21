#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorPositionError {
    IOError,
    OverflowError,
    TuiUnexpectedInputError(char, char),
}
