use std::io::Write;

use crate::{
    os_tui::{
        reset_terminal_settings, setup_terminal, InputInterface, OutputInterface, TerminalState,
    },
    tui_enums::{CursorMode, TuiMode},
    tui_keys::TuiKeys,
    Color, StringPlus, ThreeBool,
};

#[allow(unused)]
pub struct TuiTerminal {
    font_color: Color,
    background_color: Color,
    is_bold: ThreeBool,
    is_underlined: ThreeBool,
    is_inverted: ThreeBool,
    is_blinking: ThreeBool,
    cursor_mode: CursorMode,
    output_interface: OutputInterface,
    input_interface: InputInterface,
    terminal_state: TerminalState,
}

impl TuiTerminal {
    pub fn new(tui_mode: TuiMode) -> Option<TuiTerminal> {
        let (input_interface, output_interface, terminal_state): (
            InputInterface,
            OutputInterface,
            TerminalState,
        ) = setup_terminal()?;
        let mut tui_terminal = TuiTerminal {
            font_color: Color::Default,
            background_color: Color::Default,
            is_bold: ThreeBool::Default,
            is_underlined: ThreeBool::Default,
            is_inverted: ThreeBool::Default,
            is_blinking: ThreeBool::Default,
            cursor_mode: CursorMode::Default,
            output_interface: output_interface,
            input_interface: input_interface,
            terminal_state: terminal_state,
        };
        match tui_mode {
            TuiMode::FullScreen => tui_terminal.alt_buffer(),
            _ => {}
        }
        return Some(tui_terminal);
    }
    fn send_font_color_code(&mut self, color: Color) {
        match color {
            Color::White => _ = self.output_interface.write("\x1b[37m".as_bytes()),
            Color::BrightWhite => _ = self.output_interface.write("\x1b[97m".as_bytes()),
            Color::Black => _ = self.output_interface.write("\x1b[30m".as_bytes()),
            Color::BrightBlack => _ = self.output_interface.write("\x1b[90m".as_bytes()),
            Color::Red => _ = self.output_interface.write("\x1b[31m".as_bytes()),
            Color::BrightRed => _ = self.output_interface.write("\x1b[91m".as_bytes()),
            Color::Green => _ = self.output_interface.write("\x1b[32m".as_bytes()),
            Color::BrightGreen => _ = self.output_interface.write("\x1b[92m".as_bytes()),
            Color::Blue => _ = self.output_interface.write("\x1b[34m".as_bytes()),
            Color::BrightBlue => _ = self.output_interface.write("\x1b[94m".as_bytes()),
            Color::Yellow => _ = self.output_interface.write("\x1b[33m".as_bytes()),
            Color::BrightYellow => _ = self.output_interface.write("\x1b[93m".as_bytes()),
            Color::Magenta => _ = self.output_interface.write("\x1b[35m".as_bytes()),
            Color::BrightMagenta => _ = self.output_interface.write("\x1b[95m".as_bytes()),
            Color::Cyan => _ = self.output_interface.write("\x1b[36m".as_bytes()),
            Color::BrightCyan => _ = self.output_interface.write("\x1b[96m".as_bytes()),
            Color::CC256(code) => {
                _ = self
                    .output_interface
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
                _ = self.output_interface.write(code.as_bytes());
            }
            Color::Default => {}
        }
    }
    fn send_background_color_code(&mut self, color: Color) {
        match color {
            Color::White => _ = self.output_interface.write("\x1b[47m".as_bytes()),
            Color::BrightWhite => _ = self.output_interface.write("\x1b[107m".as_bytes()),
            Color::Black => _ = self.output_interface.write("\x1b[40m".as_bytes()),
            Color::BrightBlack => _ = self.output_interface.write("\x1b[100m".as_bytes()),
            Color::Red => _ = self.output_interface.write("\x1b[41m".as_bytes()),
            Color::BrightRed => _ = self.output_interface.write("\x1b[101m".as_bytes()),
            Color::Green => _ = self.output_interface.write("\x1b[42m".as_bytes()),
            Color::BrightGreen => _ = self.output_interface.write("\x1b[102m".as_bytes()),
            Color::Blue => _ = self.output_interface.write("\x1b[44m".as_bytes()),
            Color::BrightBlue => _ = self.output_interface.write("\x1b[104m".as_bytes()),
            Color::Yellow => _ = self.output_interface.write("\x1b[43m".as_bytes()),
            Color::BrightYellow => _ = self.output_interface.write("\x1b[103m".as_bytes()),
            Color::Magenta => _ = self.output_interface.write("\x1b[45m".as_bytes()),
            Color::BrightMagenta => _ = self.output_interface.write("\x1b[105m".as_bytes()),
            Color::Cyan => _ = self.output_interface.write("\x1b[46m".as_bytes()),
            Color::BrightCyan => _ = self.output_interface.write("\x1b[106m".as_bytes()),
            Color::CC256(code) => {
                _ = self
                    .output_interface
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
                _ = self.output_interface.write(code.as_bytes());
            }
            Color::Default => {}
        }
        _ = self.output_interface.write("\x1b[0K".as_bytes());
    }

    fn send_bold_code(&mut self, is_bold: ThreeBool) {
        match is_bold {
            ThreeBool::True => _ = self.output_interface.write("\x1b[1m".as_bytes()),
            ThreeBool::False => {}
            ThreeBool::Default => match self.is_bold {
                ThreeBool::True => _ = self.output_interface.write("\x1b[1m".as_bytes()),
                _ => {}
            },
        }
    }

    fn send_underlined_code(&mut self, is_underlined: ThreeBool) {
        match is_underlined {
            ThreeBool::True => _ = self.output_interface.write("\x1b[4m".as_bytes()),
            ThreeBool::False => {}
            ThreeBool::Default => match self.is_underlined {
                ThreeBool::True => _ = self.output_interface.write("\x1b[4m".as_bytes()),
                _ => {}
            },
        }
    }

    fn send_inverted_code(&mut self, is_inverted: ThreeBool) {
        match is_inverted {
            ThreeBool::True => _ = self.output_interface.write("\x1b[7m".as_bytes()),
            ThreeBool::False => {}
            ThreeBool::Default => match self.is_inverted {
                ThreeBool::True => _ = self.output_interface.write("\x1b[7m".as_bytes()),
                _ => {}
            },
        }
    }

    fn send_blinking_code(&mut self, is_blinking: ThreeBool) {
        match is_blinking {
            ThreeBool::True => _ = self.output_interface.write("\x1b[5m".as_bytes()),
            ThreeBool::False => {}
            ThreeBool::Default => match self.is_blinking {
                ThreeBool::True => _ = self.output_interface.write("\x1b[5m".as_bytes()),
                _ => {}
            },
        }
    }

    fn send_cursor_code(&mut self) {
        match self.cursor_mode {
            CursorMode::BlinkingBlock => _ = self.output_interface.write("\x1b[1\x20q".as_bytes()),
            CursorMode::SteadyBlock => _ = self.output_interface.write("\x1b[2\x20q".as_bytes()),
            CursorMode::BlinkingUnderline => {
                _ = self.output_interface.write("\x1b[3\x20q".as_bytes())
            }
            CursorMode::StedayUnderline => {
                _ = self.output_interface.write("\x1b[4\x20q".as_bytes())
            }
            CursorMode::BlinkingBar => _ = self.output_interface.write("\x1b[5\x20q".as_bytes()),
            CursorMode::SteadyBar => _ = self.output_interface.write("\x1b[6\x20q".as_bytes()),
            CursorMode::Default => _ = self.output_interface.write("\x1b[0\x20q".as_bytes()),
        }
    }

    fn send_dec_line_code(&mut self, is_dec_line: bool) {
        match is_dec_line {
            true => _ = self.output_interface.write("\x1b(0".as_bytes()),
            false => _ = self.output_interface.write("\x1b(B".as_bytes()),
        }
    }

    fn reset_font_settings(&mut self) {
        _ = self.output_interface.write("\x1b[m".as_bytes());
        self.send_font_color_code(self.font_color);
        self.send_background_color_code(self.background_color);
        self.send_bold_code(self.is_bold);
        self.send_underlined_code(self.is_underlined);
        self.send_inverted_code(self.is_inverted);
        self.send_blinking_code(self.is_blinking);
        self.send_dec_line_code(false);
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

    pub fn set_blinking(&mut self, is_blinking: ThreeBool) {
        self.is_blinking = is_blinking;
    }

    pub fn set_cursor(&mut self, cursor_mode: CursorMode) {
        self.cursor_mode = cursor_mode;
        self.send_cursor_code();
    }

    fn send_string_plus_codes(&mut self, string_plus: &StringPlus) {
        self.reset_font_settings();
        self.send_font_color_code(string_plus.get_font_color());
        self.send_background_color_code(string_plus.get_background_color());
        self.send_bold_code(string_plus.get_bold());
        self.send_underlined_code(string_plus.get_underlined());
        self.send_inverted_code(string_plus.get_inverted());
        self.send_blinking_code(string_plus.get_blinking());
        self.send_dec_line_code(string_plus.get_dec_line());
    }

    pub fn println<T: Into<StringPlus>>(&mut self, string_plus: T) {
        let string_plus: StringPlus = string_plus.into();
        let string: String = (&string_plus).into();
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
        let string: String = (&string_plus).into();
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
        _ = self.output_interface.write("\x1bc".as_bytes());
        _ = self.output_interface.flush();
    }

    pub fn clear_line(&mut self) {
        _ = self.output_interface.write("\x1b[0K".as_bytes());
        _ = self.output_interface.flush();
    }

    pub fn get_teminal_size(&self) -> Option<(u16, u16)> {
        return self.output_interface.get_size();
    }

    pub fn get_keyboard_event(&self) -> TuiKeys {
        return self.input_interface.get_keyboard_event();
    }

    fn alt_buffer(&mut self) {
        _ = self.output_interface.write("\x1b[?1049h".as_bytes());
        _ = self.output_interface.write("\x1b[0;0f".as_bytes());
        _ = self.output_interface.flush();
    }

    fn main_buffer(&mut self) {
        _ = self.output_interface.write("\x1b[?1049l".as_bytes());
        _ = self.output_interface.flush();
    }
}

impl Drop for TuiTerminal {
    fn drop(&mut self) {
        self.font_color = Color::Default;
        self.background_color = Color::Default;
        self.is_bold = ThreeBool::Default;
        self.is_underlined = ThreeBool::Default;
        self.is_inverted = ThreeBool::Default;
        self.is_blinking = ThreeBool::Default;
        self.cursor_mode = CursorMode::Default;
        self.send_cursor_code();
        self.reset_font_settings();
        self.main_buffer();
        reset_terminal_settings(&self.input_interface, &self.terminal_state);
    }
}
