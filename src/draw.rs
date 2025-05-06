use crate::properties::Editor;
use ratatui::{
    backend::CrosstermBackend,
    layout::Position,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(ed: &mut Editor<CrosstermBackend<std::io::Stdout>>) {
    let mod3 = format!(
        "cx: {:?}, cy: {:?}, rcx: {:?}, rcy: {:?}, pref: {:?}",
        ed.cx, ed.cy, ed.rcx, ed.rcy, ed.pref
    );
    let _ = ed.screen.show_cursor();
    let _ = ed.screen.set_cursor_position(Position {
        x: ed.cx as u16 + 1,
        y: ed.cy as u16 + 1,
    });
    let _ = ed.screen.show_cursor();
    let _ = ed.screen.draw(|frame| {
        let size = frame.area();
        let mut lines = Vec::new();
        //execute!(stdout(), cursor::MoveTo(cx, cy)).unwrap();
        for current_row in ed.y_offset..(ed.y_offset + size.height as usize).min(ed.line_count) {
            let text = ed.file.line(current_row);
            let str: ropey::RopeSlice = "".into();
            let trim = match ed.x_offset < text.len_chars() {
                true => text.slice(ed.x_offset..),
                false => str,
            };
            lines.push(Line::from(Span::raw(trim.to_string())));
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
        x: ed.cx as u16 + 1,
        y: ed.cy as u16 + 1,
    });
    let _ = ed.screen.show_cursor();
}
