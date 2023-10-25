use std::fmt::Debug;

use crate::tui_errors::CError;

pub trait OutputInterfaceT: Debug {
    fn get_size(&self) -> Result<(u16, u16), CError>;
}
