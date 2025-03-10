use crate::properties::Editor;
use crossterm::{cursor, event, execute, terminal, ExecutableCommand};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Write};

pub fn inputs(ed: &mut Editor<CrosstermBackend<std::io::Stdout>>) {
    if let event::Event::Key(key) = event::read().unwrap() {
        match (key.code, key.modifiers) {
            (event::KeyCode::Char('q'), event::KeyModifiers::ALT) => ed.quit = true, // Quit
            (event::KeyCode::Char('j'), event::KeyModifiers::CONTROL) => {
                if <u16 as std::convert::Into<usize>>::into(ed.cy) + 2 < ed.scrheight - 1
                    && <u16 as std::convert::Into<usize>>::into(ed.cy) + 2 < ed.line_count
                {
                    ed.cy += 1;
                } else if ed.scroll_offset + <u16 as std::convert::Into<usize>>::into(ed.cy) + 2
                    < ed.line_count
                {
                    ed.scroll_offset += 1;
                }
            }
            (event::KeyCode::Char('k'), event::KeyModifiers::CONTROL) => {
                if ed.cy > 0 {
                    ed.cy -= 1
                } else {
                    ed.scroll_offset = ed.scroll_offset.saturating_sub(1);
                }
            }
            _ => {}
        }
    }
}
