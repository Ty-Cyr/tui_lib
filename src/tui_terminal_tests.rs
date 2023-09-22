use crate::tui_terminal::TuiTerminal;

fn get_center(tui_terminal: &mut TuiTerminal) -> (u16, u16) {
    let (x, y) = tui_terminal.get_teminal_size().expect("Terminal Size");
    return (x / 2, y / 2);
}

#[allow(unused)]
#[test]
fn test_set_cursor_position() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .ok_or("Unable To Create TuiTerminal")?;
    let (x, y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();

    if position != Ok((x, y)) {
        return Err(format!("{:?} != Ok({:?})", position, (x, y)));
    }
    return Ok(());
}

#[allow(unused)]
#[test]
fn test_restore_cursor_position() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .ok_or("Unable To Create TuiTerminal")?;
    let (x, y) = get_center(&mut tui_terminal);
    let position1 = tui_terminal.get_cursor_position();
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.restore_cursor_position();
    let position2 = tui_terminal.get_cursor_position();

    if position1 != position2 {
        return Err(format!("{:?} != {:?}", position1, position2));
    }

    drop(tui_terminal);
    return Ok(());
}

#[allow(unused)]
#[test]
fn test_shift_cursor_next() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .ok_or("Unable To Create TuiTerminal")?;
    let (mut x, mut y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Next(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x = 1;
    y += 1;
    if position != Ok((x, y)) {
        return Err(format!("{:?} != Ok({:?})", position, (x, y)));
    }

    drop(tui_terminal);
    return Ok(());
}

#[allow(unused)]
#[test]
fn test_shift_cursor_previous() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .ok_or("Unable To Create TuiTerminal")?;
    let (mut x, mut y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Previous(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x = 1;
    y -= 1;

    if position != Ok((x, y)) {
        return Err(format!("{:?} != Ok({:?})", position, (x, y)));
    }

    drop(tui_terminal);
    return Ok(());
}

#[allow(unused)]
#[test]
fn test_shift_cursor_forwards() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .ok_or("Unable To Create TuiTerminal")?;
    let (mut x, y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Forwards(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x += 1;

    if position != Ok((x, y)) {
        return Err(format!("{:?} != Ok({:?})", position, (x, y)));
    }

    drop(tui_terminal);
    return Ok(());
}

#[allow(unused)]
#[test]
fn test_shift_cursor_backwards() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .ok_or("Unable To Create TuiTerminal")?;
    let (mut x, y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Backwards(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x -= 1;

    if position != Ok((x, y)) {
        return Err(format!("{:?} != Ok({:?})", position, (x, y)));
    }

    drop(tui_terminal);
    return Ok(());
}

#[allow(unused)]
#[test]
fn test_shift_cursor_up() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .ok_or("Unable To Create TuiTerminal")?;
    let (x, mut y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Up(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    y -= 1;

    if position != Ok((x, y)) {
        return Err(format!("{:?} != Ok({:?})", position, (x, y)));
    }

    drop(tui_terminal);
    return Ok(());
}

#[allow(unused)]
#[test]
fn test_shift_cursor_down() -> Result<(), String> {
    let mut tui_terminal = TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen)
        .ok_or("Unable to Create Tui Terminal")?;
    let (x, mut y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Down(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    y += 1;

    if position != Ok((x, y)) {
        return Err(format!("{:?} != Ok({:?})", position, (x, y)));
    }

    drop(tui_terminal);
    return Ok(());
}
