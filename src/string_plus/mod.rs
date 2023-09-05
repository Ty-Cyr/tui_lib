use crate::{Color, ThreeBool};

pub enum DecLine {
    TopLeft,
    HorizontalBar,
    TopMiddle,
    TopRight,
    VerticalBar,
    MiddleLeft,
    MiddleMiddle,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

impl DecLine {
    fn get_code(&self) -> String {
        return match self {
            &DecLine::TopLeft => '\x6c',
            &DecLine::TopMiddle => '\x77',
            &DecLine::TopRight => '\x6b',
            &DecLine::MiddleLeft => '\x74',
            &DecLine::MiddleMiddle => '\x6e',
            &DecLine::MiddleRight => '\x75',
            &DecLine::BottomLeft => '\x6d',
            &DecLine::BottomMiddle => '\x76',
            &DecLine::BottomRight => '\x6a',
            &DecLine::VerticalBar => '\x78',
            &DecLine::HorizontalBar => '\x71',
        }
        .into();
    }
}

#[derive(Clone)]
pub struct StringPlus {
    string: String,
    font_color: Color,
    background_color: Color,
    is_bold: ThreeBool,
    is_underlined: ThreeBool,
    is_inverted: ThreeBool,
    is_blinking: ThreeBool,
    is_dec_line: bool,
}

impl StringPlus {
    pub fn dec_line(line: DecLine) -> StringPlus {
        return StringPlus {
            string: line.get_code(),
            font_color: Color::Default,
            background_color: Color::Default,
            is_bold: ThreeBool::Default,
            is_underlined: ThreeBool::Default,
            is_inverted: ThreeBool::Default,
            is_blinking: ThreeBool::Default,
            is_dec_line: true,
        };
    }
    pub fn get_font_color(&self) -> Color {
        return self.font_color;
    }

    pub fn set_font_color(mut self, color: Color) -> StringPlus {
        self.font_color = color;
        return self;
    }

    pub fn get_background_color(&self) -> Color {
        return self.background_color;
    }

    pub fn get_bold(&self) -> ThreeBool {
        return self.is_bold;
    }

    pub fn get_underlined(&self) -> ThreeBool {
        return self.is_underlined;
    }

    pub fn get_inverted(&self) -> ThreeBool {
        return self.is_inverted;
    }

    pub fn get_blinking(&self) -> ThreeBool {
        return self.is_blinking;
    }

    pub fn get_dec_line(&self) -> bool {
        return self.is_dec_line;
    }
}

pub trait StringPlusTrait {
    fn set_font_color(self, color: Color) -> StringPlus;
    fn set_background_color(self, color: Color) -> StringPlus;
    fn set_bold(self, is_bold: ThreeBool) -> StringPlus;
    fn set_underlined(self, is_underlined: ThreeBool) -> StringPlus;
    fn set_inverted(self, is_inverted: ThreeBool) -> StringPlus;
    fn set_blinking(self, is_blinking: ThreeBool) -> StringPlus;
}

impl<T: Into<StringPlus>> StringPlusTrait for T {
    fn set_font_color(self, color: Color) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.font_color = color;
        return string_plus;
    }
    fn set_background_color(self, color: Color) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.background_color = color;
        return string_plus;
    }

    fn set_bold(self, is_bold: ThreeBool) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.is_bold = is_bold;
        return string_plus;
    }

    fn set_underlined(self, is_underlined: ThreeBool) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.is_underlined = is_underlined;
        return string_plus;
    }

    fn set_inverted(self, is_inverted: ThreeBool) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.is_inverted = is_inverted;
        return string_plus;
    }

    fn set_blinking(self, is_blinking: ThreeBool) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.is_blinking = is_blinking;
        return string_plus;
    }
}

impl From<&StringPlus> for String {
    fn from(value: &StringPlus) -> Self {
        return value.string.to_owned();
    }
}

impl ToString for StringPlus {
    fn to_string(&self) -> String {
        return self.into();
    }
}

impl From<char> for StringPlus {
    fn from(value: char) -> StringPlus {
        return value.to_string().into();
    }
}

impl From<&str> for StringPlus {
    fn from(value: &str) -> StringPlus {
        return StringPlus {
            string: value.to_string(),
            font_color: Color::Default,
            background_color: Color::Default,
            is_bold: ThreeBool::Default,
            is_underlined: ThreeBool::Default,
            is_inverted: ThreeBool::Default,
            is_blinking: ThreeBool::Default,
            is_dec_line: false,
        };
    }
}

impl From<String> for StringPlus {
    fn from(value: String) -> Self {
        return StringPlus {
            string: value,
            font_color: Color::Default,
            background_color: Color::Default,
            is_bold: ThreeBool::Default,
            is_underlined: ThreeBool::Default,
            is_inverted: ThreeBool::Default,
            is_blinking: ThreeBool::Default,
            is_dec_line: false,
        };
    }
}

pub trait AsSp {
    fn as_sp(self) -> StringPlus;
}

impl<T: ToString> AsSp for T {
    fn as_sp(self) -> StringPlus {
        return self.to_string().into();
    }
}
