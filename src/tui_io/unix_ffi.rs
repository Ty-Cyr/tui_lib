use std::ffi::{c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const ONLCR: c_uint = 0x4;
pub const OPOST: c_uint = 0x1;
pub const O_NONBLOCK: c_int = 2048;
pub const STDOUT_FILENO: c_int = 1;
pub const TCSADRAIN: c_int = 1;
pub const TIOCGWINSZ: c_ulong = 0x5413;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Termios {
    pub c_iflag: c_uint,
    pub c_oflag: c_uint,
    pub c_cflag: c_uint,
    pub c_lflag: c_uint,
    pub c_line: c_uchar,
    pub c_cc: [c_uchar; 32],
    pub c_ispeed: c_uint,
    pub c_ospeed: c_uint,
}

#[repr(C)]
pub struct Winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}
extern "C" {

    pub fn cfmakeraw(termios: *mut Termios);
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    pub fn tcgetattr(fd: c_int, termios: *mut Termios) -> c_int;
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const Termios) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
}
