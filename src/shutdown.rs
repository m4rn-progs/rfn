use crossterm::{cursor, event, execute, terminal, ExecutableCommand};
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Write};

pub fn shutdown() {
    execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode();
    execute!(stdout(), event::DisableMouseCapture).unwrap();
}
