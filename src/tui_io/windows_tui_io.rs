use std::io::{stdout, Stdout, Write};

mod windows;

use crate::{
    tui_events::TuiEvents,
    tui_io::{
        input_interface::InputInterfaceT, mouse_input::MouseInput,
        output_interface::OutputInterfaceT,
    },
};

use windows::constants::{
    virtual_keys, ENABLE_MOUSE_INPUT, ENABLE_WINDOW_INPUT, KEY_EVENT, STD_INPUT_HANDLE,
    STD_OUTPUT_HANDLE,
};

use windows::{
    constants::{ENABLE_EXTENDED_FLAGS, ENABLE_VIRTUAL_TERMINAL_INPUT},
    functions::{
        get_std_handle, GetConsoleMode, GetConsoleScreenBufferInfo, GetNumberOfConsoleInputEvents,
        ReadConsoleInputW, SetConsoleMode,
    },
    structs::{CONSOLE_MODE, CONSOLE_SCREEN_BUFFER_INFO, COORD, HANDLE, KEY_EVENT_RECORD},
};

use self::windows::structs::INPUT_RECORD;

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

    fn is_event_ready(&self) -> Option<bool> {
        let mut count = 0;
        unsafe {
            if !GetNumberOfConsoleInputEvents(self.input_handle, &mut count).as_bool() {
                return None;
            }
        }
        return Some(count > 0);
    }

    fn get_event(&self) -> Result<INPUT_RECORD, String> {
        let event = &mut INPUT_RECORD::default();
        let mut event_count = 0;
        unsafe {
            if !ReadConsoleInputW(self.input_handle.clone(), event, 1, &mut event_count).as_bool() {
                return Err("Failed To Get Event".into());
            }
        }
        return Ok(*event);
    }

    fn handle_escape_sequence(&self) -> TuiEvents {
        match self.read_raw_immediate() {
            Some('[') => {}
            Some(_) => loop {
                if let None = self.read_raw_immediate() {
                    return TuiEvents::Error;
                };
            },
            None => return TuiEvents::Escape,
        }

        let Some('<') = self.read_raw_immediate() else {
            loop {
                if let None = self.read_raw_immediate() {
                    return TuiEvents::Error;
                };
            }
        };

        let result = self.handle_mouse_events();
        match result {
            TuiEvents::Error | TuiEvents::Ignore => loop {
                if let None = self.read_raw_immediate() {
                    return result;
                }
            },
            _ => return result,
        }
    }

    fn parse_key_event_data(&self, data: KEY_EVENT_RECORD) -> TuiEvents {
        loop {
            match data.virtual_key_code {
                virtual_keys::VK_RETURN => return TuiEvents::Enter,
                virtual_keys::VK_LEFT => return TuiEvents::LeftArrow,

                virtual_keys::VK_UP => return TuiEvents::UpArrow,

                virtual_keys::VK_RIGHT => return TuiEvents::RightArrow,

                virtual_keys::VK_DOWN => return TuiEvents::DownArrow,

                virtual_keys::VK_BACK => {
                    return TuiEvents::Backspace;
                }

                virtual_keys::VK_DELETE => {
                    return TuiEvents::Delete;
                }

                virtual_keys::VK_SPACE => {
                    return TuiEvents::Space;
                }

                virtual_keys::VK_TAB => {
                    return TuiEvents::Tab;
                }

                virtual_keys::VK_ESCAPE => return self.handle_escape_sequence(),

                virtual_keys::VK_SHIFT => {
                    return TuiEvents::Ignore;
                }

                _ => {
                    let char_option: Option<char>;
                    unsafe {
                        char_option = char::from_u32(data.u_char.unicode_char as u32);
                    }
                    if let Some(character) = char_option {
                        match character as u32 {
                            0x20..=0x7D => return TuiEvents::AsciiReadable(character),
                            0 => return TuiEvents::Ignore,
                            1..=26 => return TuiEvents::Control((character as u8 + 0x40) as char),
                            0x1b => return self.handle_escape_sequence(),
                            _ => return TuiEvents::Other(character),
                        }
                    } else {
                        return TuiEvents::Error;
                    }
                }
            }
        }
    }
}

impl MouseInput for InputInterface {}

impl InputInterfaceT for InputInterface {
    fn new() -> Option<InputInterface> {
        let input_interface;
        unsafe {
            let input_handle: HANDLE = get_std_handle(STD_INPUT_HANDLE).ok()?;
            input_interface = InputInterface { input_handle };
        }
        return Some(input_interface);
    }

    fn read_parsed(&self) -> TuiEvents {
        loop {
            let Ok(event) = self.get_event() else {
                return TuiEvents::Error;
            };
            match event.event_type as u32 {
                KEY_EVENT => {
                    let key_event_data: KEY_EVENT_RECORD;
                    unsafe { key_event_data = event.event.key_event }
                    if key_event_data.key_down.as_bool() {
                        let event: TuiEvents = self.parse_key_event_data(key_event_data);
                        match event {
                            TuiEvents::Ignore => continue,
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
            let event = self.get_event().ok()?;
            match event.event_type as u32 {
                KEY_EVENT => {
                    let key_event_data: KEY_EVENT_RECORD;
                    unsafe { key_event_data = event.event.key_event }
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

    fn read_raw_immediate(&self) -> Option<char> {
        loop {
            if !self.is_event_ready()? {
                return None;
            }
            let event = self.get_event().ok()?;
            match event.event_type as u32 {
                KEY_EVENT => {
                    let key_event_data: KEY_EVENT_RECORD;
                    unsafe { key_event_data = event.event.key_event }
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
    fn get_size(&self) -> Result<(u16, u16), String> {
        let mut screen_info_struct: CONSOLE_SCREEN_BUFFER_INFO = Default::default();
        unsafe {
            let handle: HANDLE = get_std_handle(STD_OUTPUT_HANDLE)
                .ok()
                .ok_or("Failed to Get HANDLE")?;
            if !GetConsoleScreenBufferInfo(handle, &mut screen_info_struct).as_bool() {
                return Err("Unknown Error".into());
            }
        }
        let size: COORD = screen_info_struct.size;
        return Ok((size.x as u16, size.y as u16));
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
    let new_mode: CONSOLE_MODE = CONSOLE_MODE(
        ENABLE_EXTENDED_FLAGS
            | ENABLE_MOUSE_INPUT
            | ENABLE_WINDOW_INPUT
            | ENABLE_VIRTUAL_TERMINAL_INPUT,
    );
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
