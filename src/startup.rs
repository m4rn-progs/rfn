use crate::modal::Mode;
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
use ropey::Rope;
use std::ffi::OsString;
use std::fs;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Write};
use std::{env::args_os, fs::File};

pub fn startup() -> Editor<CrosstermBackend<std::io::Stdout>> {
    let args: Vec<OsString> = args_os().collect();
    let osfile_name: &OsString = &args[1];
    let mut osfile = Rope::from_reader(BufReader::new(File::open(osfile_name).unwrap())).unwrap();
    let backend = CrosstermBackend::new(stdout());
    let mut osscreen = Terminal::new(backend).unwrap();
    let dimensions = terminal::size().unwrap();
    let screenheight: usize = dimensions.1.into();
    let screenwidth: usize = dimensions.0.into();
    let file_line_count = osfile.len_lines();
    let _ = terminal::enable_raw_mode();
    execute!(stdout(), terminal::EnterAlternateScreen).unwrap();
    execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout(), event::EnableMouseCapture).unwrap();
    Editor {
        cx: 0,
        cy: 0,
        rcx: 0,
        rcy: 0,
        pref: 0,
        scrheight: screenheight,
        scrwidth: screenwidth,
        x_offset: 0,
        y_offset: 0,
        file_name: osfile_name.clone(),
        line_count: file_line_count,
        file: osfile,
        screen: osscreen,
        mode: Mode::Sun,
        quit: false,
    }
}
