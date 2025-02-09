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
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen)?;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
    stdout().execute(cursor::MoveTo(cx, cy));
    let scrheight = terminal::size()?.1;
    let mut file = Rope::from_reader(BufReader::new(fs::File::open(file_name)?))?;
    for current_row in 0..scrheight {
        if <u16 as Into<usize>>::into(current_row) < file.len_lines() {
            print!("{}\r\n", file.line(current_row.into()));
        } else {
            print!("~\r\n");
        }
    }
    for b in stdin().bytes() {
        let c = b.unwrap() as char;
        if c == 'q' {
            break;
        }
    }

    shutdown();
    Ok(())
}

fn shutdown() {
    execute!(stdout(), terminal::LeaveAlternateScreen);
    terminal::disable_raw_mode();
}
