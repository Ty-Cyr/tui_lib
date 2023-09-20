use std::ffi::c_void;

use super::structs::Termios;
extern "C" {
    pub fn cfmakeraw(termios: *mut Termios);
    pub fn fcntl(fd: i32, cmd: i32, ...) -> i32;
    pub fn ioctl(fd: i32, request: u64, ...) -> i32;
    pub fn tcgetattr(fd: i32, termios: *mut Termios) -> i32;
    pub fn tcsetattr(fd: i32, optional_actions: i32, termios: *const Termios) -> i32;
    pub fn read(fd: i32, buf: *mut c_void, count: usize) -> isize;
}
