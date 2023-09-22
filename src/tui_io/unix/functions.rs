use std::ffi::{c_void, CStr};

use super::structs::Termios;
extern "C" {
    pub fn cfmakeraw(termios: *mut Termios);
    pub fn fcntl(fd: i32, cmd: i32, ...) -> i32;
    pub fn ioctl(fd: i32, request: u64, ...) -> i32;
    pub fn tcgetattr(fd: i32, termios: *mut Termios) -> i32;
    pub fn tcsetattr(fd: i32, optional_actions: i32, termios: *const Termios) -> i32;
    pub fn read(fd: i32, buf: *mut c_void, count: usize) -> isize;
    fn strerror(errno: u32) -> *const i8;
    fn __error() -> *mut u32;
    fn __errno_location() -> *mut u32;
}

#[cfg(not(target_os = "macos"))]
fn errno() -> u32 {
    unsafe {
        return *__errno_location();
    }
}

#[cfg(target_os = "macos")]
fn errno() -> u32 {
    unsafe {
        return *__error();
    }
}

pub fn get_errno_error() -> String {
    let string_pointer: *const i8;
    let result: String;
    unsafe {
        string_pointer = strerror(errno());
        result = CStr::from_ptr(string_pointer)
            .to_str()
            .unwrap_or("Unknown Error")
            .into();
    }
    return result;
}
