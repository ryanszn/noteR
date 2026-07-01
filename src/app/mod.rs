mod actions;
mod state;

pub use state::{ActivePanel, AppMode};

use anyhow::Result;
use crossterm::event;

use crate::{input, notes::NotesStore, terminal::Tui, ui};

pub struct App {
    pub should_quit: bool,
    pub notes_store: NotesStore,
    pub folders: Vec<String>,
    pub notes: Vec<String>,
    pub selected_folder: usize,
    pub selected_note: usize,
    pub active_panel: ActivePanel,
    pub mode: AppMode,
    pub new_note_name: String,
    pub status_message: String,
}

impl App {
    pub fn new() -> Result<Self> {
        let notes_store = NotesStore::new()?;
        let folders = notes_store.folders()?;

        let notes = if let Some(first_folder) = folders.first() {
            notes_store.notes_in_folder(first_folder)?
        } else {
            Vec::new()
        };

        Ok(Self {
            should_quit: false,
            notes_store,
            folders,
            notes,
            selected_folder: 0,
            selected_note: 0,
            active_panel: ActivePanel::Folders,
            mode: AppMode::Normal,
            new_note_name: String::new(),
            status_message: "Ready".to_string(),
        })
    }

    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| {
                ui::draw(frame, self);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                let event = event::read()?;
                input::handle_event(self, terminal, event)?;
            }
        }

        Ok(())
    }
}
