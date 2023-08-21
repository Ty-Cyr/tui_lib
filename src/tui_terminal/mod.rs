use crate::{
    os_tui::{reset_terminal_settings, setup_terminal, InputInterface, TerminalState},
    tui_enums::TuiMode,
    tui_keys::TuiKeys,
    Color, StringPlus, ThreeBool,
};
use std::io::{stdout, Stdout, StdoutLock, Write};

#[allow(unused)]
pub struct TuiTerminal {
    font_color: Color,
    background_color: Color,
    is_bold: ThreeBool,
    is_underlined: ThreeBool,
    is_inverted: ThreeBool,
    output_interface: Stdout,
    input_interface: InputInterface,
    terminal_state: TerminalState,
}

impl TuiTerminal {
    pub fn new(tui_mode: TuiMode) -> Option<TuiTerminal> {
        let (input_interface, terminal_state): (InputInterface, TerminalState) =
            setup_terminal(&tui_mode)?;
        return Some(TuiTerminal {
            font_color: Color::Default,
            background_color: Color::Default,
            is_bold: ThreeBool::Default,
            is_underlined: ThreeBool::Default,
            is_inverted: ThreeBool::Default,
            output_interface: stdout(),
            input_interface: input_interface,
            terminal_state: terminal_state,
        });
    }
    fn send_font_color_code(&self, color: Color) {
        let mut output_lock: StdoutLock = self.output_interface.lock();
        match color {
            Color::White => _ = output_lock.write("\x1b[37m".as_bytes()),
            Color::Black => _ = output_lock.write("\x1b[30m".as_bytes()),
            Color::Red => _ = output_lock.write("\x1b[31m".as_bytes()),
            Color::Green => _ = output_lock.write("\x1b[32m".as_bytes()),
            Color::Blue => _ = output_lock.write("\x1b[34m".as_bytes()),
            Color::Yellow => _ = output_lock.write("\x1b[33m".as_bytes()),
            Color::Magenta => _ = output_lock.write("\x1b[35m".as_bytes()),
            Color::Cyan => _ = output_lock.write("\x1b[36m".as_bytes()),
            Color::CC256(code) => {
                _ = output_lock
                    .write(("\x1b[38;5;".to_owned() + &code.to_string() + "m").as_bytes());
            }
            Color::RGB(r, g, b) => {
                let code: String = String::from("\x1b[38;2;")
                    + &r.to_string()
                    + ";"
                    + &g.to_string()
                    + ";"
                    + &b.to_string()
                    + "m";
                _ = output_lock.write(code.as_bytes());
            }
            _ => {}
        }
    }
    fn send_background_color_code(&self, color: Color) {
        let mut output_lock: StdoutLock = self.output_interface.lock();
        match color {
            Color::White => _ = output_lock.write("\x1b[47m".as_bytes()),
            Color::Black => _ = output_lock.write("\x1b[40m".as_bytes()),
            Color::Red => _ = output_lock.write("\x1b[41m".as_bytes()),
            Color::Green => _ = output_lock.write("\x1b[42m".as_bytes()),
            Color::Blue => _ = output_lock.write("\x1b[44m".as_bytes()),
            Color::Yellow => _ = output_lock.write("\x1b[43m".as_bytes()),
            Color::Magenta => _ = output_lock.write("\x1b[45m".as_bytes()),
            Color::Cyan => _ = output_lock.write("\x1b[46m".as_bytes()),
            Color::CC256(code) => {
                _ = output_lock
                    .write(("\x1b[48;5;".to_owned() + &code.to_string() + "m").as_bytes());
            }
            Color::RGB(r, g, b) => {
                let code: String = String::from("\x1b[48;2;")
                    + &r.to_string()
                    + ";"
                    + &g.to_string()
                    + ";"
                    + &b.to_string()
                    + "m";
                _ = output_lock.write(code.as_bytes());
            }
            _ => {}
        }
        _ = output_lock.write("\x1b[0K".as_bytes());
    }

    fn send_bold_code(&mut self, is_bold: ThreeBool) {
        let mut output_lock: StdoutLock = self.output_interface.lock();
        match is_bold {
            ThreeBool::True => _ = output_lock.write("\x1b[1m".as_bytes()),
            ThreeBool::False => {}
            ThreeBool::Default => match self.is_bold {
                ThreeBool::True => _ = output_lock.write("\x1b[1m".as_bytes()),
                _ => {}
            },
        }
    }

    fn send_underlined_code(&mut self, is_underlined: ThreeBool) {
        let mut output_lock: StdoutLock = self.output_interface.lock();
        match is_underlined {
            ThreeBool::True => _ = output_lock.write("\x1b[4m".as_bytes()),
            ThreeBool::False => {}
            ThreeBool::Default => match self.is_underlined {
                ThreeBool::True => _ = output_lock.write("\x1b[4m".as_bytes()),
                _ => {}
            },
        }
    }

    fn send_inverted_code(&mut self, is_inverted: ThreeBool) {
        let mut output_lock: StdoutLock = self.output_interface.lock();
        match is_inverted {
            ThreeBool::True => _ = output_lock.write("\x1b[7m".as_bytes()),
            ThreeBool::False => {}
            ThreeBool::Default => match self.is_inverted {
                ThreeBool::True => _ = output_lock.write("\x1b[7m".as_bytes()),
                _ => {}
            },
        }
    }

    fn reset_font_settings(&mut self) {
        _ = self.output_interface.write("\x1b[m".as_bytes());
        self.send_font_color_code(self.font_color);
        self.send_background_color_code(self.background_color);
        self.send_bold_code(self.is_bold);
        self.send_underlined_code(self.is_underlined);
        self.send_inverted_code(self.is_inverted);
        _ = self.output_interface.flush();
    }

    pub fn set_font_color(&mut self, color: Color) {
        self.font_color = color;
        self.reset_font_settings();
    }
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
        self.reset_font_settings();
    }

    pub fn set_bold(&mut self, is_bold: ThreeBool) {
        self.is_bold = is_bold;
    }

    pub fn set_underlined(&mut self, is_underlined: ThreeBool) {
        self.is_underlined = is_underlined;
    }

    pub fn set_inverted(&mut self, is_inverted: ThreeBool) {
        self.is_inverted = is_inverted;
    }

    fn send_string_plus_codes(&mut self, string_plus: &StringPlus) {
        self.reset_font_settings();
        self.send_font_color_code(string_plus.get_font_color());
        self.send_background_color_code(string_plus.get_background_color());
        self.send_bold_code(string_plus.get_bold());
        self.send_underlined_code(string_plus.get_underlined());
        self.send_inverted_code(string_plus.get_inverted());
    }

    pub fn println<T: Into<StringPlus>>(&mut self, string_plus: T) {
        let string_plus: StringPlus = string_plus.into();
        let string: String = string_plus.clone().into();
        for line in string.split("\n") {
            self.send_string_plus_codes(&string_plus);
            _ = self.output_interface.write(line.as_bytes());
            self.reset_font_settings();
            _ = self.output_interface.write("\n".as_bytes());
        }
        _ = self.output_interface.flush();
    }

    pub fn print<T: Into<StringPlus>>(&mut self, string_plus: T) {
        let string_plus: StringPlus = string_plus.into();
        let string: String = string_plus.clone().into();
        let mut line_number: usize = 0;
        for line in string.split("\n") {
            self.send_string_plus_codes(&string_plus);
            if line_number != 0 {
                _ = self.output_interface.write("\n".as_bytes());
            }
            _ = self.output_interface.write(line.as_bytes());
            self.reset_font_settings();
            line_number += 1;
        }
        _ = self.output_interface.flush();
    }

    pub fn clear_screen(&mut self) {
        let mut output_lock: StdoutLock = self.output_interface.lock();
        _ = output_lock.write("\x1bc".as_bytes());
        _ = output_lock.flush();
    }

    pub fn clear_line(&mut self) {
        let mut output_lock: StdoutLock = self.output_interface.lock();
        _ = output_lock.write("\x1b[0K".as_bytes());
        _ = output_lock.flush();
    }

    pub fn get_teminal_size(&self) -> Option<(u32, u32)> {
        return self.input_interface.get_size();
    }

    pub fn get_keyboard_event(&self) -> TuiKeys {
        return self.input_interface.get_keyboard_event();
    }
}

impl Drop for TuiTerminal {
    fn drop(&mut self) {
        self.font_color = Color::Default;
        self.background_color = Color::Default;
        self.is_bold = ThreeBool::Default;
        self.is_underlined = ThreeBool::Default;
        self.is_inverted = ThreeBool::Default;
        self.reset_font_settings();
        reset_terminal_settings(&self.input_interface, &self.terminal_state);
    }
}
