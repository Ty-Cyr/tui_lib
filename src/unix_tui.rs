use crate::tui_enums::TuiMode;
use crate::tui_events::TuiEvent;
use crate::tui_keys::TuiKeys;
use libc::{
    c_char, c_void, cfmakeraw, fcntl, read as cread, tcgetattr, tcsetattr, termios, F_GETFL,
    F_SETFL, ONLCR, OPOST, O_NONBLOCK, TCSADRAIN,
};

use std::io::stdin;
use std::os::unix::prelude::AsRawFd;

#[derive(Clone, Copy)]
pub struct TerminalState {
    termios_struct: termios,
}

#[derive(Clone, Copy)]
pub struct InputInterface {
    input_fd: i32,
}

impl InputInterface {
    pub fn new() -> InputInterface {
        let input_fd: i32 = stdin().as_raw_fd();
        return InputInterface { input_fd: input_fd };
    }
    pub fn get_input_mode(&self) -> Option<termios> {
        let mut termios_struct: termios = new_termios();
        unsafe {
            if tcgetattr(self.input_fd.clone(), &mut termios_struct) == -1 {
                return None;
            }
        }
        return Some(termios_struct);
    }
    pub fn set_input_mode(&self, mut termios_struct: termios) {
        unsafe {
            tcsetattr(self.input_fd, TCSADRAIN, &mut termios_struct);
        }
    }

    pub fn get_raw_termios_struct(&self) -> termios {
        let mut termios_struct: termios = new_termios();
        unsafe {
            cfmakeraw(&mut termios_struct);
            termios_struct.c_oflag |= ONLCR | OPOST;
        }
        return termios_struct;
    }

    fn read_char(&self) -> char {
        let mut buffer: [c_char; 1] = [0];
        unsafe { cread(self.input_fd.clone(), buffer.as_mut_ptr() as *mut c_void, 1) };

        return (buffer[0] as u8) as char;
    }

    fn try_read_char(&self) -> Option<char> {
        let mut buffer: [c_char; 1] = [0];
        unsafe {
            fcntl(
                self.input_fd.clone(),
                F_SETFL,
                fcntl(self.input_fd.clone(), F_GETFL) | O_NONBLOCK,
            );
            if 1 != cread(self.input_fd.clone(), buffer.as_mut_ptr() as *mut c_void, 1) {
                return None;
            }
            fcntl(
                self.input_fd.clone(),
                F_SETFL,
                fcntl(self.input_fd.clone(), F_GETFL) & !O_NONBLOCK,
            );
        };

        return Some((buffer[0] as u8) as char);
    }

    fn handle_escape_input_s1(&self) -> TuiEvent {
        let input_char_option: Option<char> = self.try_read_char();
        match input_char_option {
            None => return TuiEvent::KeyEvent(TuiKeys::Escape),
            Some(input_char) => match input_char {
                '[' => return self.handle_escape_input_s2(),
                _ => return TuiEvent::Error,
            },
        };
    }

    fn handle_escape_input_s2(&self) -> TuiEvent {
        let input_char: char = self.read_char();
        match input_char {
            'A' => return TuiEvent::KeyEvent(TuiKeys::UpArrow),
            'B' => return TuiEvent::KeyEvent(TuiKeys::DownArrow),
            'C' => return TuiEvent::KeyEvent(TuiKeys::RightArrow),
            'D' => return TuiEvent::KeyEvent(TuiKeys::LeftArrow),
            '3' => {
                if let Some(input_char) = self.try_read_char() {
                    if input_char == '~' {
                        return TuiEvent::KeyEvent(TuiKeys::Delete);
                    }
                }
                return TuiEvent::Error;
            }
            _ => return TuiEvent::Other,
        }
    }

    pub fn get_event(&self) -> TuiEvent {
        let input_char: char = self.read_char();
        match input_char {
            '\x1b' => {
                return self.handle_escape_input_s1();
            }
            '\x7F' => {
                return TuiEvent::KeyEvent(TuiKeys::Backspace);
            }
            '\n' | '\r' => {
                return TuiEvent::KeyEvent(TuiKeys::Enter);
            }
            ' ' => {
                return TuiEvent::KeyEvent(TuiKeys::Space);
            }
            '\t' => {
                return TuiEvent::KeyEvent(TuiKeys::Tab);
            }
            _ => {
                return TuiEvent::KeyEvent(TuiKeys::Other(input_char));
            }
        }
    }
}

#[allow(unused)]
pub fn setup_terminal(tui_mode: &TuiMode) -> Option<(InputInterface, TerminalState)> {
    let input_interface: InputInterface = InputInterface::new();
    let terminal_state: TerminalState = TerminalState {
        termios_struct: input_interface.get_input_mode()?,
    };
    input_interface.set_input_mode(input_interface.get_raw_termios_struct());

    return Some((input_interface, terminal_state));
}

#[allow(unused)]
pub fn reset_terminal_settings(input_interface: &InputInterface, terminal_state: &TerminalState) {
    input_interface.set_input_mode(terminal_state.termios_struct);
}

#[cfg(target_os = "linux")]
pub fn new_termios() -> termios {
    let termios_struct: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    return termios_struct;
}
#[cfg(target_os = "macos")]
pub fn new_termios() -> termios {
    let termios_struct: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0; 20],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    return termios_struct;
}
