use crate::properties::Editor;
use crossterm::event;
use ratatui::backend::CrosstermBackend;

pub fn inputs(ed: &mut Editor<CrosstermBackend<std::io::Stdout>>) {
    if let event::Event::Key(key) = event::read().unwrap() {
        match (key.code, key.modifiers) {
            // Quit
            (event::KeyCode::Char('q'), event::KeyModifiers::ALT) => ed.quit = true,
            // Left
            (event::KeyCode::Char('h'), event::KeyModifiers::CONTROL) => {
                if ed.cx > 0 {
                    ed.cx = ed.cx.saturating_sub(1);
                    ed.rcx = ed.rcx.saturating_sub(1);
                    ed.pref = ed.rcx;
                } else {
                    ed.x_offset = ed.x_offset.saturating_sub(1);
                    ed.rcx = ed.rcx.saturating_sub(1);
                    ed.pref = ed.rcx;
                }
            }
            // Down
            (event::KeyCode::Char('j'), event::KeyModifiers::CONTROL) => {
                if ed.file.line(ed.rcy + 1).len_chars() < ed.pref {
                    ed.rcx = ed.file.line(ed.rcy + 1).len_chars().saturating_sub(2);
                    if ed.rcx < ed.x_offset {
                        let difference = ed.x_offset - ed.rcx;
                        ed.x_offset = ed.x_offset.saturating_sub(ed.scrwidth + difference - 1);
                        ed.cx = ed.rcx.saturating_sub(ed.x_offset);
                    } else {
                        ed.cx = ed.rcx.saturating_sub(ed.x_offset);
                    }
                } else {
                    ed.rcx = ed.pref;
                    if ed.rcx > ed.x_offset {
                        let difference = ed.x_offset.saturating_sub(ed.rcx);
                        ed.x_offset = ed.x_offset.saturating_sub(ed.scrwidth + difference + 2);
                        ed.cx = ed.scrwidth - 2;
                    } else {
                        ed.cx = ed.rcx.saturating_sub(ed.x_offset);
                    }
                }
                if ed.cy + 2 < ed.scrheight - 1 && ed.rcy < ed.line_count {
                    ed.cy += 1;
                    ed.rcy += 1;
                } else if ed.y_offset + ed.cy + 2 < ed.line_count {
                    ed.y_offset += 1;
                    ed.rcy += 1;
                }
            }
            // Up
            (event::KeyCode::Char('k'), event::KeyModifiers::CONTROL) => {
                if ed.rcy > ed.y_offset && ed.cy > 0 {
                    ed.rcy = ed.rcy.saturating_sub(1);
                    ed.cy = ed.cy.saturating_sub(1);
                } else if ed.rcy == ed.y_offset {
                    ed.rcy = ed.rcy.saturating_sub(1);
                    ed.y_offset = ed.y_offset.saturating_sub(1);
                }
            }
            // Right
            (event::KeyCode::Char('l'), event::KeyModifiers::CONTROL) => {
                if ed.cx + 2 < ed.scrwidth.saturating_sub(1)
                    && ed.cx + 2 < ed.file.line(ed.cy).len_chars()
                {
                    ed.cx += 1;
                    ed.rcx += 1;
                    ed.pref = ed.rcx;
                } else if ed.x_offset + ed.cx + 2 < ed.file.line(ed.cy).len_chars() {
                    ed.x_offset += 1;
                    ed.rcx += 1;
                    ed.pref = ed.rcx;
                }
            }

            _ => {}
        }
    }
}
