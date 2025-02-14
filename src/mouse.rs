use crossterm::{cursor, event, execute, terminal, ExecutableCommand};

pub fn scan(line_count: &usize, scroll_offset: &mut usize) {
    if event::poll(std::time::Duration::from_millis(16)).unwrap() {
        if let event::Event::Mouse(mouse_event) = event::read().unwrap() {
            match mouse_event.kind {
                event::MouseEventKind::ScrollUp => {
                    if *scroll_offset > 0 {
                        *scroll_offset -= 1;
                    }
                }

                event::MouseEventKind::ScrollDown => {
                    if *scroll_offset < *line_count {
                        *scroll_offset += 1;
                    }
                }
                _ => {}
            }
        }
    }
}
