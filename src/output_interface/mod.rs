pub trait OutputInterfaceT {
    fn get_size(&self) -> Option<(u16, u16)>;
}
