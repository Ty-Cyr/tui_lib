pub mod input_interface;
pub mod output_interface;

#[cfg_attr(windows, path = "windows_tui_io.rs")]
#[cfg_attr(unix, path = "unix_tui_io.rs")]
pub mod windows_tui_io;
