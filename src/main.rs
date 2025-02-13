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
    let mut scroll_offset: usize = 0;
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen)?;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
    execute!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    let scrheight: usize = terminal::size()?.1.into();
    let mut file = Rope::from_reader(BufReader::new(fs::File::open(file_name)?))?;
    draw(&mut file, scroll_offset, scrheight, cx, cy);
    for b in stdin().bytes() {
        let c = b.unwrap() as char;
        match c {
            'q' => break,
            'k' => {
                if cy > 0 {
                    execute!(stdout(), cursor::MoveUp(1)).unwrap();
                    cy -= 1;
                } else if scroll_offset > 0 {
                    scroll_offset -= 1;
                    draw(&mut file, scroll_offset, scrheight, cx, cy);
                    stdout().flush().unwrap();
                }
            }
            'j' => {
                if file.len_lines() < scrheight {
                    if usize::from(cy) > file.len_lines().saturating_sub(1) {
                        execute!(stdout(), cursor::MoveDown(1)).unwrap();
                        cy += 1;
                    }
                } else {
                    if usize::from(cy) >= scrheight {
                        execute!(stdout(), cursor::MoveDown(1)).unwrap();
                        cy += 1;
                    } else if scrheight + scroll_offset < file.len_lines() {
                        scroll_offset += 1;
                        draw(&mut file, scroll_offset, scrheight, cx, cy);
                        stdout().flush().unwrap();
                    }
                }
            }
            _ => {}
        }
        //draw(&mut file, scroll_offset, scrheight, cx, cy);
        stdout().flush().unwrap();
    }
    shutdown();
    Ok(())
}

fn draw(file: &mut Rope, scroll_offset: usize, scrheight: usize, cx: u16, cy: u16) {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All));
    //execute!(stdout(), cursor::MoveTo(cx, cy));
    for current_row in 0..scrheight - 1 {
        let line_idx = scroll_offset + current_row;
        if line_idx < file.len_lines() {
            print!("{}\r", file.line(line_idx));
        } else {
            print!("~\r\n");
        }
    }
    stdout().flush().unwrap();
}

fn shutdown() {
    execute!(stdout(), terminal::LeaveAlternateScreen);
    terminal::disable_raw_mode();
}
