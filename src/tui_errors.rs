use std::{error::Error, fmt::Display};

#[derive(Clone, Debug)]
pub struct CError {
    pub error_string: String,
}
impl From<String> for CError {
    fn from(value: String) -> CError {
        return CError {
            error_string: value,
        };
    }
}
impl From<&str> for CError {
    fn from(value: &str) -> CError {
        return CError {
            error_string: value.into(),
        };
    }
}

impl Display for CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str(&self.error_string);
    }
}
impl Error for CError {}

#[derive(Clone, Copy, Debug)]
pub struct IOError {}

impl Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str("IO Error");
    }
}
impl Error for IOError {}
#[derive(Clone, Copy, Debug)]
pub struct OverflowError {}

impl Display for OverflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_str("Overflow");
    }
}
impl Error for OverflowError {}
#[derive(Clone, Copy, Debug)]
pub struct TuiUnexpectedInputError {
    pub expected: char,
    pub recieved: char,
}

impl Display for TuiUnexpectedInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = "Expected: ".to_string()
            + &self.expected.to_string()
            + "\nGot: "
            + &self.recieved.to_string();
        return f.write_str(&result);
    }
}
impl Error for TuiUnexpectedInputError {}
