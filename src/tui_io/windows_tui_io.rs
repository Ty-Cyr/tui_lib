use std::io::{stdout, Stdout, Write};

mod windows;

use crate::{
    tui_io::{input_interface::InputInterfaceT, output_interface::OutputInterfaceT},
    tui_keys::TuiKeys,
};

use windows::constants::{
    virtual_keys, ENABLE_MOUSE_INPUT, ENABLE_WINDOW_INPUT, KEY_EVENT, STD_INPUT_HANDLE,
    STD_OUTPUT_HANDLE,
};

use windows::structs::{CONSOLE_MODE, CONSOLE_SCREEN_BUFFER_INFO, COORD, HANDLE, KEY_EVENT_RECORD};

use windows::functions::{
    get_std_handle, GetConsoleMode, GetConsoleScreenBufferInfo, ReadConsoleInputW, SetConsoleMode,
};
#[derive(Clone, Copy)]
pub struct TerminalState {
    console_mode: CONSOLE_MODE,
}

#[derive(Clone, Copy)]
pub struct InputInterface {
    input_handle: HANDLE,
}

impl InputInterface {
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
}

impl InputInterfaceT for InputInterface {
    fn new() -> Option<InputInterface> {
        let input_interface;
        unsafe {
            let input_handle: HANDLE = get_std_handle(STD_INPUT_HANDLE).ok()?;
            input_interface = InputInterface { input_handle };
        }
        return Some(input_interface);
    }

    fn read_keyboard(&self) -> TuiKeys {
        loop {
            let lpbuffer = [&mut Default::default()];
            let mut event_count: u32 = 0;
            unsafe {
                if !ReadConsoleInputW(
                    self.input_handle.clone(),
                    lpbuffer[0],
                    lpbuffer.len() as u32,
                    &mut event_count,
                )
                .as_bool()
                {
                    return TuiKeys::Error;
                }
            }
            match lpbuffer[0].event_type as u32 {
                KEY_EVENT => {
                    let key_event_data: KEY_EVENT_RECORD;
                    unsafe { key_event_data = lpbuffer[0].event.key_event }
                    if key_event_data.key_down.as_bool() {
                        let event: TuiKeys = parse_key_event_data(key_event_data);
                        match event {
                            TuiKeys::Ignore => continue,
                            _ => return event,
                        }
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }
        }
    }

    fn read_raw(&self) -> Option<char> {
        loop {
            let lpbuffer = [&mut Default::default()];
            let mut event_count: u32 = 0;
            unsafe {
                let result = ReadConsoleInputW(
                    self.input_handle.clone(),
                    lpbuffer[0],
                    lpbuffer.len() as u32,
                    &mut event_count,
                );
                if !result.as_bool() {
                    println!("HERE");
                    return None;
                }
            }
            match lpbuffer[0].event_type as u32 {
                KEY_EVENT => {
                    let key_event_data: KEY_EVENT_RECORD;
                    unsafe { key_event_data = lpbuffer[0].event.key_event }
                    if key_event_data.key_down.as_bool() {
                        let result: char;
                        unsafe {
                            result = key_event_data.u_char.ascii_char.try_into().ok()?;
                        }
                        return Some(result);
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }
        }
    }
}
pub struct OutputInterface {
    output_handle: Stdout,
}

impl OutputInterfaceT for OutputInterface {
    fn get_size(&self) -> Option<(u16, u16)> {
        let mut screen_info_struct: CONSOLE_SCREEN_BUFFER_INFO = Default::default();
        unsafe {
            let handle: HANDLE = get_std_handle(STD_OUTPUT_HANDLE).ok()?;
            if !GetConsoleScreenBufferInfo(handle, &mut screen_info_struct).as_bool() {
                return None;
            }
        }
        let size: COORD = screen_info_struct.size;
        return Some((size.x as u16, size.y as u16));
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
    let new_mode: CONSOLE_MODE = CONSOLE_MODE(ENABLE_MOUSE_INPUT | ENABLE_WINDOW_INPUT);
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
        match data.virtual_key_code {
            virtual_keys::VK_RETURN => return TuiKeys::Enter,
            virtual_keys::VK_LEFT => return TuiKeys::LeftArrow,

            virtual_keys::VK_UP => return TuiKeys::UpArrow,

            virtual_keys::VK_RIGHT => return TuiKeys::RightArrow,

            virtual_keys::VK_DOWN => return TuiKeys::DownArrow,

            virtual_keys::VK_BACK => {
                return TuiKeys::Backspace;
            }

            virtual_keys::VK_DELETE => {
                return TuiKeys::Delete;
            }

            virtual_keys::VK_SPACE => {
                return TuiKeys::Space;
            }

            virtual_keys::VK_TAB => {
                return TuiKeys::Tab;
            }

            virtual_keys::VK_ESCAPE => {
                return TuiKeys::Escape;
            }

            virtual_keys::VK_SHIFT => {
                return TuiKeys::Ignore;
            }

            _ => {
                let char_option: Option<char>;
                unsafe {
                    char_option = char::from_u32(data.u_char.unicode_char as u32);
                }
                if let Some(character) = char_option {
                    match character as u32 {
                        0x20..=0x7D => return TuiKeys::AsciiReadable(character),
                        0 => return TuiKeys::Ignore,
                        1..=26 => return TuiKeys::Control((character as u8 + 0x40) as char),
                        0x1b => return TuiKeys::Escape,
                        _ => return TuiKeys::Other(character),
                    }
                } else {
                    return TuiKeys::Error;
                }
            }
        }
    }
}
