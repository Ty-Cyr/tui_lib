#[cfg_attr(windows, path = "windows_tui.rs")]
#[cfg_attr(unix, path = "unix_tui.rs")]
mod os_tui;

pub mod string_plus;
use string_plus::StringPlus;

pub mod tui_enums;
use tui_enums::{Color, ThreeBool};

mod font_settings;

pub mod tui_keys;

pub mod tui_terminal;

mod input_interface;
mod output_interface;
