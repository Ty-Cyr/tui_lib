use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TuiIoError;

impl Display for TuiIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str("Tui IO Error");
    }
}

impl Error for TuiIoError {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Overflow;

impl Display for Overflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str("Overflow");
    }
}

impl Error for Overflow {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TuiUnexpectedInputError {
    expected: char,
    input: char,
}

impl TuiUnexpectedInputError {
    pub fn new(expected: char, input: char) -> Box<TuiUnexpectedInputError> {
        return TuiUnexpectedInputError {
            expected: expected,
            input: input,
        }
        .into();
    }
}

impl Display for TuiUnexpectedInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(
            &("Tui Unexpected Input: ".to_owned()
                + &self.input.to_string()
                + "\nExpected: "
                + &self.expected.to_string()),
        );
    }
}

impl Error for TuiUnexpectedInputError {}
