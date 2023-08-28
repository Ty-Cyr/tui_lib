use std::io::{stdout, Stdout, Write};

use crate::tui_keys::TuiKeys;
use windows::Win32::{
    Foundation::HANDLE,
    System::Console::{
        GetConsoleMode, GetConsoleScreenBufferInfo, GetStdHandle, ReadConsoleInputW,
        SetConsoleMode, CONSOLE_MODE, CONSOLE_SCREEN_BUFFER_INFO, COORD, ENABLE_MOUSE_INPUT,
        ENABLE_WINDOW_INPUT, INPUT_RECORD, KEY_EVENT, KEY_EVENT_RECORD, STD_INPUT_HANDLE,
        STD_OUTPUT_HANDLE,
    },
    UI::Input::KeyboardAndMouse::{
        VIRTUAL_KEY, VK_BACK, VK_DELETE, VK_DOWN, VK_ESCAPE, VK_LEFT, VK_RETURN, VK_RIGHT,
        VK_SHIFT, VK_SPACE, VK_TAB, VK_UP,
    },
};
#[allow(unused)]
#[derive(Clone, Copy)]
pub struct TerminalState {
    console_mode: CONSOLE_MODE,
}
pub struct InputInterface {
    input_handle: HANDLE,
}

impl InputInterface {
    fn new() -> Option<InputInterface> {
        let input_interface;
        unsafe {
            let input_handle: HANDLE = GetStdHandle(STD_INPUT_HANDLE).ok()?;
            input_interface = InputInterface { input_handle };
        }
        return Some(input_interface);
    }
    fn get_console_mode(&self) -> Option<CONSOLE_MODE> {
        let mut console_mode: CONSOLE_MODE = Default::default();
        unsafe {
            if !GetConsoleMode(self.input_handle.clone(), &mut console_mode).as_bool() {
                return None;
            }
        }
        return Some(console_mode);
    }
    fn set_console_mode(&self, console_mode: CONSOLE_MODE) -> Option<()> {
        unsafe {
            if !SetConsoleMode(self.input_handle.clone(), console_mode).as_bool() {
                return None;
            }
        }
        _ = self.get_console_mode();
        return Some(());
    }

    pub fn get_keyboard_event(&self) -> TuiKeys {
        loop {
            let lpbuffer: &mut [INPUT_RECORD] = &mut [Default::default()];
            let mut event_count: u32 = 0;
            unsafe {
                if !ReadConsoleInputW(self.input_handle.clone(), lpbuffer, &mut event_count)
                    .as_bool()
                {
                    return TuiKeys::Error;
                }
            }
            match lpbuffer[0].EventType as u32 {
                KEY_EVENT => {
                    let key_event_data: KEY_EVENT_RECORD;
                    unsafe { key_event_data = lpbuffer[0].Event.KeyEvent }
                    if key_event_data.bKeyDown.as_bool() {
                        let event: TuiKeys = parse_key_event_data(key_event_data);
                        match event {
                            TuiKeys::Ignore => continue,
                            _ => return event,
                        }
                    } else {
                        continue;
                    }
                }
                _ => return TuiKeys::Error,
            }
        }
    }
}

pub struct OutputInterface {
    output_handle: Stdout,
}

impl OutputInterface {
    pub fn get_size(&self) -> Option<(u16, u16)> {
        let mut screen_info_struct: CONSOLE_SCREEN_BUFFER_INFO = Default::default();
        unsafe {
            let handle: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE).ok()?;
            if !GetConsoleScreenBufferInfo(handle, &mut screen_info_struct).as_bool() {
                return None;
            }
        }
        let size: COORD = screen_info_struct.dwSize;
        if size.X >= 0 && size.Y >= 0 {
            return Some((size.X as u16, size.Y as u16));
        }
        return None;
    }
}

impl Write for OutputInterface {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        return self.output_handle.write(buf);
    }

    fn flush(&mut self) -> std::io::Result<()> {
        return self.output_handle.flush();
    }
}

pub fn setup_terminal() -> Option<(InputInterface, OutputInterface, TerminalState)> {
    let input_interface: InputInterface = InputInterface::new()?;
    let console_mode: CONSOLE_MODE = input_interface.get_console_mode()?;
    let output_interface: OutputInterface = OutputInterface {
        output_handle: stdout(),
    };
    let new_mode: CONSOLE_MODE = ENABLE_MOUSE_INPUT | ENABLE_WINDOW_INPUT;
    _ = input_interface.set_console_mode(new_mode)?;
    return Some((
        input_interface,
        output_interface,
        TerminalState { console_mode },
    ));
}

pub fn reset_terminal_settings(input_interface: &InputInterface, terminal_state: &TerminalState) {
    _ = input_interface.set_console_mode(terminal_state.console_mode);
}

fn parse_key_event_data(data: KEY_EVENT_RECORD) -> TuiKeys {
    loop {
        match VIRTUAL_KEY(data.wVirtualKeyCode) {
            VK_RETURN => return TuiKeys::Enter,
            VK_LEFT => return TuiKeys::LeftArrow,

            VK_UP => return TuiKeys::UpArrow,

            VK_RIGHT => return TuiKeys::RightArrow,

            VK_DOWN => return TuiKeys::DownArrow,

            VK_BACK => {
                return TuiKeys::Backspace;
            }

            VK_DELETE => {
                return TuiKeys::Delete;
            }

            VK_SPACE => {
                return TuiKeys::Space;
            }

            VK_TAB => {
                return TuiKeys::Tab;
            }

            VK_ESCAPE => {
                return TuiKeys::Escape;
            }

            VK_SHIFT => {
                return TuiKeys::Ignore;
            }

            _ => {
                let char_option: Option<char>;
                unsafe {
                    char_option = char::from_u32(data.uChar.UnicodeChar as u32);
                }
                if let Some(character) = char_option {
                    if (character as u32) > 0x20 && (character as u32) < 0x7F {
                        return TuiKeys::AsciiReadable(character);
                    }
                    return TuiKeys::Other(character);
                } else {
                    return TuiKeys::Error;
                }
            }
        }
    }
}
