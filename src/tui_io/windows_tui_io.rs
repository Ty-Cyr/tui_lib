use std::{
    error::Error,
    io::{stdout, Stdout, Write},
};

mod windows;

use crate::{
    tui_errors::CError,
    tui_events::TuiEvents,
    tui_io::{
        input_interface::InputInterfaceT, input_parser::ParseInput,
        output_interface::OutputInterfaceT, terminal_interface::TerminalTrait,
    },
};

use windows::constants::{KEY_EVENT, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};

use windows::{
    constants::{ENABLE_EXTENDED_FLAGS, ENABLE_VIRTUAL_TERMINAL_INPUT},
    functions::{
        get_std_handle, GetConsoleMode, GetConsoleScreenBufferInfo, GetNumberOfConsoleInputEvents,
        ReadConsoleInputW, SetConsoleMode,
    },
    structs::{CONSOLE_MODE, CONSOLE_SCREEN_BUFFER_INFO, COORD, HANDLE, KEY_EVENT_RECORD},
};

use self::windows::{functions::get_c_error, structs::INPUT_RECORD};

#[derive(Clone, Copy, Debug)]
pub struct InputInterface {
    input_handle: HANDLE,
}

#[derive(Debug)]
pub struct OutputInterface {
    output_handle: Stdout,
}
pub struct TerminalManager {}
#[derive(Clone, Copy, Debug)]
pub struct TerminalState {
    console_mode: CONSOLE_MODE,
}

impl InputInterface {
    fn get_console_mode(&self) -> Result<CONSOLE_MODE, CError> {
        let mut console_mode: CONSOLE_MODE = Default::default();
        unsafe {
            if !GetConsoleMode(self.input_handle.clone(), &mut console_mode).as_bool() {
                return Err(get_c_error().to_string().into());
            }
        }
        return Ok(console_mode);
    }
    fn set_console_mode(&self, console_mode: CONSOLE_MODE) -> Result<(), CError> {
        unsafe {
            if !SetConsoleMode(self.input_handle.clone(), console_mode).as_bool() {
                return Err(get_c_error().to_string().into());
            }
        }
        _ = self.get_console_mode();
        return Ok(());
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
}

impl ParseInput for InputInterface {}

impl InputInterfaceT for InputInterface {
    fn new() -> Result<InputInterface, Box<dyn Error>> {
        let input_interface;
        unsafe {
            let input_handle: HANDLE = get_std_handle(STD_INPUT_HANDLE)?;
            input_interface = InputInterface { input_handle };
        }
        return Ok(input_interface);
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
                        let input_char = unsafe { key_event_data.u_char.ascii_char as char };
                        let event: TuiEvents = self.parse_input(input_char);
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

impl OutputInterfaceT for OutputInterface {
    fn get_size(&self) -> Result<(u16, u16), CError> {
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

impl TerminalTrait for TerminalManager {
    fn setup_terminal() -> Result<(InputInterface, OutputInterface, TerminalState), Box<dyn Error>>
    {
        let input_interface: InputInterface = InputInterface::new()?;
        let console_mode: CONSOLE_MODE = input_interface.get_console_mode()?;
        let output_interface: OutputInterface = OutputInterface {
            output_handle: stdout(),
        };
        let new_mode: CONSOLE_MODE =
            CONSOLE_MODE(ENABLE_EXTENDED_FLAGS | ENABLE_VIRTUAL_TERMINAL_INPUT);
        _ = input_interface.set_console_mode(new_mode)?;
        return Ok((
            input_interface,
            output_interface,
            TerminalState { console_mode },
        ));
    }

    fn reset_terminal_settings(input_interface: &InputInterface, terminal_state: &TerminalState) {
        _ = input_interface.set_console_mode(terminal_state.console_mode);
    }
}
