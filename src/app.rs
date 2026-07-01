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
}

#[derive(Clone, Copy, PartialEq)]
pub enum ActivePanel {
    Folders,
    Notes,
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
        })
    }

    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| {
                ui::draw(frame, self);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                let event = event::read()?;
                input::handle_event(self, event);
            }
        }

        Ok(())
    }

    fn refresh_notes_for_selected_folder(&mut self) {
        if let Some(folder) = self.folders.get(self.selected_folder) {
            match self.notes_store.notes_in_folder(folder) {
                Ok(notes) => {
                    self.notes = notes;
                    self.selected_note = 0;
                }
                Err(_) => {
                    self.notes.clear();
                    self.selected_note = 0;
                }
            }
        }
    }

    pub fn move_down(&mut self) {
        match self.active_panel {
            ActivePanel::Folders => {
                if self.selected_note + 1 < self.folders.len() {
                    self.selected_folder += 1;
                    self.refresh_notes_for_selected_folder();
                }
            }
            ActivePanel::Notes => {
                if self.selected_note + 1 < self.notes.len() {
                    self.selected_note += 1;
                }
            }
        }
    }

    pub fn move_up(&mut self) {
        match self.active_panel {
            ActivePanel::Folders => {
                if self.selected_folder > 0 {
                    self.selected_folder -= 1;
                    self.refresh_notes_for_selected_folder();
                }
            }
            ActivePanel::Notes => {
                if self.selected_note > 0 {
                    self.selected_note -= 1;
                }
            }
        }
    }

    pub fn focus_left(&mut self) {
        self.active_panel = ActivePanel::Folders;
    }

    pub fn focus_right(&mut self) {
        self.active_panel = ActivePanel::Notes;
    }
}
