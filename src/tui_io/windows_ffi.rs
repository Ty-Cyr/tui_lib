pub const STD_INPUT_HANDLE: i32 = (u32::MAX - 9) as i32;
pub const STD_OUTPUT_HANDLE: i32 = (u32::MAX - 10) as i32;
pub const ENABLE_MOUSE_INPUT: u32 = 0x2;
pub const ENABLE_WINDOW_INPUT: u32 = 0x8;
pub const KEY_EVENT: u32 = 0x1;

#[repr(C)]
#[derive(Clone, Copy)]
pub union char_union {
    pub unicode_char: u16,
    pub ascii_char: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_EVENT_RECORD {
    pub key_down: BOOL,
    pub repeat_count: u16,
    pub virtual_key_code: u16,
    pub virtual_scan_code: u16,
    pub u_char: char_union,
    pub control_key_state: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COORD {
    pub x: u16,
    pub y: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MOUSE_EVENT_RECORD {
    pub mouse_position: COORD,
    pub button_state: u32,
    pub control_key_state: u32,
    pub event_flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub size: COORD,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MENU_EVENT_RECORD {
    pub command_id: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FOCUS_EVENT_RECORD {
    pub set_focus: BOOL,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union EventRecord {
    pub key_event: KEY_EVENT_RECORD,
    pub mouse_event: MOUSE_EVENT_RECORD,
    pub window_buffer_size_event: WINDOW_BUFFER_SIZE_RECORD,
    pub menu_event: MENU_EVENT_RECORD,
    pub focus_event: FOCUS_EVENT_RECORD,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_RECORD {
    pub event_type: u16,
    pub event: EventRecord,
}

impl Default for INPUT_RECORD {
    fn default() -> Self {
        Self {
            event_type: Default::default(),
            event: EventRecord {
                key_event: KEY_EVENT_RECORD {
                    key_down: BOOL(0),
                    repeat_count: 0,
                    virtual_key_code: 0,
                    virtual_scan_code: 0,
                    u_char: char_union { unicode_char: 0 },
                    control_key_state: 0,
                },
            },
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SMALL_RECT {
    pub left: u16,
    pub top: u16,
    pub right: u16,
    pub bottom: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub size: COORD,
    pub cursor_position: COORD,
    pub attributes: u16,
    pub window: SMALL_RECT,
    pub max_window_size: COORD,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BOOL(pub i32);
impl BOOL {
    pub fn as_bool(&self) -> bool {
        if self.0 == 0 {
            return false;
        }
        return true;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONSOLE_MODE(pub u32);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HANDLE(pub isize);

extern "C" {
    pub fn GetConsoleMode(hConsoleHandle: HANDLE, LPDWORD: *mut CONSOLE_MODE) -> BOOL;
    pub fn SetConsoleMode(hConsoleHandle: HANDLE, DWORD: CONSOLE_MODE) -> BOOL;
    pub fn ReadConsoleInputW(
        hConsoleInput: HANDLE,
        lpBuffer: *mut INPUT_RECORD,
        nLength: u32,
        lpNumberOfEventsRead: *mut u32,
    ) -> BOOL;
    pub fn GetConsoleScreenBufferInfo(
        hConsoleOutput: HANDLE,
        lpConsoleScreenBufferInfo: *mut CONSOLE_SCREEN_BUFFER_INFO,
    ) -> BOOL;
}

mod inner_ffi {
    extern "C" {
        pub fn GetStdHandle(nStdHandle: i32) -> super::HANDLE;
    }
}

pub unsafe fn get_std_handle(std_handle: i32) -> Result<HANDLE, ()> {
    let result = inner_ffi::GetStdHandle(std_handle);
    if result.0 == -1 {
        return Err(());
    }
    return Ok(result);
}

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
