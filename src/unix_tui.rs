use crate::tui_keys::TuiKeys;
use libc::{
    c_char, c_void, cfmakeraw, fcntl, ioctl, read as c_read, tcgetattr, tcsetattr, termios,
    winsize, F_GETFL, F_SETFL, ONLCR, OPOST, O_NONBLOCK, STDOUT_FILENO, TCSADRAIN, TIOCGWINSZ,
};

use std::fs::OpenOptions;
use std::io::{stdin, stdout, Stdout, Write};
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

    fn handle_escape_input_s1(&self) -> TuiKeys {
        let input_char_option: Option<char> = self.try_read_char();
        match input_char_option {
            None => return TuiKeys::Escape,
            Some(input_char) => match input_char {
                '[' => return self.handle_escape_input_s2(),
                _ => {
                    return TuiKeys::Error;
                }
            },
        };
    }

    fn handle_escape_input_s2(&self) -> TuiKeys {
        let input_char: char = self.read_char();
        match input_char {
            'A' => return TuiKeys::UpArrow,
            'B' => return TuiKeys::DownArrow,
            'C' => return TuiKeys::RightArrow,
            'D' => return TuiKeys::LeftArrow,
            '3' => {
                if let Some(input_char) = self.try_read_char() {
                    if input_char == '~' {
                        return TuiKeys::Delete;
                    }
                }
                return TuiKeys::Error;
            }
            _ => {
                return TuiKeys::Error;
            }
        }
    }

    pub fn get_keyboard_event(&self) -> TuiKeys {
        let input_char: char = self.read_char();
        match input_char {
            '\x1b' => {
                return self.handle_escape_input_s1();
            }
            '\x7F' => {
                return TuiKeys::Backspace;
            }
            '\n' | '\r' => {
                return TuiKeys::Enter;
            }
            ' ' => {
                return TuiKeys::Space;
            }
            '\t' => {
                return TuiKeys::Tab;
            }
            _ => {
                if (input_char as u32) > 0x20 && (input_char as u32) < 0x7F {
                    return TuiKeys::AsciiReadable(input_char);
                }
                return TuiKeys::Other(input_char);
            }
        }
    }
}

pub struct OutputInterface {
    output_handle: Stdout,
}

impl OutputInterface {
    pub fn get_size(&self) -> Option<(u16, u16)> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")
            .ok()?;
        let mut window_size: winsize = winsize {
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
        _ = file.as_raw_fd();
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
    let input_interface: InputInterface = InputInterface::new();
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
