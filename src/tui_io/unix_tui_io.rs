use crate::tui_errors::CError;
use crate::tui_events::TuiEvents;
use std::error::Error;
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
use super::input_parser::ParseInput;
use super::output_interface::OutputInterfaceT;
use super::terminal_interface::TerminalTrait;

#[derive(Clone, Copy, Debug)]
pub struct InputInterface {
    input_fd: i32,
}
#[derive(Debug)]
pub struct OutputInterface {
    output_handle: Stdout,
}
pub struct TerminalManager {}

#[derive(Clone, Copy, Debug)]
pub struct TerminalState {
    termios_struct: Termios,
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
}

impl InputInterfaceT for InputInterface {
    fn new() -> Result<InputInterface, Box<(dyn Error)>> {
        let input_fd: i32 = stdin().as_raw_fd();
        return Ok(InputInterface { input_fd: input_fd });
    }

    fn read_parsed(&self) -> TuiEvents {
        loop {
            let Some(input_char) = self.read_raw() else {
                return TuiEvents::Error;
            };
            let event = self.parse_input(input_char);
            let TuiEvents::Ignore = event else {
                return event;
            };
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

impl ParseInput for InputInterface {}

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

impl TerminalTrait for TerminalManager {
    fn setup_terminal() -> Result<(InputInterface, OutputInterface, TerminalState), Box<dyn Error>>
    {
        let input_interface: InputInterface = InputInterface::new()?;
        let output_interface: OutputInterface = OutputInterface {
            output_handle: stdout(),
        };
        let terminal_state: TerminalState = TerminalState {
            termios_struct: input_interface.get_input_mode()?,
        };
        input_interface.set_input_mode(input_interface.get_raw_termios_struct())?;

        return Ok((input_interface, output_interface, terminal_state));
    }

    fn reset_terminal_settings(input_interface: &InputInterface, terminal_state: &TerminalState) {
        _ = input_interface.set_input_mode(terminal_state.termios_struct);
    }
}
