use crate::properties::Editor;
use crossterm::event;
use ratatui::backend::CrosstermBackend;

pub fn scan(ed: &mut Editor<CrosstermBackend<std::io::Stdout>>) {
    if event::poll(std::time::Duration::from_millis(16)).unwrap() {
        if let event::Event::Mouse(mouse_event) = event::read().unwrap() {
            match mouse_event.kind {
                event::MouseEventKind::ScrollUp => {
                    if ed.y_offset > 0 {
                        ed.y_offset -= 1;
                    }
                }

                event::MouseEventKind::ScrollDown => {
                    if ed.y_offset < ed.line_count {
                        ed.y_offset += 1;
                    }
                }
                _ => {}
            }
        }
    }
}
