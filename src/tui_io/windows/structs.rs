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
#[derive(Clone, Copy, Debug, Default)]
pub struct CONSOLE_MODE(pub u32);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct HANDLE(pub isize);
