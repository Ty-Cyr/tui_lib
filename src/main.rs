use tui_lib::{
    string_plus::{DecLine, StringPlusTrait},
    tui_enums::ThreeBool,
    tui_enums::{Color, CursorMode, TuiMode},
    tui_keys::TuiKeys,
    tui_terminal::TuiTerminal,
};

fn main() -> Result<(), String> {
    let mut tui_terminal: TuiTerminal =
        TuiTerminal::new(TuiMode::FullScreen).ok_or("Failed To Setup Terminal".to_string())?;
    tui_terminal.set_background_color(Color::Black);
    tui_terminal.set_cursor(CursorMode::Hidden);
    tui_terminal.println("White".set_font_color(Color::White));
    tui_terminal.println("White".set_background_color(Color::White));
    tui_terminal.println("Bright White".set_font_color(Color::BrightWhite));
    tui_terminal.println("Bright White".set_background_color(Color::BrightWhite));

    tui_terminal.println("Red".set_font_color(Color::Red));
    tui_terminal.println("Red".set_background_color(Color::Red));
    tui_terminal.println("Bright Red".set_font_color(Color::BrightRed));
    tui_terminal.println("Bright Red".set_background_color(Color::BrightRed));

    tui_terminal.println("Green".set_font_color(Color::Green));
    tui_terminal.println("Green".set_background_color(Color::Green));
    tui_terminal.println("Bright Green".set_font_color(Color::BrightGreen));
    tui_terminal.println("Bright Green".set_background_color(Color::BrightGreen));

    tui_terminal.println("Blue".set_font_color(Color::Blue));
    tui_terminal.println("Blue".set_background_color(Color::Blue));
    tui_terminal.println("Bright Blue".set_font_color(Color::BrightBlue));
    tui_terminal.println("Bright Blue".set_background_color(Color::BrightBlue));

    tui_terminal.println("Yellow".set_font_color(Color::Yellow));
    tui_terminal.println("Yellow".set_background_color(Color::Yellow));
    tui_terminal.println("Bright Yellow".set_font_color(Color::BrightYellow));
    tui_terminal.println("Bright Yellow".set_background_color(Color::BrightYellow));

    tui_terminal.println("Cyan".set_font_color(Color::Cyan));
    tui_terminal.println("Cyan".set_background_color(Color::Cyan));
    tui_terminal.println("Bright Cyan".set_font_color(Color::BrightCyan));
    tui_terminal.println("Bright Cyan".set_background_color(Color::BrightCyan));

    tui_terminal.println("Magenta".set_font_color(Color::Magenta));
    tui_terminal.println("Magenta".set_background_color(Color::Magenta));
    tui_terminal.println("Bright Magenta".set_font_color(Color::BrightMagenta));
    tui_terminal.println("Bright Magenta".set_background_color(Color::BrightMagenta));

    tui_terminal.println(
        "Black"
            .set_background_color(Color::White)
            .set_font_color(Color::Black),
    );
    tui_terminal.println("Black".set_background_color(Color::Black));
    tui_terminal.println(
        "Bright Black"
            .set_background_color(Color::White)
            .set_font_color(Color::BrightBlack),
    );
    tui_terminal.println("Bright Black".set_background_color(Color::BrightBlack));

    tui_terminal.print(DecLine::TopLeft);
    tui_terminal.print(DecLine::TopMiddle);
    tui_terminal.print(DecLine::TopMiddle);
    tui_terminal.println(DecLine::TopRight);
    tui_terminal.print(DecLine::MiddleLeft);
    tui_terminal.print(DecLine::MiddleMiddle);
    tui_terminal.print(DecLine::MiddleMiddle);
    tui_terminal.println(DecLine::MiddleRight);
    tui_terminal.print(DecLine::BottomLeft);
    tui_terminal.print(DecLine::BottomMiddle);
    tui_terminal.print(DecLine::BottomMiddle);
    tui_terminal.println(DecLine::BottomRight);

    tui_terminal.println(
        "blinking\nlines"
            .set_blinking(ThreeBool::True)
            .set_background_color(Color::CC256(41)),
    );
    tui_terminal.println("Italics".set_italics(ThreeBool::True));
    tui_terminal.println(
        DecLine::Block
            .set_font_color(Color::Red)
            .set_blinking(ThreeBool::True),
    );
    tui_terminal.set_background_color(Color::RGB(50, 130, 0));
    tui_terminal.println("Hello World");
    tui_terminal.default_settings();

    loop {
        let (a, b): (u16, u16) = tui_terminal
            .get_teminal_size()
            .ok_or("Failed To get Size")?;
        tui_terminal.println(format!("({}, {})", a, b));
        let event: TuiKeys = tui_terminal.get_keyboard_event();
        match event {
            TuiKeys::Enter => tui_terminal.println("NEWLINE".set_font_color(Color::RGB(0, 255, 0))),
            TuiKeys::LeftArrow => {
                tui_terminal.println("Left Arrow".set_font_color(Color::RGB(0, 0, 255)))
            }
            TuiKeys::RightArrow => {
                tui_terminal.println("Right Arrow".set_font_color(Color::RGB(0, 0, 255)))
            }
            TuiKeys::UpArrow => {
                tui_terminal.println("Up Arrow".set_font_color(Color::RGB(0, 0, 255)))
            }
            TuiKeys::DownArrow => {
                tui_terminal.println("Down Arrow".set_font_color(Color::RGB(0, 0, 255)))
            }
            TuiKeys::Backspace => {
                tui_terminal.println("BACKSPACE".set_font_color(Color::RGB(255, 255, 0)))
            }
            TuiKeys::Delete => {
                tui_terminal.println("DELETE".set_font_color(Color::RGB(255, 255, 0)))
            }
            TuiKeys::Space => tui_terminal.println("SPACE".set_font_color(Color::RGB(255, 0, 255))),
            TuiKeys::Tab => tui_terminal.println("TAB".set_font_color(Color::RGB(255, 0, 255))),
            TuiKeys::Escape => {
                tui_terminal.println(
                    "ESCAPE"
                        .set_bold(ThreeBool::True)
                        .set_underlined(ThreeBool::True),
                );
                tui_terminal.println("Exiting");
                return Ok(());
            }

            TuiKeys::AsciiReadable(c) => {
                tui_terminal.println(c.to_string());
            }

            TuiKeys::Control(c) => {
                tui_terminal.print(DecLine::TopLeft);
                tui_terminal.print(DecLine::HorizontalBar);
                tui_terminal.println(DecLine::TopRight);
                tui_terminal.print(DecLine::VerticalBar);
                tui_terminal.print(c);
                tui_terminal.println(DecLine::VerticalBar);
                tui_terminal.print(DecLine::BottomLeft);
                tui_terminal.print(DecLine::HorizontalBar);
                tui_terminal.println(DecLine::BottomRight);
            }

            TuiKeys::Other(c) => {
                tui_terminal.println("Key: ".to_string() + &c.to_string());
                tui_terminal.println("Key-U8: ".to_string() + &(c as u8).to_string());
            }
            _ => {
                tui_terminal.println(
                    "Failed To Get Keyboard Event"
                        .set_bold(ThreeBool::True)
                        .set_font_color(Color::Red),
                );
            }
        }
    }
}
