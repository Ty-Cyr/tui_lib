use crate::tui_events::TuiEvents;
use std::ffi::{c_char, c_void};

mod unix;
use unix::constants::{F_GETFL, F_SETFL};
use unix::functions::{cfmakeraw, read as c_read};

use std::io::{stdin, stdout, Stdout, Write};
use std::os::unix::prelude::AsRawFd;

use self::unix::constants::{ONLCR, OPOST, O_NONBLOCK, STDOUT_FILENO, TCSADRAIN, TIOCGWINSZ};
use self::unix::functions::{fcntl, ioctl, tcgetattr, tcsetattr};
use self::unix::structs::{Termios, Winsize};

use super::input_interface::InputInterfaceT;
use super::output_interface::OutputInterfaceT;

#[derive(Clone, Copy)]
pub struct TerminalState {
    termios_struct: Termios,
}

#[derive(Clone, Copy)]
pub struct InputInterface {
    input_fd: i32,
}

impl InputInterface {
    pub fn get_input_mode(&self) -> Option<Termios> {
        let mut termios_struct: Termios = new_termios();
        unsafe {
            if tcgetattr(self.input_fd.clone(), &mut termios_struct) == -1 {
                return None;
            }
        }
        return Some(termios_struct);
    }
    pub fn set_input_mode(&self, mut termios_struct: Termios) {
        unsafe {
            tcsetattr(self.input_fd, TCSADRAIN, &mut termios_struct);
        }
    }

    pub fn get_raw_termios_struct(&self) -> Termios {
        let mut termios_struct: Termios = new_termios();
        unsafe {
            cfmakeraw(&mut termios_struct);
            termios_struct.c_oflag |= ONLCR | OPOST;
        }
        return termios_struct;
    }

    fn read_char(&self) -> char {
        let mut buffer: [c_char; 1] = [0];
        unsafe { c_read(self.input_fd.clone(), buffer.as_mut_ptr() as *mut c_void, 1) };

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
            if 1 != c_read(self.input_fd.clone(), buffer.as_mut_ptr() as *mut c_void, 1) {
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

    fn handle_escape_input_s1(&self) -> TuiEvents {
        let input_char_option: Option<char> = self.try_read_char();
        match input_char_option {
            None => return TuiEvents::Escape,
            Some('[') => return self.handle_escape_input_s2(),
            Some(_) => return TuiEvents::Error,
        };
    }

    fn handle_escape_input_s2(&self) -> TuiEvents {
        let input_char: char = self.read_char();
        match input_char {
            'A' => return TuiEvents::UpArrow,
            'B' => return TuiEvents::DownArrow,
            'C' => return TuiEvents::RightArrow,
            'D' => return TuiEvents::LeftArrow,
            '3' => {
                if let Some('~') = self.try_read_char() {
                    return TuiEvents::Delete;
                }
                return TuiEvents::Error;
            }
            _ => {
                return TuiEvents::Error;
            }
        }
    }
}

impl InputInterfaceT for InputInterface {
    fn new() -> Option<InputInterface> {
        let input_fd: i32 = stdin().as_raw_fd();
        return Some(InputInterface { input_fd: input_fd });
    }

    fn read_parsed(&self) -> TuiEvents {
        let input_char: char = self.read_char();
        match input_char {
            '\x1b' => {
                return self.handle_escape_input_s1();
            }
            '\x7F' => {
                return TuiEvents::Backspace;
            }
            '\n' | '\r' => {
                return TuiEvents::Enter;
            }
            ' ' => {
                return TuiEvents::Space;
            }
            '\t' => {
                return TuiEvents::Tab;
            }
            _ => match input_char as u32 {
                0x20..=0x7D => return TuiEvents::AsciiReadable(input_char),
                0 => TuiEvents::Ignore,
                1..=26 => return TuiEvents::Control((input_char as u8 + 0x40) as char),
                _ => return TuiEvents::Other(input_char),
            },
        }
    }

    fn read_raw(&self) -> Option<char> {
        return Some(self.read_char());
    }
}

pub struct OutputInterface {
    output_handle: Stdout,
}

impl OutputInterfaceT for OutputInterface {
    fn get_size(&self) -> Option<(u16, u16)> {
        let mut window_size: Winsize = Winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        unsafe {
            if 0 != ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut window_size) {
                return None;
            }
        }
        return Some((window_size.ws_col as u16, window_size.ws_row as u16));
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
    let output_interface: OutputInterface = OutputInterface {
        output_handle: stdout(),
    };
    let terminal_state: TerminalState = TerminalState {
        termios_struct: input_interface.get_input_mode()?,
    };
    input_interface.set_input_mode(input_interface.get_raw_termios_struct());

    return Some((input_interface, output_interface, terminal_state));
}

pub fn reset_terminal_settings(input_interface: &InputInterface, terminal_state: &TerminalState) {
    input_interface.set_input_mode(terminal_state.termios_struct);
}

#[cfg(not(target_os = "macos"))]
pub fn new_termios() -> Termios {
    let termios_struct: Termios = Termios {
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
pub fn new_termios() -> Termios {
    let termios_struct: Termios = Termios {
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
