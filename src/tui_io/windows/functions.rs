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
}

mod inner_ffi {
    extern "C" {
        pub fn GetStdHandle(nStdHandle: i32) -> super::HANDLE;
    }
}

pub unsafe fn get_c_error() -> CError {
    return GetLastError().to_string().into();
}

pub unsafe fn get_std_handle(std_handle: i32) -> Result<HANDLE, CError> {
    let result = inner_ffi::GetStdHandle(std_handle);
    if result.0 == -1 {
        return Err(get_c_error());
    }
    return Ok(result);
}
