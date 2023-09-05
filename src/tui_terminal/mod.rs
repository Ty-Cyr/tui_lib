use std::io::Write;

use crate::{
    font_settings::FontSettings,
    os_tui::{
        reset_terminal_settings, setup_terminal, InputInterface, OutputInterface, TerminalState,
    },
    tui_enums::{CursorMode, TuiMode},
    tui_keys::TuiKeys,
    Color, StringPlus, ThreeBool,
};

#[allow(unused)]
pub struct TuiTerminal {
    font_settings: FontSettings,
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
            font_settings: FontSettings::default(),
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
    fn get_font_color_code(&mut self, mut color: Color) -> String {
        if let Color::Default = color {
            color = self.font_settings.font_color;
        }
        let ascii_code: String;
        return match color {
            Color::White => "37",
            Color::BrightWhite => "97",
            Color::Black => "30",
            Color::BrightBlack => "90",
            Color::Red => "31",
            Color::BrightRed => "91",
            Color::Green => "32",
            Color::BrightGreen => "92",
            Color::Blue => "34",
            Color::BrightBlue => "94",
            Color::Yellow => "33",
            Color::BrightYellow => "93",
            Color::Magenta => "35",
            Color::BrightMagenta => "95",
            Color::Cyan => "36",
            Color::BrightCyan => "96",
            Color::CC256(code) => {
                ascii_code = "38;5;".to_string() + &code.to_string();
                ascii_code.as_str()
            }
            Color::RGB(r, g, b) => {
                ascii_code = "38;2;".to_string()
                    + &r.to_string()
                    + ";"
                    + &g.to_string()
                    + ";"
                    + &b.to_string();
                ascii_code.as_str()
            }
            Color::Default => "39",
        }
        .to_string();
    }

    fn get_background_color_code(&mut self, mut color: Color) -> String {
        if let Color::Default = color {
            color = self.font_settings.background_color;
        }
        let ascii_code: String;
        return match color {
            Color::White => "47",
            Color::BrightWhite => "107",
            Color::Black => "40",
            Color::BrightBlack => "100",
            Color::Red => "41",
            Color::BrightRed => "101",
            Color::Green => "42",
            Color::BrightGreen => "102",
            Color::Blue => "44",
            Color::BrightBlue => "104",
            Color::Yellow => "43",
            Color::BrightYellow => "103",
            Color::Magenta => "45",
            Color::BrightMagenta => "105",
            Color::Cyan => "46",
            Color::BrightCyan => "106",
            Color::CC256(code) => {
                ascii_code = "48;5;".to_string() + &code.to_string();
                ascii_code.as_str()
            }
            Color::RGB(r, g, b) => {
                ascii_code = "48;2;".to_string()
                    + &r.to_string()
                    + ";"
                    + &g.to_string()
                    + ";"
                    + &b.to_string();
                ascii_code.as_str()
            }
            Color::Default => "49",
        }
        .to_string();
    }

    fn get_bold_code(&mut self, mut is_bold: ThreeBool) -> &str {
        if let ThreeBool::Default = is_bold {
            is_bold = self.font_settings.is_bold;
        }
        return match is_bold {
            ThreeBool::True => "1",
            ThreeBool::False | ThreeBool::Default => "22",
        };
    }

    fn get_underlined_code(&mut self, mut is_underlined: ThreeBool) -> &str {
        if let ThreeBool::Default = is_underlined {
            is_underlined = self.font_settings.is_underlined;
        }
        return match is_underlined {
            ThreeBool::True => "4",
            ThreeBool::False | ThreeBool::Default => "24",
        };
    }

    fn get_italics_code(&mut self, mut is_italics: ThreeBool) -> &str {
        if let ThreeBool::Default = is_italics {
            is_italics = self.font_settings.is_underlined;
        }
        return match is_italics {
            ThreeBool::True => "3",
            ThreeBool::False | ThreeBool::Default => "23",
        };
    }

    fn get_inverted_code(&mut self, mut is_inverted: ThreeBool) -> &str {
        if let ThreeBool::Default = is_inverted {
            is_inverted = self.font_settings.is_inverted;
        }
        return match is_inverted {
            ThreeBool::True => "7",
            ThreeBool::False | ThreeBool::Default => "27",
        };
    }

    fn get_blinking_code(&mut self, mut is_blinking: ThreeBool) -> &str {
        if let ThreeBool::Default = is_blinking {
            is_blinking = self.font_settings.is_blinking;
        }
        return match is_blinking {
            ThreeBool::True => "5",
            ThreeBool::False | ThreeBool::Default => "25",
        };
    }

    fn send_cursor_code(&mut self) {
        _ = self.output_interface.write("\x1b[?25h".as_bytes());
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
            CursorMode::Hidden => _ = self.output_interface.write("\x1b[?25l".as_bytes()),
            CursorMode::Default => _ = self.output_interface.write("\x1b[0\x20q".as_bytes()),
        }
        _ = self.output_interface.flush();
    }

    fn send_dec_line_code(&mut self, is_dec_line: bool) {
        match is_dec_line {
            true => _ = self.output_interface.write("\x1b(0".as_bytes()),
            false => _ = self.output_interface.write("\x1b(B".as_bytes()),
        }
    }

    fn send_font_settings(&mut self, font_settings: &FontSettings) {
        let mut code: String = String::from("\x1b[");
        code += self.get_font_color_code(font_settings.font_color).as_str();
        code += ";";
        code += self
            .get_background_color_code(font_settings.background_color)
            .as_str();
        code += ";";
        code += self.get_bold_code(font_settings.is_bold);
        code += ";";
        code += self.get_underlined_code(font_settings.is_underlined);
        code += ";";
        code += self.get_italics_code(font_settings.is_italics);
        code += ";";
        code += self.get_inverted_code(font_settings.is_inverted);
        code += ";";
        code += self.get_blinking_code(font_settings.is_blinking);
        code += "m";
        _ = self.output_interface.write(code.as_bytes());
        self.send_dec_line_code(font_settings.is_dec_line);
        _ = self.output_interface.flush();
        self.clear_end_line();
    }

    pub fn set_font_color(&mut self, color: Color) {
        self.font_settings.font_color = color;
        self.send_font_settings(&self.font_settings.clone());
    }
    pub fn set_background_color(&mut self, color: Color) {
        self.font_settings.background_color = color;
        self.send_font_settings(&self.font_settings.clone());
    }

    pub fn set_bold(&mut self, is_bold: ThreeBool) {
        self.font_settings.is_bold = is_bold;
    }

    pub fn set_underlined(&mut self, is_underlined: ThreeBool) {
        self.font_settings.is_underlined = is_underlined;
    }

    pub fn set_italics(&mut self, is_italics: ThreeBool) {
        self.font_settings.is_italics = is_italics;
    }

    pub fn set_inverted(&mut self, is_inverted: ThreeBool) {
        self.font_settings.is_inverted = is_inverted;
    }

    pub fn set_blinking(&mut self, is_blinking: ThreeBool) {
        self.font_settings.is_blinking = is_blinking;
    }

    pub fn set_cursor(&mut self, cursor_mode: CursorMode) {
        self.cursor_mode = cursor_mode;
        self.send_cursor_code();
    }

    pub fn println<T: Into<StringPlus>>(&mut self, string_plus: T) {
        let string_plus: StringPlus = string_plus.into();
        let string: String = (&string_plus).into();
        for line in string.split("\n") {
            self.send_font_settings(string_plus.get_font_settings());
            _ = self.output_interface.write(line.as_bytes());
            self.send_font_settings(&self.font_settings.clone());
            _ = self.output_interface.write("\n".as_bytes());
        }
        _ = self.output_interface.flush();
    }

    pub fn print<T: Into<StringPlus>>(&mut self, string_plus: T) {
        let string_plus: StringPlus = string_plus.into();
        let string: String = (&string_plus).into();
        let mut line_number: usize = 0;
        for line in string.split("\n") {
            self.send_font_settings(string_plus.get_font_settings());
            if line_number != 0 {
                _ = self.output_interface.write("\n".as_bytes());
            }
            _ = self.output_interface.write(line.as_bytes());
            self.send_font_settings(&self.font_settings.clone());
            line_number += 1;
        }
        _ = self.output_interface.flush();
    }

    pub fn clear_screen(&mut self) {
        _ = self.output_interface.write("\x1bc".as_bytes());
        _ = self.output_interface.flush();
    }

    pub fn clear_end_line(&mut self) {
        _ = self.output_interface.write("\x1b[0K".as_bytes());
        _ = self.output_interface.flush();
    }

    pub fn clear_beginning_line(&mut self) {
        _ = self.output_interface.write("\x1b[1K".as_bytes());
        _ = self.output_interface.flush();
    }

    pub fn clear_line(&mut self) {
        _ = self.output_interface.write("\x1b[2K".as_bytes());
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

    pub fn default_settings(&mut self) {
        self.font_settings = FontSettings::default();
        self.send_font_settings(&FontSettings::default());
    }
}

impl Drop for TuiTerminal {
    fn drop(&mut self) {
        self.cursor_mode = CursorMode::Default;
        self.send_cursor_code();
        self.send_font_settings(&FontSettings::default());
        self.main_buffer();
        reset_terminal_settings(&self.input_interface, &self.terminal_state);
    }
}
