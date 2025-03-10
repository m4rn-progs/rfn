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
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Stdin, Write};

pub fn draw(ed: &mut Editor<CrosstermBackend<std::io::Stdout>>) {
    let mod3 = format!("{:?}", ed.file_name);
    let _ = ed.screen.show_cursor();
    let _ = ed.screen.set_cursor_position(Position {
        x: ed.cx + 1,
        y: ed.cy + 1,
    });
    let _ = ed.screen.show_cursor();
    let _ = ed.screen.draw(|frame| {
        let size = frame.area();
        let mut lines = Vec::new();
        //execute!(stdout(), cursor::MoveTo(cx, cy)).unwrap();
        for current_row in
            ed.scroll_offset..(ed.scroll_offset + size.height as usize).min(ed.line_count)
        {
            let text = ed.file.line(current_row).to_string();
            lines.push(Line::from(Span::raw(text)));
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(mod3))
            .style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Rgb(193, 11, 11)),
            );
        frame.render_widget(paragraph, size);
    });
    let _ = ed.screen.set_cursor_position(Position {
        x: ed.cx + 1,
        y: ed.cy + 1,
    });
    let _ = ed.screen.show_cursor();
}
