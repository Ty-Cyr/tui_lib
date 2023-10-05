use crate::{
    tui_enums::{Color, ThreeBool},
    tui_terminal::TuiTerminal,
};

fn get_center(tui_terminal: &mut TuiTerminal) -> Result<(u16, u16), String> {
    let (x, y) = tui_terminal
        .get_teminal_size()
        .map_err(|error| error.to_string())?;
    return Ok((x / 2, y / 2));
}

#[test]
fn test_set_cursor_position() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .map_err(|_| "Unable To Create TuiTerminal")?;
    let (x, y) = get_center(&mut tui_terminal)?;
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    return match position {
        Ok(coordinates) if coordinates == (x, y) => Ok(()),
        Ok(coordinates) => Err(format!("{:?} != Ok({:?})", coordinates, position)),
        Err(error) => Err(error.to_string()),
    };
}

#[test]
fn test_restore_cursor_position() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .map_err(|_| "Unable To Create TuiTerminal")?;
    let (x, y) = get_center(&mut tui_terminal)?;
    let position1 = tui_terminal
        .get_cursor_position()
        .map_err(|error| error.to_string())?;

    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.restore_cursor_position();
    let position2 = tui_terminal
        .get_cursor_position()
        .map_err(|error| error.to_string())?;
    return if position1 == position2 {
        Ok(())
    } else {
        Err(format!("{:?} != {:?}", position1, position2))?
    };
}

#[test]
fn test_shift_cursor_next() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .map_err(|_| "Unable To Create TuiTerminal")?;
    let (mut x, mut y) = get_center(&mut tui_terminal)?;
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Next(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x = 1;
    y += 1;

    return match position {
        Ok(coordinates) if coordinates == (x, y) => Ok(()),
        Ok(coordinates) => Err(format!("{:?} != Ok({:?})", coordinates, position)),
        Err(error) => Err(error.to_string()),
    };
}

#[test]
fn test_shift_cursor_previous() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .map_err(|_| "Unable To Create TuiTerminal")?;
    let (mut x, mut y) = get_center(&mut tui_terminal)?;
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Previous(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x = 1;
    y -= 1;

    return match position {
        Ok(coordinates) if coordinates == (x, y) => Ok(()),
        Ok(coordinates) => Err(format!("{:?} != Ok({:?})", coordinates, position)),
        Err(error) => Err(error.to_string()),
    };
}

#[test]
fn test_shift_cursor_forwards() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .map_err(|_| "Unable To Create TuiTerminal")?;
    let (mut x, y) = get_center(&mut tui_terminal)?;
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Forwards(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x += 1;

    return match position {
        Ok(coordinates) if coordinates == (x, y) => Ok(()),
        Ok(coordinates) => Err(format!("{:?} != Ok({:?})", coordinates, position)),
        Err(error) => Err(error.to_string()),
    };
}

#[test]
fn test_shift_cursor_backwards() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .map_err(|_| "Unable To Create TuiTerminal")?;
    let (mut x, y) = get_center(&mut tui_terminal)?;
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Backwards(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x -= 1;

    return match position {
        Ok(coordinates) if coordinates == (x, y) => Ok(()),
        Ok(coordinates) => Err(format!("{:?} != Ok({:?})", coordinates, position)),
        Err(error) => Err(error.to_string()),
    };
}

#[test]
fn test_shift_cursor_up() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .map_err(|_| "Unable To Create TuiTerminal")?;
    let (x, mut y) = get_center(&mut tui_terminal)?;
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Up(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    y -= 1;

    return match position {
        Ok(coordinates) if coordinates == (x, y) => Ok(()),
        Ok(coordinates) => Err(format!("{:?} != Ok({:?})", coordinates, position)),
        Err(error) => Err(error.to_string()),
    };
}

#[test]
fn test_shift_cursor_down() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .map_err(|_| "Unable to Create Tui Terminal")?;
    let (x, mut y) = get_center(&mut tui_terminal)?;
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Down(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    y += 1;

    return match position {
        Ok(coordinates) if coordinates == (x, y) => Ok(()),
        Ok(coordinates) => Err(format!("{:?} != Ok({:?})", coordinates, position)),
        Err(error) => Err(error.to_string()),
    };
}

#[test]
fn test_font_settings_restore() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .map_err(|_| "Unable to Create Tui Terminal")?;
    let font_settings1 = tui_terminal.get_font_settings();
    tui_terminal.set_font_color(Color::RGB(255, 41, 144));
    tui_terminal.set_background_color(Color::BrightMagenta);
    tui_terminal.set_inverted(ThreeBool::True);
    tui_terminal.set_font_settings(font_settings1.clone());
    let font_settings2 = tui_terminal.get_font_settings();
    if font_settings2 != tui_terminal.get_font_settings() {
        Err(format!("{:?} != {:?}", font_settings1, font_settings2))?;
    }
    Ok(())
}
