use std::{ffi::CStr, ptr::null};

use crate::tui_errors::CError;

use super::structs::{BOOL, CONSOLE_MODE, CONSOLE_SCREEN_BUFFER_INFO, HANDLE, INPUT_RECORD};
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
    pub fn GetNumberOfConsoleInputEvents(hConsoleInput: HANDLE, numberOfEvents: *mut u32) -> BOOL;
    fn GetLastError() -> u32;
    fn FormatMessageA(
        dwFlages: u32,
        lpSource: *const u8,
        dwMessageId: u32,
        dwlangaugeId: u32,
        lpBuffer: *mut i8,
        nSize: u32,
        ...
    ) -> u32;
}

mod inner_ffi {
    extern "C" {
        pub fn GetStdHandle(nStdHandle: i32) -> super::HANDLE;
    }
}

pub unsafe fn get_c_error() -> CError {
    const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0;
    const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
    const FORMAT_MESSAGE_MAX_WIDTH_MASK: u32 = 0x000000FF;
    let error_code = GetLastError();
    let mut buffer = [0; 512];
    let format_result = FormatMessageA(
        FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_MAX_WIDTH_MASK,
        null(),
        error_code,
        0,
        buffer.as_mut_ptr(),
        buffer.len() as u32,
    );
    if format_result == 0 {
        return "Failed to Get Error Message".into();
    }
    return CStr::from_ptr(buffer.as_ptr())
        .to_str()
        .unwrap_or("Failed To Get Error Message: Invalid CString")
        .trim()
        .into();
}

pub unsafe fn get_std_handle(std_handle: i32) -> Result<HANDLE, CError> {
    let result = inner_ffi::GetStdHandle(std_handle);
    if result.0 == -1 {
        return Err(get_c_error());
    }
    return Ok(result);
}
