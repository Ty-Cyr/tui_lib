use crate::tui_enums::TuiMode;
use crate::tui_events::TuiEvent;
use std::io::stdin;

#[derive(Clone, Copy)]
pub struct TerminalState {}
pub struct InputInterface {}

impl InputInterface {
    pub fn get_event(&self) -> TuiEvent {
        return TuiEvent::Error;
    }
}

#[allow(unused)]
pub fn setup_terminal(tui_mode: &TuiMode) -> Option<(InputInterface, TerminalState)> {
    let stdin_fd = stdin().as_raw_fd();
    return Some((InputInterface {}, TerminalState {}));
}

#[allow(unused)]
pub fn reset_terminal_settings(input_interface: &InputInterface, terminal_state: &TerminalState) {}
