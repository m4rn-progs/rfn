use anyhow;
use crossterm::{cursor, event, execute, terminal, ExecutableCommand};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Position},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use ropey::Rope;
use std::env::args_os;
use std::ffi::OsString;
use std::fs;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Write};
mod draw;
mod mouse;
mod shutdown;
mod startup;
use shutdown::shutdown;
use startup::startup;
mod modal;

fn main() -> anyhow::Result<()> {
    let mut ed = startup();
    loop {
        draw::draw(
            &mut ed.screen,
            &mut ed.file,
            ed.line_count,
            ed.scroll_offset,
            ed.cx,
            ed.cy,
            &ed.mode,
        );
        if let event::Event::Key(key) = event::read()? {
            match (key.code, key.modifiers) {
                (event::KeyCode::Char('q'), event::KeyModifiers::CONTROL) => break, // Quit
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
        mouse::scan(&ed.line_count, &mut ed.scroll_offset);
    }

    shutdown();
    Ok(())
}
