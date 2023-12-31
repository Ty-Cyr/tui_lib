pub mod input_interface;
pub mod output_interface;
pub mod terminal_interface;

#[cfg_attr(windows, path = "windows_tui_io.rs")]
#[cfg_attr(unix, path = "unix_tui_io.rs")]
pub mod tui_io;

mod input_parser;
