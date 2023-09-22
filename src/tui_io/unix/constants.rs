pub const F_GETFL: i32 = 3;
pub const F_SETFL: i32 = 4;

#[cfg(not(target_os = "macos"))]
pub const ONLCR: u32 = 0x4;
#[cfg(not(target_os = "macos"))]
pub const OPOST: u32 = 0x1;
#[cfg(not(target_os = "macos"))]
pub const O_NONBLOCK: i32 = 2048;

#[cfg(target_os = "macos")]
pub const ONLCR: u64 = 0x2;
#[cfg(target_os = "macos")]
pub const OPOST: u64 = 0x1;
#[cfg(target_os = "macos")]
pub const O_NONBLOCK: i32 = 4;

pub const STDOUT_FILENO: i32 = 1;
pub const TCSADRAIN: i32 = 1;
#[cfg(not(target_os = "macos"))]
pub const TIOCGWINSZ: u64 = 0x5413;

#[cfg(target_os = "macos")]
pub const TIOCGWINSZ: u64 = 0x40087468;
