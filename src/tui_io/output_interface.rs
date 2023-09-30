use crate::tui_errors::CError;

pub trait OutputInterfaceT {
    fn get_size(&self) -> Result<(u16, u16), CError>;
}
