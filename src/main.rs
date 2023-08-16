use tui_lib::{
    string_plus::StringPlus,
    tui_enums::ThreeBool,
    tui_enums::{Color, TuiMode},
    tui_events::TuiEvent,
    tui_keys::TuiKeys,
    tui_terminal::TuiTerminal,
};

fn main() -> Result<(), String> {
    let mut tui_terminal: TuiTerminal;
    match TuiTerminal::new(TuiMode::Standard) {
        Some(tui_terminal_instance) => tui_terminal = tui_terminal_instance,
        None => {
            return Err("Failed To Setup Terminal".to_string());
        }
    }
    tui_terminal.clear_screen();
    loop {
        let event: TuiEvent = tui_terminal.get_event();
        match event {
            TuiEvent::KeyEvent(key) => match key {
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
                TuiKeys::Tab => tui_terminal
                    .println(StringPlus::new("TAB").set_font_color(Color::RGB(255, 0, 255))),
                TuiKeys::Escape => {
                    tui_terminal.println(
                        StringPlus::new("ESCAPE")
                            .set_bold(ThreeBool::True)
                            .set_underlined(ThreeBool::True),
                    );
                    tui_terminal.println("Exiting");
                    return Ok(());
                }

                TuiKeys::Other(c) => {
                    tui_terminal.println("Key: ".to_string() + &c.to_string());
                    tui_terminal.println("Key: ".to_string() + &(c as u8).to_string());
                }
            },
            TuiEvent::BufferSizeEvent => tui_terminal.println("Buffer Size Event"),
            _ => {
                tui_terminal.println("Unknown Event");
            }
        }
    }
}
