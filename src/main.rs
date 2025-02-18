use anyhow;
use crossterm::{cursor, event, execute, terminal, ExecutableCommand};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
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
use shutdown::shutdown;

enum Mode {
    Sun,
    Moon,
    Cast,
    Sky,
    Star,
}

enum Actions {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<OsString> = args_os().collect();
    let mut file_name: &OsString = &args[1];
    let mut cx: u16 = 0;
    let mut cy: u16 = 0;
    let mut mode = Mode::Sun;
    let mut scroll_offset = 0usize;
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen)?;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
    execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    let scrheight: usize = terminal::size()?.1.into();
    let mut file = Rope::from_reader(BufReader::new(fs::File::open(file_name)?))?;
    let line_count = file.len_lines();
    let backend = CrosstermBackend::new(stdout());
    let mut screen = Terminal::new(backend)?;
    execute!(stdout(), event::EnableMouseCapture).unwrap();
    loop {
        draw::draw(&mut screen, &mut file, line_count, scroll_offset);
        if let event::Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char('q') => break, // Quit
                event::KeyCode::Char('j') => {
                    if scroll_offset + 1 < line_count {
                        scroll_offset += 1;
                    }
                }
                event::KeyCode::Char('k') => {
                    if scroll_offset > 0 {
                        scroll_offset -= 1;
                    }
                }
                _ => {}
            }
        }
        mouse::scan(&line_count, &mut scroll_offset);
    }

    shutdown();
    Ok(())
}
