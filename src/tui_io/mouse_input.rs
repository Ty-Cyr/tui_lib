use crate::tui_events::TuiEvents;

use super::input_interface::InputInterfaceT;

pub trait MouseInput: InputInterfaceT {
    fn handle_mouse_events(&self) -> TuiEvents {
        return match self.read_raw_immediate() {
            Some('0') => match self.get_coordinates() {
                Some((x, y, 'M')) => TuiEvents::LeftClick(x, y),
                Some((_, _, 'm')) => TuiEvents::Ignore,
                None | Some(_) => TuiEvents::Error,
            },
            Some('1') => match self.get_coordinates() {
                Some((x, y, 'M')) => TuiEvents::MiddleClick(x, y),
                Some((_, _, 'm')) => TuiEvents::Ignore,
                None | Some(_) => TuiEvents::Error,
            },
            Some('2') => match self.get_coordinates() {
                Some((x, y, 'M')) => TuiEvents::RightClick(x, y),
                Some((_, _, 'm')) => TuiEvents::Ignore,
                None | Some(_) => TuiEvents::Error,
            },
            Some('3') => self.handle_mouse_move_event(),
            Some('6') => self.handle_scroll_event(),
            None | Some(_) => return TuiEvents::Error,
        };
    }

    fn handle_mouse_move_event(&self) -> TuiEvents {
        return match self.read_raw_immediate() {
            Some('2') => match self.get_coordinates() {
                Some((x, y, 'M')) => TuiEvents::LeftDrag(x, y),
                Some(_) | None => TuiEvents::Error,
            },
            Some('3') => match self.get_coordinates() {
                Some((x, y, 'M')) => TuiEvents::MiddleDrag(x, y),
                Some(_) | None => TuiEvents::Error,
            },
            Some('4') => match self.get_coordinates() {
                Some((x, y, 'M')) => TuiEvents::RightDrag(x, y),
                Some(_) | None => TuiEvents::Error,
            },
            Some('5') => match self.get_coordinates() {
                Some((x, y, 'm')) => TuiEvents::MouseMove(x, y),
                Some(_) | None => TuiEvents::Error,
            },
            None | Some(_) => TuiEvents::Error,
        };
    }

    fn handle_scroll_event(&self) -> TuiEvents {
        return match self.read_raw_immediate() {
            Some('4') => match self.get_coordinates() {
                Some((x, y, 'M')) => TuiEvents::ScrollUp(x, y),
                None | Some(_) => TuiEvents::Error,
            },
            Some('5') => match self.get_coordinates() {
                Some((x, y, 'M')) => TuiEvents::ScrollDown(x, y),
                None | Some(_) => TuiEvents::Error,
            },
            None | Some(_) => TuiEvents::Error,
        };
    }

    fn get_u16(&self) -> Option<(u16, char)> {
        let mut num = 0;
        loop {
            let next = self.read_raw_immediate()?;
            match next as u16 {
                0x30..=0x39 => {
                    let digit = next as u16 - 0x30;
                    if num > u16::MAX {
                        return None;
                    }
                    num *= 10;
                    if digit > u16::MAX - num {
                        return None;
                    }
                    num += digit;
                }
                _ => return Some((num, next)),
            }
        }
    }

    fn get_coordinates(&self) -> Option<(u16, u16, char)> {
        let Some(';') = self.read_raw_immediate() else {
            return None;
        };
        let Some((x, ';')) = self.get_u16() else {
            return None;
        };
        return match self.get_u16() {
            Some((y, c)) => Some((x, y, c)),
            None => None,
        };
    }
}