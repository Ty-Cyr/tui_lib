use crate::tui_events::TuiEvents;

use super::input_interface::InputInterfaceT;

pub trait ParseInput: InputInterfaceT {
    fn parse_input(&self, input_char: char) -> TuiEvents {
        match input_char {
            '\x1b' => {
                return self.handle_escape_input_s1();
            }
            '\x7F' => {
                return TuiEvents::Backspace;
            }
            '\n' | '\r' => {
                return TuiEvents::Enter;
            }
            ' ' => {
                return TuiEvents::Space;
            }
            '\t' => {
                return TuiEvents::Tab;
            }
            _ => match input_char as u32 {
                0x20..=0x7D => return TuiEvents::AsciiReadable(input_char),
                0 => TuiEvents::Ignore,
                1..=26 => return TuiEvents::Control((input_char as u8 + 0x40) as char),
                _ => return TuiEvents::Other(input_char),
            },
        }
    }
    fn handle_escape_input_s1(&self) -> TuiEvents {
        let input_char_option: Option<char> = self.read_raw_immediate();
        let result = match input_char_option {
            None => TuiEvents::Escape,
            Some('[') => self.handle_escape_input_s2(),
            Some(_) => TuiEvents::Error,
        };
        match result {
            TuiEvents::Error | TuiEvents::Ignore => loop {
                if let None = self.read_raw_immediate() {
                    return result;
                };
            },
            _ => return result,
        }
    }

    fn handle_escape_input_s2(&self) -> TuiEvents {
        let input_char = self.read_raw_immediate();
        return match input_char {
            Some('A') => TuiEvents::UpArrow,
            Some('B') => TuiEvents::DownArrow,
            Some('C') => TuiEvents::RightArrow,
            Some('D') => TuiEvents::LeftArrow,
            Some('3') => {
                if let Some('~') = self.read_raw_immediate() {
                    return TuiEvents::Delete;
                }
                TuiEvents::Error
            }
            Some('<') => self.handle_mouse_events(),
            Some(_) | None => TuiEvents::Error,
        };
    }
    fn handle_mouse_events(&self) -> TuiEvents {
        return match self.read_raw_immediate() {
            Some('0') => match self.get_coordinates() {
                Some((coordinates, 'M')) => TuiEvents::LeftClick(coordinates),
                Some((_, 'm')) => TuiEvents::Ignore,
                None | Some(_) => TuiEvents::Error,
            },
            Some('1') => match self.get_coordinates() {
                Some((coordinates, 'M')) => TuiEvents::MiddleClick(coordinates),
                Some((_, 'm')) => TuiEvents::Ignore,
                None | Some(_) => TuiEvents::Error,
            },
            Some('2') => match self.get_coordinates() {
                Some((coordinates, 'M')) => TuiEvents::RightClick(coordinates),
                Some((_, 'm')) => TuiEvents::Ignore,
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
                Some((coordinates, 'M')) => TuiEvents::LeftDrag(coordinates),
                Some(_) | None => TuiEvents::Error,
            },
            Some('3') => match self.get_coordinates() {
                Some((coordinates, 'M')) => TuiEvents::MiddleDrag(coordinates),
                Some(_) | None => TuiEvents::Error,
            },
            Some('4') => match self.get_coordinates() {
                Some((coordinates, 'M')) => TuiEvents::RightDrag(coordinates),
                Some(_) | None => TuiEvents::Error,
            },
            Some('5') => match self.get_coordinates() {
                Some((coordinates, 'm')) => TuiEvents::MouseMove(coordinates),
                Some(_) | None => TuiEvents::Error,
            },
            None | Some(_) => TuiEvents::Error,
        };
    }

    fn handle_scroll_event(&self) -> TuiEvents {
        return match self.read_raw_immediate() {
            Some('4') => match self.get_coordinates() {
                Some((coordinates, 'M')) => TuiEvents::ScrollUp(coordinates),
                None | Some(_) => TuiEvents::Error,
            },
            Some('5') => match self.get_coordinates() {
                Some((coordinates, 'M')) => TuiEvents::ScrollDown(coordinates),
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

    fn get_coordinates(&self) -> Option<((u16, u16), char)> {
        let Some(';') = self.read_raw_immediate() else {
            return None;
        };
        let Some((x, ';')) = self.get_u16() else {
            return None;
        };
        return match self.get_u16() {
            Some((y, c)) => Some(((x, y), c)),
            None => None,
        };
    }
}
