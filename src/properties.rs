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
    pub cx: usize,
    pub cy: usize,
    pub rcx: usize,
    pub rcy: usize,
    pub pref: usize,
    pub scrheight: usize,
    pub scrwidth: usize,
    pub x_offset: usize,
    pub y_offset: usize,
    pub file_name: OsString,
    pub line_count: usize,
    pub file: Rope,
    pub screen: Terminal<T>,
    pub mode: Mode,
    pub quit: bool,
}
