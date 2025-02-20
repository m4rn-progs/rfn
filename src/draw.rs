use crate::mode::Mode;
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

pub fn draw<T: ratatui::backend::Backend>(
    screen: &mut Terminal<T>,
    file: &mut Rope,
    line_count: usize,
    scroll_offset: usize,
    cx: u16,
    cy: u16,
    mode: &Mode,
) {
    let mut mod3 = format!("{:?}", mode);
    let _ = screen.show_cursor();
    let _ = screen.set_cursor_position(Position {
        x: cx + 1,
        y: cy + 1,
    });
    let _ = screen.show_cursor();
    let _ = screen.draw(|frame| {
        let size = frame.area();
        let mut lines = Vec::new();
        //execute!(stdout(), cursor::MoveTo(cx, cy)).unwrap();
        for current_row in scroll_offset..(scroll_offset + size.height as usize).min(line_count) {
            let text = file.line(current_row).to_string();
            lines.push(Line::from(Span::raw(text)));
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(mod3))
            .style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Rgb(241, 14, 14)),
            );
        frame.render_widget(paragraph, size);
    });
    let _ = screen.set_cursor_position(Position {
        x: cx + 1,
        y: cy + 1,
    });
    let _ = screen.show_cursor();
}
