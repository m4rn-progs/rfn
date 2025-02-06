use anyhow;
use crossterm::{cursor, event, execute, terminal, ExecutableCommand};
use std::fs;
use std::io::{stdin, stdout, Read, Stdin, Write};
mod editor;

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
    let mut cx: u16 = 0;
    let mut cy: u16 = 0;
    let mut mode = Mode::Sun;
    startup();

    shutdown();
    Ok(())
}

fn startup() {
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen)?;
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
    stdout().execute(cursor::MoveTo(cx, cy));

    //      for b in stdin().bytes() {
    //            let c = b.unwrap() as char;
    //            println!("{}\r", c);
    //            if c == 'q' {
    //                break;
    //            }
    //        }
}
fn shutdown() {
    execute!(stdout(), terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
}
