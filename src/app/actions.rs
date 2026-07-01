use std::process::Command;

use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::terminal::Tui;

use super::{ActivePanel, App, AppMode};

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
                    self.all_notes = notes.clone();
                    self.notes = notes;
                    self.selected_note = 0;
                }
                Err(_) => {
                    self.all_notes.clear();
                    self.notes.clear();
                    self.selected_note = 0;
                }
            }
        }
    }

    pub fn start_searching(&mut self) {
        self.mode = AppMode::Searching;
        self.search_query.clear();
        self.status_message = "Search: type query, Enter to accept, Esc to cancel".to_string();
    }

    pub fn cancel_searching(&mut self) {
        self.mode = AppMode::Normal;
        self.search_query.clear();
        self.notes = self.all_notes.clone();
        self.selected_note = 0;
        self.status_message = "Search cancelled".to_string();
    }

    pub fn push_search_char(&mut self, c: char) {
        self.search_query.push(c);
        self.apply_search_filter();
    }

    pub fn pop_search_char(&mut self) {
        self.search_query.pop();
        self.apply_search_filter();
    }

    pub fn accept_search(&mut self) {
        self.mode = AppMode::Normal;
        self.status_message = if self.search_query.is_empty() {
            "Search cleared".to_string()
        } else {
            format!("Search: {}", self.search_query)
        };
    }

    fn apply_search_filter(&mut self) {
        let query = self.search_query.to_lowercase();

        if query.is_empty() {
            self.notes = self.all_notes.clone();
            self.selected_note = 0;
            return;
        }

        self.notes = self
            .all_notes
            .iter()
            .filter(|note| note.to_lowercase().contains(&query))
            .cloned()
            .collect();

        self.selected_note = 0;
    }

    pub fn start_creating_note(&mut self) {
        self.mode = AppMode::CreatingNote;
        self.new_note_name.clear();
        self.status_message = "New note: type a name, Enter to create, Esc to cancel.".to_string();
    }

    pub fn cancel_creating_note(&mut self) {
        self.mode = AppMode::Normal;
        self.new_note_name.clear();
        self.status_message = "Cancelled new note.".to_string();
    }

    pub fn push_new_note_char(&mut self, c: char) {
        self.new_note_name.push(c);
    }

    pub fn pop_new_note_char(&mut self) {
        self.new_note_name.pop();
    }

    pub fn create_note_from_input(&mut self) -> Result<()> {
        let Some(folder) = self.folders.get(self.selected_folder) else {
            self.status_message = "No folder selected.".to_string();
            return Ok(());
        };

        let mut file_name = self.new_note_name.trim().to_string();

        if file_name.is_empty() {
            self.status_message = "Note name cannot be empty.".to_string();
            return Ok(());
        }

        if !file_name.ends_with(".md") {
            file_name.push_str(".md");
        }

        self.notes_store.create_note(folder, &file_name)?;
        self.refresh_notes_for_selected_folder();

        if let Some(index) = self.notes.iter().position(|note| note == &file_name) {
            self.selected_note = index;
        }

        self.mode = AppMode::Normal;
        self.new_note_name.clear();
        self.active_panel = ActivePanel::Notes;
        self.status_message = format!("Created {file_name}");

        Ok(())
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
