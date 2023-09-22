pub trait OutputInterfaceT {
    fn get_size(&self) -> Result<(u16, u16), String>;
}
