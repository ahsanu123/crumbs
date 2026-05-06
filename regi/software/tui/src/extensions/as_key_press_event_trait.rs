use crossterm::event::{Event, KeyEventKind};

pub trait EventExt {
    fn as_key_press_event(self) -> Option<crossterm::event::KeyEvent>;
}

impl EventExt for Event {
    fn as_key_press_event(self) -> Option<crossterm::event::KeyEvent> {
        match self {
            Event::Key(key) if key.kind == KeyEventKind::Press => Some(key),
            _ => None,
        }
    }
}
