use crate::modal::Mode;
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

pub struct Editor<T: ratatui::backend::Backend> {
    pub cx: u16,
    pub cy: u16,
    pub scrheight: usize,
    pub scroll_offset: usize,
    pub file_name: OsString,
    pub line_count: usize,
    pub file: Rope,
    pub screen: Terminal<T>,
    pub mode: Mode,
    pub quit: bool,
}
