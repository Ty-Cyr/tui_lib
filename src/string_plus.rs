use crate::{font_settings::FontSettings, Color, ThreeBool};

#[derive(Clone, Copy, Debug)]
pub enum DecCharSet {
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
    Block,
    LessEqual,
    GreaterEqual,
    NotEqual,
    Other(char),
}

impl DecCharSet {
    fn get_code(&self) -> String {
        return match self {
            &DecCharSet::TopLeft => '\x6c',
            &DecCharSet::TopMiddle => '\x77',
            &DecCharSet::TopRight => '\x6b',
            &DecCharSet::MiddleLeft => '\x74',
            &DecCharSet::MiddleMiddle => '\x6e',
            &DecCharSet::MiddleRight => '\x75',
            &DecCharSet::BottomLeft => '\x6d',
            &DecCharSet::BottomMiddle => '\x76',
            &DecCharSet::BottomRight => '\x6a',
            &DecCharSet::VerticalBar => '\x78',
            &DecCharSet::HorizontalBar => '\x71',
            &DecCharSet::Block => '\x61',
            &DecCharSet::LessEqual => '\x79',
            &DecCharSet::GreaterEqual => '\x7a',
            &DecCharSet::NotEqual => '\x7c',
            &DecCharSet::Other(code) => code,
        }
        .into();
    }
}

#[derive(Clone, Debug)]
pub struct StringPlus {
    string: String,
    font_settings: FontSettings,
}

impl StringPlus {
    pub fn get_font_color(&self) -> Color {
        return self.font_settings.font_color;
    }

    pub fn set_font_color(mut self, color: Color) -> StringPlus {
        self.font_settings.font_color = color;
        return self;
    }

    pub fn get_background_color(&self) -> Color {
        return self.font_settings.background_color;
    }

    pub fn get_bold(&self) -> ThreeBool {
        return self.font_settings.is_bold;
    }

    pub fn get_underlined(&self) -> ThreeBool {
        return self.font_settings.is_underlined;
    }

    pub fn get_inverted(&self) -> ThreeBool {
        return self.font_settings.is_inverted;
    }

    pub fn get_blinking(&self) -> ThreeBool {
        return self.font_settings.is_blinking;
    }

    pub fn get_font_settings(&self) -> &FontSettings {
        return &self.font_settings;
    }

    pub fn get_dec_line(&self) -> bool {
        return self.font_settings.is_dec_line;
    }

    pub fn len(&self) -> usize {
        return self.string.len();
    }
}

pub trait StringPlusTrait {
    fn set_font_color(self, color: Color) -> StringPlus;
    fn set_background_color(self, color: Color) -> StringPlus;
    fn set_bold(self, is_bold: ThreeBool) -> StringPlus;
    fn set_underlined(self, is_underlined: ThreeBool) -> StringPlus;
    fn set_italics(self, is_blinking: ThreeBool) -> StringPlus;
    fn set_inverted(self, is_inverted: ThreeBool) -> StringPlus;
    fn set_blinking(self, is_blinking: ThreeBool) -> StringPlus;
    fn set_font_settings(self, font_settings: FontSettings) -> StringPlus;
}

impl<T: Into<StringPlus>> StringPlusTrait for T {
    fn set_font_color(self, color: Color) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.font_settings.font_color = color;
        return string_plus;
    }
    fn set_background_color(self, color: Color) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.font_settings.background_color = color;
        return string_plus;
    }

    fn set_bold(self, is_bold: ThreeBool) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.font_settings.is_bold = is_bold;
        return string_plus;
    }

    fn set_underlined(self, is_underlined: ThreeBool) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.font_settings.is_underlined = is_underlined;
        return string_plus;
    }

    fn set_italics(self, is_italics: ThreeBool) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.font_settings.is_italics = is_italics;
        return string_plus;
    }

    fn set_inverted(self, is_inverted: ThreeBool) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.font_settings.is_inverted = is_inverted;
        return string_plus;
    }

    fn set_blinking(self, is_blinking: ThreeBool) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.font_settings.is_blinking = is_blinking;
        return string_plus;
    }

    fn set_font_settings(self, font_settings: FontSettings) -> StringPlus {
        let mut string_plus: StringPlus = self.into();
        string_plus.font_settings = font_settings;
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
            font_settings: FontSettings::default(),
        };
    }
}

impl From<String> for StringPlus {
    fn from(value: String) -> Self {
        return StringPlus {
            string: value,
            font_settings: FontSettings::default(),
        };
    }
}

impl From<DecCharSet> for StringPlus {
    fn from(dec_line: DecCharSet) -> StringPlus {
        let mut font_settings: FontSettings = FontSettings::default();
        font_settings.is_dec_line = true;
        return StringPlus {
            string: dec_line.get_code(),
            font_settings: font_settings,
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
