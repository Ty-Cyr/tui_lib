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
    pub fn new(string: &str) -> StringPlus {
        return StringPlus {
            string: string.to_string(),
            font_color: Color::Default,
            background_color: Color::Default,
            is_bold: ThreeBool::Default,
            is_underlined: ThreeBool::Default,
            is_inverted: ThreeBool::Default,
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
    pub fn set_background_color(mut self, color: Color) -> StringPlus {
        self.background_color = color;
        return self;
    }

    pub fn get_bold(&self) -> ThreeBool {
        return self.is_bold;
    }

    pub fn set_bold(mut self, is_bold: ThreeBool) -> StringPlus {
        self.is_bold = is_bold;
        return self;
    }

    pub fn get_underlined(&self) -> ThreeBool {
        return self.is_underlined;
    }

    pub fn set_underlined(mut self, is_underlined: ThreeBool) -> StringPlus {
        self.is_underlined = is_underlined;
        return self;
    }

    pub fn get_inverted(&self) -> ThreeBool {
        return self.is_inverted;
    }
    pub fn set_inverted(mut self, is_inverted: ThreeBool) -> StringPlus {
        self.is_inverted = is_inverted;
        return self;
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
