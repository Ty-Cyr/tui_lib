use std::error::Error;

use super::tui_io::{InputInterface, OutputInterface, TerminalState};

pub trait TerminalTrait {
    fn setup_terminal() -> Result<(InputInterface, OutputInterface, TerminalState), Box<dyn Error>>;
    fn reset_terminal_settings(input_interface: &InputInterface, terminal_state: &TerminalState);
}
