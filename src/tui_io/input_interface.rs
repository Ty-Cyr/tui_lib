use std::{error::Error, fmt::Debug};

use crate::tui_events::TuiEvents;

pub trait InputInterfaceT: Debug {
    fn new() -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn read_parsed(&self) -> TuiEvents;
    fn read_raw(&self) -> Option<char>;
    fn read_raw_immediate(&self) -> Option<char>;
}
