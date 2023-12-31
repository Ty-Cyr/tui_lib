use std::{
    error::Error,
    io::Write,
    sync::{Mutex, MutexGuard},
};

static TUI_TERMINAL_LOCK: Mutex<()> = Mutex::new(());

use crate::{
    font_settings::FontSettings,
    tui_enums::{CursorMode, CursorNav, TuiMode},
    tui_errors::{CError, IOError, OverflowError, TuiUnexpectedInputError},
    tui_events::TuiEvents,
    tui_io::{
        input_interface::InputInterfaceT,
        output_interface::OutputInterfaceT,
        terminal_interface::TerminalTrait,
        tui_io::{InputInterface, OutputInterface, TerminalManager, TerminalState},
    },
    Color, StringPlus, ThreeBool,
};

#[derive(Debug)]
pub struct TuiTerminal {
    font_settings: FontSettings,
    cursor_mode: CursorMode,
    output_interface: OutputInterface,
    input_interface: InputInterface,
    terminal_state: TerminalState,
    lock: MutexGuard<'static, ()>,
}

impl TuiTerminal {
    pub fn new(tui_mode: TuiMode) -> Result<TuiTerminal, Box<dyn Error>> {
        let lock: MutexGuard<'static, ()> = TUI_TERMINAL_LOCK.lock()?;
        let (input_interface, output_interface, terminal_state): (
            InputInterface,
            OutputInterface,
            TerminalState,
        ) = TerminalManager::setup_terminal()?;
        let mut tui_terminal = TuiTerminal {
            font_settings: FontSettings::default(),
            cursor_mode: CursorMode::Default,
            output_interface: output_interface,
            input_interface: input_interface,
            terminal_state: terminal_state,
            lock: lock,
        };
        tui_terminal.enable_mouse_events();
        match tui_mode {
            TuiMode::FullScreen => tui_terminal.alt_buffer(),
            _ => {}
        }
        return Ok(tui_terminal);
    }

    fn get_font_color_code(&self, mut color: Color) -> String {
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
        .into();
    }

    fn get_background_color_code(&self, mut color: Color) -> String {
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
        .into();
    }

    fn get_bold_code(&self, mut is_bold: ThreeBool) -> &str {
        if let ThreeBool::Default = is_bold {
            is_bold = self.font_settings.is_bold;
        }
        return match is_bold {
            ThreeBool::True => "1",
            ThreeBool::False | ThreeBool::Default => "22",
        };
    }

    fn get_underlined_code(&self, mut is_underlined: ThreeBool) -> &str {
        if let ThreeBool::Default = is_underlined {
            is_underlined = self.font_settings.is_underlined;
        }
        return match is_underlined {
            ThreeBool::True => "4",
            ThreeBool::False | ThreeBool::Default => "24",
        };
    }

    fn get_italics_code(&self, mut is_italics: ThreeBool) -> &str {
        if let ThreeBool::Default = is_italics {
            is_italics = self.font_settings.is_underlined;
        }
        return match is_italics {
            ThreeBool::True => "3",
            ThreeBool::False | ThreeBool::Default => "23",
        };
    }

    fn get_inverted_code(&self, mut is_inverted: ThreeBool) -> &str {
        if let ThreeBool::Default = is_inverted {
            is_inverted = self.font_settings.is_inverted;
        }
        return match is_inverted {
            ThreeBool::True => "7",
            ThreeBool::False | ThreeBool::Default => "27",
        };
    }

    fn get_blinking_code(&self, mut is_blinking: ThreeBool) -> &str {
        if let ThreeBool::Default = is_blinking {
            is_blinking = self.font_settings.is_blinking;
        }
        return match is_blinking {
            ThreeBool::True => "5",
            ThreeBool::False | ThreeBool::Default => "25",
        };
    }

    pub fn shift_cursor(&mut self, cursor_nav: CursorNav) {
        _ = self
            .output_interface
            .write(cursor_nav.get_code().as_bytes());
        _ = self.output_interface.flush();
    }

    pub fn set_cursor_position(&mut self, x: u16, y: u16) {
        _ = self
            .output_interface
            .write(("\x1b[".to_string() + &y.to_string() + ";" + &x.to_string() + "H").as_bytes());
        _ = self.output_interface.flush();
    }

    pub fn get_cursor_position(&mut self) -> Result<(u16, u16), Box<dyn Error>> {
        _ = self.output_interface.write(b"\x1b[6n");
        _ = self.output_interface.flush();
        let mut input = self.input_interface.read_raw().ok_or(IOError {})?;
        if input != '\x1b' {
            Err(TuiUnexpectedInputError {
                expected: '\x1b',
                recieved: input,
            })?;
        }
        input = self.input_interface.read_raw().ok_or(IOError {})?;
        if input != '[' {
            Err(TuiUnexpectedInputError {
                expected: '\x1b',
                recieved: input,
            })?;
        }
        let mut y: u16 = 0;
        let mut x: u16 = 0;
        loop {
            input = self.input_interface.read_raw().ok_or(IOError {})?;
            match input as u8 {
                0x30..=0x39 => {
                    let digit = (input as u16) - 0x30;
                    if u16::MAX / 10 < y {
                        Err(OverflowError {})?;
                    }
                    y *= 10;
                    if u16::MAX - y < digit {
                        Err(OverflowError {})?;
                    }
                    y += digit;
                }
                0x3B => break,
                _ => Err(TuiUnexpectedInputError {
                    expected: ';',
                    recieved: input,
                })?,
            }
        }
        loop {
            input = self.input_interface.read_raw().ok_or(IOError {})?;
            match input as u8 {
                0x30..=0x39 => {
                    let digit = (input as u16) - 0x30;
                    if u16::MAX / 10 < x {
                        Err(OverflowError {})?;
                    }
                    x *= 10;
                    if u16::MAX - x < digit {
                        Err(OverflowError {})?;
                    }
                    x += digit;
                }
                0x52 => break,
                _ => Err(TuiUnexpectedInputError {
                    expected: 'R',
                    recieved: input,
                })?,
            }
        }
        return Ok((x, y));
    }

    fn send_cursor_code(&mut self) {
        _ = self.output_interface.write(b"\x1b[?25h");
        match self.cursor_mode {
            CursorMode::BlinkingBlock => _ = self.output_interface.write(b"\x1b[1\x20q"),
            CursorMode::SteadyBlock => _ = self.output_interface.write(b"\x1b[2\x20q"),
            CursorMode::BlinkingUnderline => _ = self.output_interface.write(b"\x1b[3\x20q"),
            CursorMode::StedayUnderline => _ = self.output_interface.write(b"\x1b[4\x20q"),
            CursorMode::BlinkingBar => _ = self.output_interface.write(b"\x1b[5\x20q"),
            CursorMode::SteadyBar => _ = self.output_interface.write(b"\x1b[6\x20q"),
            CursorMode::Hidden => _ = self.output_interface.write(b"\x1b[?25l"),
            CursorMode::Default => _ = self.output_interface.write(b"\x1b[0\x20q"),
        }
        _ = self.output_interface.flush();
    }

    fn send_dec_line_code(&mut self, is_dec_line: bool) {
        match is_dec_line {
            true => _ = self.output_interface.write(b"\x1b(0"),
            false => _ = self.output_interface.write(b"\x1b(B"),
        }
    }

    fn calc_font_settings_code(&self, font_settings: &FontSettings) -> String {
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
        return code;
    }

    fn send_font_settings(&mut self, font_settings: &FontSettings) {
        let code = self.calc_font_settings_code(font_settings);
        _ = self.output_interface.write(code.as_bytes());
        self.send_dec_line_code(font_settings.is_dec_line);
        _ = self.output_interface.flush();
        self.clear_end_line();
    }

    fn send_font_settings_passive(&mut self, font_settings: &FontSettings) {
        let code = self.calc_font_settings_code(font_settings);
        _ = self.output_interface.write(code.as_bytes());
        self.send_dec_line_code(font_settings.is_dec_line);
        _ = self.output_interface.flush();
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

    pub fn get_cursor(&self) -> CursorMode {
        return self.cursor_mode.clone();
    }

    pub fn save_cursor_position(&mut self) {
        _ = self.output_interface.write(b"\x1b7");
        _ = self.output_interface.flush();
    }

    pub fn restore_cursor_position(&mut self) {
        _ = self.output_interface.write(b"\x1b8");
        _ = self.output_interface.flush();
    }

    pub fn get_font_settings(&self) -> FontSettings {
        return self.font_settings.clone();
    }

    pub fn set_font_settings(&mut self, font_settings: FontSettings) {
        self.font_settings = font_settings;
        self.send_font_settings(&self.font_settings.clone());
    }

    pub fn set_font_settings_passive(&mut self, font_settings: FontSettings) {
        self.font_settings = font_settings;
        self.send_font_settings_passive(&self.font_settings.clone());
    }

    pub fn write<T: Into<StringPlus>>(&mut self, string_plus: T) {
        let string_plus: StringPlus = string_plus.into();
        let string: String = (&string_plus).into();
        let mut line_number: usize = 0;
        for line in string.split("\n") {
            self.send_font_settings_passive(string_plus.get_font_settings());
            if line_number != 0 {
                _ = self.output_interface.write(b"\n");
            }
            _ = self.output_interface.write(line.as_bytes());
            self.send_font_settings_passive(&self.font_settings.clone());
            line_number += 1;
        }
        _ = self.output_interface.flush();
    }

    pub fn println<T: Into<StringPlus>>(&mut self, string_plus: T) {
        let string_plus: StringPlus = string_plus.into();
        let string: String = (&string_plus).into();
        for line in string.split("\n") {
            self.send_font_settings(string_plus.get_font_settings());
            _ = self.output_interface.write(line.as_bytes());
            self.send_font_settings(&self.font_settings.clone());
            _ = self.output_interface.write(b"\n");
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
                _ = self.output_interface.write(b"\n");
            }
            _ = self.output_interface.write(line.as_bytes());
            self.send_font_settings(&self.font_settings.clone());
            line_number += 1;
        }
        _ = self.output_interface.flush();
    }

    pub fn clear_screen(&mut self) {
        _ = self.output_interface.write(b"\x1b[2J");
        _ = self.output_interface.flush();
    }

    pub fn clear_end_line(&mut self) {
        _ = self.output_interface.write(b"\x1b[0K");
        _ = self.output_interface.flush();
    }

    pub fn clear_beginning_line(&mut self) {
        _ = self.output_interface.write(b"\x1b[1K");
        _ = self.output_interface.flush();
    }

    pub fn clear_line(&mut self) {
        _ = self.output_interface.write(b"\x1b[2K");
        _ = self.output_interface.flush();
    }

    pub fn get_teminal_size(&self) -> Result<(u16, u16), CError> {
        return self.output_interface.get_size();
    }

    pub fn get_event(&self) -> TuiEvents {
        return self.input_interface.read_parsed();
    }

    fn alt_buffer(&mut self) {
        _ = self.output_interface.write(b"\x1b[?1049h");
        _ = self.output_interface.flush();
    }

    fn main_buffer(&mut self) {
        _ = self.output_interface.write(b"\x1b[?1049l");
        _ = self.output_interface.flush();
    }

    pub fn default_settings(&mut self) {
        self.font_settings = FontSettings::default();
        self.send_font_settings(&FontSettings::default());
    }

    pub fn enable_mouse_events(&mut self) {
        _ = self.output_interface.write("\x1b[?1003h".as_bytes());
        _ = self.output_interface.write("\x1b[?1006h".as_bytes());
        _ = self.output_interface.write("\x1b[?1015h".as_bytes());
        _ = self.output_interface.flush();
    }

    pub fn disable_mouse_events(&mut self) {
        _ = self.output_interface.write("\x1b[?1003l".as_bytes());
        _ = self.output_interface.write("\x1b[?1006l".as_bytes());
        _ = self.output_interface.write("\x1b[?1015l".as_bytes());
        _ = self.output_interface.flush();
    }
}

impl Drop for TuiTerminal {
    fn drop(&mut self) {
        self.cursor_mode = CursorMode::Default;
        self.send_cursor_code();
        self.send_font_settings(&FontSettings::default());
        self.main_buffer();
        self.disable_mouse_events();
        TerminalManager::reset_terminal_settings(&self.input_interface, &self.terminal_state);
        _ = self.output_interface.flush();
        let _lock = &self.lock;
    }
}
