use crossterm::event::{Event, KeyCode};

use crate::app::App;

pub fn handle_event(app: &mut App, event: Event) {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Char('q') => {
                app.should_quit = true;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                app.move_down();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.move_up();
            }
            KeyCode::Char('h') | KeyCode::Left => {
                app.focus_left();
            }
            KeyCode::Char('l') | KeyCode::Right => {
                app.focus_right();
            }
            _ => {}
        }
    }
}
