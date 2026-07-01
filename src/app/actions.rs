use std::process::Command;

use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::terminal::Tui;

use super::{ActivePanel, App};

impl App {
    pub fn open_selected_note(&mut self, terminal: &mut Tui) -> Result<()> {
        let Some(folder) = self.folders.get(self.selected_folder) else {
            return Ok(());
        };

        let Some(note) = self.notes.get(self.selected_note) else {
            return Ok(());
        };

        let path = self.notes_store.note_path(folder, note);

        disable_raw_mode()?;

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());

        Command::new(editor).arg(path).status()?;

        enable_raw_mode()?;

        terminal.clear()?;

        Ok(())
    }

    pub fn refresh_notes_for_selected_folder(&mut self) {
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
                if self.selected_folder + 1 < self.folders.len() {
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
