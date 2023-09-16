use crate::tui_keys::TuiKeys;

pub trait InputInterfaceT {
    fn new() -> Option<Self>
    where
        Self: Sized;
    fn read_keyboard(&self) -> TuiKeys;
    fn read_raw(&self) -> Option<char>;
}
