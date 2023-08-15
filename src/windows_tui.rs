use crate::{tui_enums::TuiMode, tui_events::TuiEvent, tui_keys::TuiKeys};
use windows::Win32::{
    Foundation::HANDLE,
    System::Console::{
        GetConsoleMode, GetStdHandle, ReadConsoleInputW, SetConsoleMode, CONSOLE_MODE,
        ENABLE_MOUSE_INPUT, ENABLE_WINDOW_INPUT, INPUT_RECORD, KEY_EVENT, KEY_EVENT_RECORD,
        STD_INPUT_HANDLE, WINDOW_BUFFER_SIZE_EVENT,
    },
    UI::Input::KeyboardAndMouse::{
        VIRTUAL_KEY, VK_BACK, VK_DELETE, VK_DOWN, VK_ESCAPE, VK_LEFT, VK_RETURN, VK_RIGHT,
        VK_SHIFT, VK_SPACE, VK_TAB, VK_UP,
    },
};
#[allow(unused)]
#[derive(Clone, Copy)]
pub struct TerminalState {
    tui_mode: TuiMode,
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

    pub fn get_event(&self) -> TuiEvent {
        loop {
            let lpbuffer: &mut [INPUT_RECORD] = &mut [Default::default()];
            let mut event_count: u32 = 0;
            unsafe {
                if !ReadConsoleInputW(self.input_handle.clone(), lpbuffer, &mut event_count)
                    .as_bool()
                {
                    return TuiEvent::Error;
                }
            }
            match lpbuffer[0].EventType as u32 {
                KEY_EVENT => {
                    let key_event_data: KEY_EVENT_RECORD;
                    unsafe { key_event_data = lpbuffer[0].Event.KeyEvent }
                    return parse_key_event_data(key_event_data);
                }
                WINDOW_BUFFER_SIZE_EVENT => return TuiEvent::BufferSizeEvent,
                _ => return TuiEvent::Other,
            }
        }
    }
}

pub fn setup_terminal(tui_mode: &TuiMode) -> Option<(InputInterface, TerminalState)> {
    let input_interface: InputInterface = InputInterface::new()?;
    let console_mode: CONSOLE_MODE = input_interface.get_console_mode()?;
    let new_mode: CONSOLE_MODE = ENABLE_MOUSE_INPUT | ENABLE_WINDOW_INPUT;
    _ = input_interface.set_console_mode(new_mode)?;
    return Some((
        input_interface,
        TerminalState {
            tui_mode: tui_mode.clone(),
            console_mode,
        },
    ));
}

pub fn reset_terminal_settings(input_interface: &InputInterface, terminal_state: &TerminalState) {
    _ = input_interface.set_console_mode(terminal_state.console_mode);
}

fn parse_key_event_data(data: KEY_EVENT_RECORD) -> TuiEvent {
    match VIRTUAL_KEY(data.wVirtualKeyCode) {
        VK_RETURN => {
            return TuiEvent::KeyEvent(data.bKeyDown.as_bool(), TuiKeys::Enter, data.wRepeatCount)
        }
        VK_LEFT => {
            return TuiEvent::KeyEvent(
                data.bKeyDown.as_bool(),
                TuiKeys::LeftArrow,
                data.wRepeatCount,
            )
        }

        VK_UP => {
            return TuiEvent::KeyEvent(data.bKeyDown.as_bool(), TuiKeys::UpArrow, data.wRepeatCount)
        }

        VK_RIGHT => {
            return TuiEvent::KeyEvent(
                data.bKeyDown.as_bool(),
                TuiKeys::RightArrow,
                data.wRepeatCount,
            )
        }

        VK_DOWN => {
            return TuiEvent::KeyEvent(
                data.bKeyDown.as_bool(),
                TuiKeys::DownArrow,
                data.wRepeatCount,
            )
        }

        VK_BACK => {
            return TuiEvent::KeyEvent(
                data.bKeyDown.as_bool(),
                TuiKeys::Backspace,
                data.wRepeatCount,
            );
        }

        VK_DELETE => {
            return TuiEvent::KeyEvent(data.bKeyDown.as_bool(), TuiKeys::Delete, data.wRepeatCount);
        }

        VK_SPACE => {
            return TuiEvent::KeyEvent(data.bKeyDown.as_bool(), TuiKeys::Space, data.wRepeatCount);
        }

        VK_TAB => {
            return TuiEvent::KeyEvent(data.bKeyDown.as_bool(), TuiKeys::Tab, data.wRepeatCount);
        }

        VK_ESCAPE => {
            return TuiEvent::KeyEvent(data.bKeyDown.as_bool(), TuiKeys::Escape, data.wRepeatCount);
        }

        VK_SHIFT => {
            return TuiEvent::KeyEvent(data.bKeyDown.as_bool(), TuiKeys::Shift, data.wRepeatCount);
        }

        _ => {
            let char_option: Option<char>;
            unsafe {
                char_option = char::from_u32(data.uChar.UnicodeChar as u32);
            }
            if let Some(character) = char_option {
                return TuiEvent::KeyEvent(
                    data.bKeyDown.as_bool(),
                    TuiKeys::Other(character),
                    data.wRepeatCount,
                );
            } else {
                return TuiEvent::Error;
            }
        }
    }
}
