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

pub fn draw<T: ratatui::backend::Backend>(
    screen: &mut Terminal<T>,
    file: &mut Rope,
    line_count: usize,
    scroll_offset: usize,
) {
    screen.draw(|frame| {
        let size = frame.area();
        let mut lines = Vec::new();
        for current_row in scroll_offset..(scroll_offset + size.height as usize).min(line_count) {
            let text = file.line(current_row).to_string();
            lines.push(Line::from(Span::raw(text)));
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Viewer"))
            .style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Rgb(115, 17, 30)),
            );
        frame.render_widget(paragraph, size);
    });
}
