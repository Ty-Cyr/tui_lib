pub const STD_INPUT_HANDLE: i32 = (u32::MAX - 9) as i32;
pub const STD_OUTPUT_HANDLE: i32 = (u32::MAX - 10) as i32;
pub const ENABLE_MOUSE_INPUT: u32 = 0x2;
pub const ENABLE_EXTENDED_FLAGS: u32 = 0x80;
pub const ENABLE_WINDOW_INPUT: u32 = 0x8;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x200;
pub const KEY_EVENT: u32 = 0x1;

pub mod virtual_keys {
    pub const VK_BACK: u16 = 0x8;
    pub const VK_DELETE: u16 = 0x2E;
    pub const VK_UP: u16 = 0x26;
    pub const VK_DOWN: u16 = 0x28;
    pub const VK_LEFT: u16 = 0x25;
    pub const VK_RIGHT: u16 = 0x27;
    pub const VK_SPACE: u16 = 0x20;
    pub const VK_TAB: u16 = 0x9;
    pub const VK_RETURN: u16 = 0xD;
    pub const VK_SHIFT: u16 = 0x10;
    pub const VK_ESCAPE: u16 = 0x1B;
}
