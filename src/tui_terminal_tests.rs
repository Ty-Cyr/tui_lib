use std::io::stdin;

use crate::tui_terminal::TuiTerminal;

fn get_center(tui_terminal: &mut TuiTerminal) -> (u16, u16) {
    let (x, y) = tui_terminal.get_teminal_size().expect("Terminal Size");
    return (x / 2, y / 2);
}

#[test]
fn test_set_cursor_position() {
    let mut tui_terminal =
        TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen).expect("Terminal");
    let (x, y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    drop(tui_terminal);
    assert_eq!(position, Ok((x, y)));
}

#[test]
fn test_restore_cursor_position() {
    let mut tui_terminal =
        TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen).expect("Terminal");
    let (x, y) = get_center(&mut tui_terminal);
    let position1 = tui_terminal.get_cursor_position();
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.restore_cursor_position();
    let position2 = tui_terminal.get_cursor_position();
    drop(tui_terminal);
    assert_eq!(position1, position2);
}

#[test]
fn test_shift_cursor_next() {
    let mut tui_terminal =
        TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen).expect("Terminal");
    let (mut x, mut y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Next(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x = 1;
    y += 1;
    drop(tui_terminal);
    assert_eq!(position, Ok((x, y)));
}

#[test]
fn test_shift_cursor_previous() {
    let mut tui_terminal =
        TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen).expect("Terminal");
    let (mut x, mut y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Previous(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x = 1;
    y -= 1;
    drop(tui_terminal);
    assert_eq!(position, Ok((x, y)));
}

#[test]
fn test_shift_cursor_forwards() {
    let mut tui_terminal =
        TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen).expect("Terminal");
    let (mut x, y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Forwards(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x += 1;
    drop(tui_terminal);
    assert_eq!(position, Ok((x, y)));
}

#[test]
fn test_shift_cursor_backwards() {
    let mut tui_terminal =
        TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen).expect("Terminal");
    let (mut x, y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Backwards(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    x -= 1;
    drop(tui_terminal);
    assert_eq!(position, Ok((x, y)));
}

#[test]
fn test_shift_cursor_up() {
    let mut tui_terminal =
        TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen).expect("Terminal");
    let (x, mut y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Up(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    y -= 1;
    drop(tui_terminal);
    assert_eq!(position, Ok((x, y)));
}

#[test]
fn test_shift_cursor_down() {
    let lock = stdin().lock();
    let mut tui_terminal =
        TuiTerminal::new(crate::tui_enums::TuiMode::FullScreen).expect("Terminal");
    let (x, mut y) = get_center(&mut tui_terminal);
    tui_terminal.save_cursor_position();
    tui_terminal.set_cursor_position(x, y);
    tui_terminal.shift_cursor(crate::tui_enums::CursorNav::Down(1));
    let position = tui_terminal.get_cursor_position();
    tui_terminal.restore_cursor_position();
    y += 1;

    drop(tui_terminal);
    assert_eq!(position, Ok((x, y)));
    _ = lock;
}
