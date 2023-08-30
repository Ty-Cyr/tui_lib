use crate::{Color, ThreeBool};

#[derive(Clone)]
pub struct StringPlus {
    string: String,
    font_color: Color,
    background_color: Color,
    is_bold: ThreeBool,
    is_underlined: ThreeBool,
    is_inverted: ThreeBool,
}

impl StringPlus {
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
}

pub trait StringPlusTrait {
    fn set_font_color(self, color: Color) -> StringPlus;
    fn set_background_color(self, color: Color) -> StringPlus;
    fn set_bold(self, is_bold: ThreeBool) -> StringPlus;
    fn set_underlined(self, is_underlined: ThreeBool) -> StringPlus;
    fn set_inverted(self, is_inverted: ThreeBool) -> StringPlus;
}

impl<T: Into<StringPlus>> StringPlusTrait for T {
    fn set_font_color(self, color: Color) -> StringPlus {
        let mut string_plus = self.into();
        string_plus.font_color = color;
        return string_plus;
    }
    fn set_background_color(self, color: Color) -> StringPlus {
        let mut string_plus = self.into();
        string_plus.background_color = color;
        return string_plus;
    }

    fn set_bold(self, is_bold: ThreeBool) -> StringPlus {
        let mut string_plus = self.into();
        string_plus.is_underlined = is_bold;
        return string_plus;
    }

    fn set_underlined(self, is_underlined: ThreeBool) -> StringPlus {
        let mut string_plus = self.into();
        string_plus.is_underlined = is_underlined;
        return string_plus;
    }

    fn set_inverted(self, is_inverted: ThreeBool) -> StringPlus {
        let mut string_plus = self.into();
        string_plus.is_inverted = is_inverted;
        return string_plus;
    }
}

impl Into<String> for StringPlus {
    fn into(self) -> String {
        return self.string;
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
        };
    }
}

impl Into<StringPlus> for String {
    fn into(self) -> StringPlus {
        return StringPlus {
            string: self,
            font_color: Color::Default,
            background_color: Color::Default,
            is_bold: ThreeBool::Default,
            is_underlined: ThreeBool::Default,
            is_inverted: ThreeBool::Default,
        };
    }
}

pub trait AsSp {
    fn as_sp(self) -> StringPlus;
}

impl<T: Into<StringPlus>> AsSp for T {
    fn as_sp(self) -> StringPlus {
        return self.into();
    }
}
