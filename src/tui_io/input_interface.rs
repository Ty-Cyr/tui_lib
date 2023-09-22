use crate::tui_events::TuiEvents;

pub trait InputInterfaceT {
    fn new() -> Option<Self>
    where
        Self: Sized;
    fn read_parsed(&self) -> TuiEvents;
    fn read_raw(&self) -> Option<char>;
}
