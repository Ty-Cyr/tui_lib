use crate::tui_errors::CError;
use crate::tui_events::TuiEvents;
use std::ffi::{c_char, c_void};

mod unix;
use unix::constants::{F_GETFL, F_SETFL};
use unix::functions::{cfmakeraw, read as c_read};

use std::io::{stdin, stdout, Stdout, Write};
use std::os::unix::prelude::AsRawFd;

use self::unix::constants::{ONLCR, OPOST, O_NONBLOCK, STDOUT_FILENO, TCSADRAIN, TIOCGWINSZ};
use self::unix::functions::{fcntl, get_errno_error, ioctl, tcgetattr, tcsetattr};
use self::unix::structs::{Termios, Winsize};

use super::input_interface::InputInterfaceT;
use super::mouse_input::MouseInput;
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
    pub fn get_input_mode(&self) -> Result<Termios, CError> {
        let mut termios_struct: Termios = Termios::default();
        unsafe {
            if tcgetattr(self.input_fd.clone(), &mut termios_struct) == -1 {
                return Err(get_errno_error());
            }
        }
        return Ok(termios_struct);
    }
    pub fn set_input_mode(&self, mut termios_struct: Termios) -> Result<(), CError> {
        unsafe {
            if -1 == tcsetattr(self.input_fd, TCSADRAIN, &mut termios_struct) {
                return Err(get_errno_error());
            }
        }
        return Ok(());
    }

    pub fn get_raw_termios_struct(&self) -> Termios {
        let mut termios_struct: Termios = Termios::default();
        unsafe {
            cfmakeraw(&mut termios_struct);
            termios_struct.c_oflag |= ONLCR | OPOST;
        }
        return termios_struct;
    }

    fn read_char(&self) -> char {
        let mut buffer: [c_char; 1] = [0];
        unsafe {
            c_read(self.input_fd.clone(), buffer.as_mut_ptr() as *mut c_void, 1);
        };

        return (buffer[0] as u8) as char;
    }

    fn handle_escape_input_s1(&self) -> TuiEvents {
        let input_char_option: Option<char> = self.read_raw_immediate();
        let result = match input_char_option {
            None => TuiEvents::Escape,
            Some('[') => self.handle_escape_input_s2(),
            Some(_) => TuiEvents::Error,
        };
        match result {
            TuiEvents::Error | TuiEvents::Ignore => loop {
                if let None = self.read_raw_immediate() {
                    return result;
                };
            },
            _ => return result,
        }
    }

    fn handle_escape_input_s2(&self) -> TuiEvents {
        let input_char: char = self.read_char();
        return match input_char {
            'A' => TuiEvents::UpArrow,
            'B' => TuiEvents::DownArrow,
            'C' => TuiEvents::RightArrow,
            'D' => TuiEvents::LeftArrow,
            '3' => {
                if let Some('~') = self.read_raw_immediate() {
                    return TuiEvents::Delete;
                }
                TuiEvents::Error
            }
            '<' => self.handle_mouse_events(),
            _ => TuiEvents::Error,
        };
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

    fn read_raw_immediate(&self) -> Option<char> {
        let input_interface = NonBlockInputInterface::new(self.input_fd);
        return input_interface.read_raw_immediate();
    }
}

impl MouseInput for InputInterface {}

struct NonBlockInputInterface {
    input_fd: i32,
    fd_flags: i32,
}

impl NonBlockInputInterface {
    fn new(input_fd: i32) -> NonBlockInputInterface {
        let fd_flags;
        unsafe {
            fd_flags = fcntl(input_fd.clone(), F_GETFL);
            fcntl(input_fd.clone(), F_SETFL, fd_flags | O_NONBLOCK)
        };
        return NonBlockInputInterface {
            input_fd: input_fd,
            fd_flags: fd_flags,
        };
    }
    fn read_raw_immediate(&self) -> Option<char> {
        let mut buffer: [c_char; 1] = [0];
        unsafe {
            if 1 != c_read(self.input_fd.clone(), buffer.as_mut_ptr() as *mut c_void, 1) {
                return None;
            }
        };

        return Some((buffer[0] as u8) as char);
    }
}

impl Drop for NonBlockInputInterface {
    fn drop(&mut self) {
        unsafe { fcntl(self.input_fd.clone(), F_SETFL, self.fd_flags) };
    }
}

pub struct OutputInterface {
    output_handle: Stdout,
}

impl OutputInterfaceT for OutputInterface {
    fn get_size(&self) -> Result<(u16, u16), CError> {
        let mut window_size: Winsize = Winsize::default();
        unsafe {
            if 0 != ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut window_size) {
                return Err(get_errno_error());
            }
        }
        return Ok((window_size.ws_col as u16, window_size.ws_row as u16));
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
        termios_struct: input_interface.get_input_mode().ok()?,
    };
    input_interface
        .set_input_mode(input_interface.get_raw_termios_struct())
        .ok()?;

    return Some((input_interface, output_interface, terminal_state));
}

pub fn reset_terminal_settings(input_interface: &InputInterface, terminal_state: &TerminalState) {
    _ = input_interface.set_input_mode(terminal_state.termios_struct);
}
