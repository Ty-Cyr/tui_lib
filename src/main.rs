use std::env;
use tui_lib::{
    string_plus::{AsSp, DecCharSet, StringPlusTrait},
    tui_enums::ThreeBool,
    tui_enums::{Color, CursorMode, TuiMode},
    tui_events::TuiEvents,
    tui_terminal::TuiTerminal,
};

fn feature_sample() -> Result<(), String> {
    let mut tui_terminal: TuiTerminal = TuiTerminal::new(TuiMode::FullScreen)
        .map_err(|_| "Failed To Setup Terminal".to_string())?;
    tui_terminal.set_background_color(Color::Black);
    tui_terminal.clear_screen();
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

    tui_terminal.print(DecCharSet::TopLeft);
    tui_terminal.print(DecCharSet::TopMiddle);
    tui_terminal.print(DecCharSet::TopMiddle);
    tui_terminal.println(DecCharSet::TopRight);
    tui_terminal.print(DecCharSet::MiddleLeft);
    tui_terminal.print(DecCharSet::MiddleMiddle);
    tui_terminal.print(DecCharSet::MiddleMiddle);
    tui_terminal.println(DecCharSet::MiddleRight);
    tui_terminal.print(DecCharSet::BottomLeft);
    tui_terminal.print(DecCharSet::BottomMiddle);
    tui_terminal.print(DecCharSet::BottomMiddle);
    tui_terminal.println(DecCharSet::BottomRight);

    tui_terminal.println(
        "blinking\nlines"
            .set_blinking(ThreeBool::True)
            .set_background_color(Color::CC256(41)),
    );
    tui_terminal.println("Italics".set_italics(ThreeBool::True));
    tui_terminal.println(
        DecCharSet::Block
            .set_font_color(Color::Red)
            .set_blinking(ThreeBool::True),
    );
    tui_terminal.set_background_color(Color::RGB(50, 130, 0));
    tui_terminal.println("Hello World");
    tui_terminal.default_settings();
    tui_terminal.set_background_color(Color::Black);
    let (x, y) = tui_terminal
        .get_cursor_position()
        .ok()
        .ok_or("Failed To Get Cursor Position")?;

    tui_terminal
        .println(String::new() + "Position:  (" + &x.to_string() + ", " + &y.to_string() + ")");
    loop {
        let (a, b): (u16, u16) = tui_terminal
            .get_teminal_size()
            .ok()
            .ok_or("Failed To get Size")?;
        tui_terminal
            .println(String::new() + "Size: (" + &a.to_string() + ", " + &b.to_string() + ")");
        let event: TuiEvents = tui_terminal.get_event();
        match event {
            TuiEvents::Enter => {
                tui_terminal.println("NEWLINE".set_font_color(Color::RGB(0, 255, 0)))
            }
            TuiEvents::LeftArrow => {
                tui_terminal.println("Left Arrow".set_font_color(Color::BrightBlue))
            }
            TuiEvents::RightArrow => {
                tui_terminal.println("Right Arrow".set_font_color(Color::BrightBlue))
            }
            TuiEvents::UpArrow => {
                tui_terminal.println("Up Arrow".set_font_color(Color::BrightBlue))
            }
            TuiEvents::DownArrow => {
                tui_terminal.println("Down Arrow".set_font_color(Color::BrightBlue))
            }
            TuiEvents::Backspace => {
                tui_terminal.println("BACKSPACE".set_font_color(Color::RGB(255, 255, 0)))
            }
            TuiEvents::Delete => {
                tui_terminal.println("DELETE".set_font_color(Color::RGB(255, 255, 0)))
            }
            TuiEvents::Space => {
                tui_terminal.println("SPACE".set_font_color(Color::RGB(255, 0, 255)))
            }
            TuiEvents::Tab => tui_terminal.println("TAB".set_font_color(Color::RGB(255, 0, 255))),
            TuiEvents::Escape => {
                tui_terminal.println(
                    "ESCAPE"
                        .set_bold(ThreeBool::True)
                        .set_underlined(ThreeBool::True),
                );
            }

            TuiEvents::Control('C') => {
                tui_terminal.println(
                    "CTRL-C"
                        .set_bold(ThreeBool::True)
                        .set_underlined(ThreeBool::True),
                );
                tui_terminal.println("Exiting");
                return Ok(());
            }

            TuiEvents::AsciiReadable(c) => {
                tui_terminal.println(c.to_string());
            }

            TuiEvents::Control(c) => {
                tui_terminal.print(DecCharSet::TopLeft);
                tui_terminal.print(DecCharSet::HorizontalBar);
                tui_terminal.println(DecCharSet::TopRight);
                tui_terminal.print(DecCharSet::VerticalBar);
                tui_terminal.print(c);
                tui_terminal.println(DecCharSet::VerticalBar);
                tui_terminal.print(DecCharSet::BottomLeft);
                tui_terminal.print(DecCharSet::HorizontalBar);
                tui_terminal.println(DecCharSet::BottomRight);
            }

            TuiEvents::Other(c) => {
                tui_terminal.println("Key: ".to_string() + &c.to_string());
                tui_terminal.println("Key-U8: ".to_string() + &(c as u8).to_string());
            }

            TuiEvents::LeftClick((x, y)) => {
                tui_terminal.print("LEFTCLICK: (".set_background_color(Color::Blue));
                tui_terminal.print(x.as_sp().set_background_color(Color::Blue));
                tui_terminal.print(", ".set_background_color(Color::Blue));
                tui_terminal.print(y.as_sp().set_background_color(Color::Blue));
                tui_terminal.println(")".set_background_color(Color::Blue));
            }

            TuiEvents::MiddleClick((x, y)) => {
                tui_terminal.print("MIDDLECLICK: (".set_background_color(Color::Blue));
                tui_terminal.print(x.as_sp().set_background_color(Color::Blue));
                tui_terminal.print(", ".set_background_color(Color::Blue));
                tui_terminal.print(y.as_sp().set_background_color(Color::Blue));
                tui_terminal.println(")".set_background_color(Color::Blue));
            }

            TuiEvents::RightClick((x, y)) => {
                tui_terminal.print("RIGHTCLICK: (".set_background_color(Color::Blue));
                tui_terminal.print(x.as_sp().set_background_color(Color::Blue));
                tui_terminal.print(", ".set_background_color(Color::Blue));
                tui_terminal.print(y.as_sp().set_background_color(Color::Blue));
                tui_terminal.println(")".set_background_color(Color::Blue));
            }
            TuiEvents::MouseMove((x, y)) => {
                tui_terminal.print("MOUSE MOVE: (".set_background_color(Color::Blue));
                tui_terminal.print(x.as_sp().set_background_color(Color::Blue));
                tui_terminal.print(", ".set_background_color(Color::Blue));
                tui_terminal.print(y.as_sp().set_background_color(Color::Blue));
                tui_terminal.println(")".set_background_color(Color::Blue));
            }
            TuiEvents::LeftDrag((x, y)) => {
                tui_terminal.print("LEFT DRAG: (".set_background_color(Color::Blue));
                tui_terminal.print(x.as_sp().set_background_color(Color::Blue));
                tui_terminal.print(", ".set_background_color(Color::Blue));
                tui_terminal.print(y.as_sp().set_background_color(Color::Blue));
                tui_terminal.println(")".set_background_color(Color::Blue));
            }
            TuiEvents::MiddleDrag((x, y)) => {
                tui_terminal.print("MIDDLE DRAG: (".set_background_color(Color::Blue));
                tui_terminal.print(x.as_sp().set_background_color(Color::Blue));
                tui_terminal.print(", ".set_background_color(Color::Blue));
                tui_terminal.print(y.as_sp().set_background_color(Color::Blue));
                tui_terminal.println(")".set_background_color(Color::Blue));
            }
            TuiEvents::RightDrag((x, y)) => {
                tui_terminal.print("RIGHT DRAG: (".set_background_color(Color::Blue));
                tui_terminal.print(x.as_sp().set_background_color(Color::Blue));
                tui_terminal.print(", ".set_background_color(Color::Blue));
                tui_terminal.print(y.as_sp().set_background_color(Color::Blue));
                tui_terminal.println(")".set_background_color(Color::Blue));
            }
            TuiEvents::ScrollUp((x, y)) => {
                tui_terminal.print("SROLL UP: (".set_background_color(Color::Blue));
                tui_terminal.print(x.as_sp().set_background_color(Color::Blue));
                tui_terminal.print(", ".set_background_color(Color::Blue));
                tui_terminal.print(y.as_sp().set_background_color(Color::Blue));
                tui_terminal.println(")".set_background_color(Color::Blue));
            }

            TuiEvents::ScrollDown((x, y)) => {
                tui_terminal.print("SROLL Down: (".set_background_color(Color::Blue));
                tui_terminal.print(x.as_sp().set_background_color(Color::Blue));
                tui_terminal.print(", ".set_background_color(Color::Blue));
                tui_terminal.print(y.as_sp().set_background_color(Color::Blue));
                tui_terminal.println(")".set_background_color(Color::Blue));
            }
            TuiEvents::Ignore => {}
            TuiEvents::Error => {
                tui_terminal.println(
                    "Failed To Get Event"
                        .set_bold(ThreeBool::True)
                        .set_font_color(Color::Red),
                );
            }
        }
    }
}

fn draw_test() -> Result<(), String> {
    let mut tui_terminal =
        TuiTerminal::new(TuiMode::FullScreen).map_err(|_| "Failed To Get Tui Terminal")?;
    tui_terminal.set_cursor_position(1, 1);
    let (width, height) = tui_terminal
        .get_teminal_size()
        .ok()
        .ok_or("Failed To Get Terminal Size")?;
    tui_terminal.set_cursor(CursorMode::Hidden);
    tui_terminal.set_cursor_position(width / 2, height / 2);
    tui_terminal.println(
        DecCharSet::MiddleMiddle
            .set_blinking(ThreeBool::True)
            .set_font_color(Color::Magenta),
    );
    tui_terminal.print(DecCharSet::LessEqual);
    tui_terminal.print("=");
    tui_terminal.print(DecCharSet::NotEqual);
    tui_terminal.print(DecCharSet::GreaterEqual);
    loop {
        let event = tui_terminal.get_event();
        match event {
            TuiEvents::Control('C') | TuiEvents::Control('c') | TuiEvents::Escape => break,
            _ => continue,
        }
    }
    return Ok(());
}

#[allow(unreachable_code)]
fn main() -> Result<(), String> {
    let mut args = env::args();
    if args.len() != 2 {
        return Err("Usage: <1 | 2>".into());
    }
    args.next();
    return match args.next().ok_or("Usage: <1 | 2>")?.as_str() {
        "1" => draw_test(),
        "2" => feature_sample(),
        _ => Err("Usage: <1 | 2>".into()),
    };
}
