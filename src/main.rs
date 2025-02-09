use anyhow;
use crossterm::{cursor, event, execute, terminal, ExecutableCommand};
use ropey::Rope;
use std::env::args_os;
use std::ffi::OsString;
use std::fs;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Write};
use std::thread;

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
    for b in stdin().bytes() {
        let c = b.unwrap() as char;
        match c {
            'q' => break,
            'k' => {
                if cy > 0 {
                    cy -= 1;
                } else if scroll_offset > 0 {
                    scroll_offset -= 1;
                }
            }
            'j' => {
                if cy > 0 {
                    cy += 1;
                } else if scrheight + scroll_offset < file.len_lines() {
                    scroll_offset += 1;
                }
            }
            _ => {}
        }
        draw(&file, scroll_offset, scrheight);
        stdout().flush().unwrap();
    }
    shutdown();
    Ok(())
}

fn draw(file: &Rope, scroll_offset: usize, scrheight: usize) {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All));
    for current_row in 0..scrheight {
        let line_idx = scroll_offset + current_row;
        if line_idx < file.len_lines() {
            print!("{}\r", file.line(line_idx));
        } else {
            print!("~\r");
        }
    }
    stdout().flush().unwrap();
}

fn shutdown() {
    execute!(stdout(), terminal::LeaveAlternateScreen);
    terminal::disable_raw_mode();
}
