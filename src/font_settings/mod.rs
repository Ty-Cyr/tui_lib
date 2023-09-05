use crate::tui_enums::{Color, ThreeBool};

#[derive(Clone, Copy)]
pub struct FontSettings {
    pub font_color: Color,
    pub background_color: Color,
    pub is_bold: ThreeBool,
    pub is_underlined: ThreeBool,
    pub is_inverted: ThreeBool,
    pub is_blinking: ThreeBool,
    pub is_dec_line: bool,
}

impl Default for FontSettings {
    fn default() -> FontSettings {
        FontSettings {
            font_color: Color::Default,
            background_color: Color::Default,
            is_bold: ThreeBool::Default,
            is_underlined: ThreeBool::Default,
            is_inverted: ThreeBool::Default,
            is_blinking: ThreeBool::Default,
            is_dec_line: false,
        }
    }
}
