use anyhow::Result;
use crossterm::event::{Event, KeyCode};

use crate::{
    app::{App, AppMode},
    terminal::Tui,
};

pub fn handle_event(app: &mut App, terminal: &mut Tui, event: Event) -> Result<()> {
    if let Event::Key(key) = event {
        match app.mode {
            AppMode::Normal => {
                handle_normal_mode(app, terminal, key.code)?;
            }
            AppMode::CreatingNote => {
                handle_creating_note_mode(app, key.code)?;
            }
            AppMode::Searching => {
                handle_search_mode(app, key.code);
            }
        }
    }

    Ok(())
}

fn handle_normal_mode(app: &mut App, terminal: &mut Tui, key_code: KeyCode) -> Result<()> {
    match key_code {
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
        KeyCode::Enter => {
            app.open_selected_note(terminal)?;
        }
        KeyCode::Char('n') => {
            app.start_creating_note();
        }
        KeyCode::Char('/') => {
            app.start_searching();
        }
        _ => {}
    }

    Ok(())
}

fn handle_creating_note_mode(app: &mut App, key_code: KeyCode) -> Result<()> {
    match key_code {
        KeyCode::Esc => {
            app.cancel_creating_note();
        }
        KeyCode::Enter => {
            app.create_note_from_input()?;
        }
        KeyCode::Backspace => {
            app.pop_new_note_char();
        }
        KeyCode::Char(c) => {
            app.push_new_note_char(c);
        }
        _ => {}
    }

    Ok(())
}

fn handle_search_mode(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Esc => {
            app.cancel_searching();
        }
        KeyCode::Enter => {
            app.accept_search();
        }
        KeyCode::Backspace => {
            app.pop_search_char();
        }
        KeyCode::Down => {
            app.move_down();
        }
        KeyCode::Up => {
            app.move_up();
        }
        KeyCode::Char(c) => {
            app.push_search_char(c);
        }
        _ => {}
    }
}
