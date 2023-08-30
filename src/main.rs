use tui_lib::{
    string_plus::StringPlus,
    tui_enums::ThreeBool,
    tui_enums::{Color, CursorMode, TuiMode},
    tui_keys::TuiKeys,
    tui_terminal::TuiTerminal,
};

fn main() -> Result<(), String> {
    let mut tui_terminal: TuiTerminal =
        TuiTerminal::new(TuiMode::FullScreen).ok_or("Failed To Setup Terminal".to_string())?;
    tui_terminal.set_cursor(CursorMode::Default);
    tui_terminal.println(StringPlus::new("White").set_font_color(Color::White));
    tui_terminal.println(StringPlus::new("White").set_background_color(Color::White));
    tui_terminal.println(StringPlus::new("Bright White").set_font_color(Color::BrightWhite));
    tui_terminal.println(StringPlus::new("Bright White").set_background_color(Color::BrightWhite));

    tui_terminal.println(StringPlus::new("Red").set_font_color(Color::Red));
    tui_terminal.println(StringPlus::new("Red").set_background_color(Color::Red));
    tui_terminal.println(StringPlus::new("Bright Red").set_font_color(Color::BrightRed));
    tui_terminal.println(StringPlus::new("Bright Red").set_background_color(Color::BrightRed));

    tui_terminal.println(StringPlus::new("Green").set_font_color(Color::Green));
    tui_terminal.println(StringPlus::new("Green").set_background_color(Color::Green));
    tui_terminal.println(StringPlus::new("Bright Green").set_font_color(Color::BrightGreen));
    tui_terminal.println(StringPlus::new("Bright Green").set_background_color(Color::BrightGreen));

    tui_terminal.println(StringPlus::new("Blue").set_font_color(Color::Blue));
    tui_terminal.println(StringPlus::new("Blue").set_background_color(Color::Blue));
    tui_terminal.println(StringPlus::new("Bright Blue").set_font_color(Color::BrightBlue));
    tui_terminal.println(StringPlus::new("Bright Blue").set_background_color(Color::BrightBlue));

    tui_terminal.println(StringPlus::new("Yellow").set_font_color(Color::Yellow));
    tui_terminal.println(StringPlus::new("Yellow").set_background_color(Color::Yellow));
    tui_terminal.println(StringPlus::new("Bright Yellow").set_font_color(Color::BrightYellow));
    tui_terminal
        .println(StringPlus::new("Bright Yellow").set_background_color(Color::BrightYellow));

    tui_terminal.println(StringPlus::new("Cyan").set_font_color(Color::Cyan));
    tui_terminal.println(StringPlus::new("Cyan").set_background_color(Color::Cyan));
    tui_terminal.println(StringPlus::new("Bright Cyan").set_font_color(Color::BrightCyan));
    tui_terminal.println(StringPlus::new("Bright Cyan").set_background_color(Color::BrightCyan));

    tui_terminal.println(StringPlus::new("Magenta").set_font_color(Color::Magenta));
    tui_terminal.println(StringPlus::new("Magenta").set_background_color(Color::Magenta));
    tui_terminal.println(StringPlus::new("Bright Magenta").set_font_color(Color::BrightMagenta));
    tui_terminal
        .println(StringPlus::new("Bright Magenta").set_background_color(Color::BrightMagenta));

    tui_terminal.println(
        StringPlus::new("Black")
            .set_background_color(Color::White)
            .set_font_color(Color::Black),
    );
    tui_terminal.println(StringPlus::new("Black").set_background_color(Color::Black));
    tui_terminal.println(
        StringPlus::new("Bright Black")
            .set_background_color(Color::White)
            .set_font_color(Color::BrightBlack),
    );
    tui_terminal.println(StringPlus::new("Bright Black").set_background_color(Color::BrightBlack));

    loop {
        let (a, b): (u16, u16) = tui_terminal
            .get_teminal_size()
            .ok_or("Failed To get Size")?;
        tui_terminal.println(format!("{}, {}", a, b));
        let event: TuiKeys = tui_terminal.get_keyboard_event();
        match event {
            TuiKeys::Enter => tui_terminal
                .println(StringPlus::new("NEWLINE").set_font_color(Color::RGB(0, 255, 0))),
            TuiKeys::LeftArrow => tui_terminal
                .println(StringPlus::new("Left Arrow").set_font_color(Color::RGB(0, 0, 255))),
            TuiKeys::RightArrow => tui_terminal
                .println(StringPlus::new("Right Arrow").set_font_color(Color::RGB(0, 0, 255))),
            TuiKeys::UpArrow => tui_terminal
                .println(StringPlus::new("Up Arrow").set_font_color(Color::RGB(0, 0, 255))),
            TuiKeys::DownArrow => tui_terminal
                .println(StringPlus::new("Down Arrow").set_font_color(Color::RGB(0, 0, 255))),
            TuiKeys::Backspace => tui_terminal
                .println(StringPlus::new("BACKSPACE").set_font_color(Color::RGB(255, 255, 0))),
            TuiKeys::Delete => tui_terminal
                .println(StringPlus::new("DELETE").set_font_color(Color::RGB(255, 255, 0))),
            TuiKeys::Space => tui_terminal
                .println(StringPlus::new("SPACE").set_font_color(Color::RGB(255, 0, 255))),
            TuiKeys::Tab => {
                tui_terminal.println(StringPlus::new("TAB").set_font_color(Color::RGB(255, 0, 255)))
            }
            TuiKeys::Escape => {
                tui_terminal.println(
                    StringPlus::new("ESCAPE")
                        .set_bold(ThreeBool::True)
                        .set_underlined(ThreeBool::True),
                );
                tui_terminal.println("Exiting");
                return Ok(());
            }

            TuiKeys::AsciiReadable(c) => {
                tui_terminal.println(c.to_string());
            }

            TuiKeys::Other(c) => {
                tui_terminal.println("Key: ".to_string() + &c.to_string());
                tui_terminal.println("Key-U8: ".to_string() + &(c as u8).to_string());
            }
            _ => {
                tui_terminal.println(
                    StringPlus::new("Failed To Get Keyboard Event")
                        .set_bold(ThreeBool::True)
                        .set_font_color(Color::Red),
                );
            }
        }
    }
}
