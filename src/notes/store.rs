use std::{fs, path::PathBuf};

use anyhow::Result;

pub struct NotesStore {
    pub root: PathBuf,
}

impl NotesStore {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().expect("Could not find home directory");

        let root = home.join("notes");

        fs::create_dir_all(&root)?;

        let store = Self { root };

        store.ensure_default_folders()?;

        Ok(store)
    }

    pub fn folders(&self) -> Result<Vec<String>> {
        let mut folders = Vec::new();

        let entries = fs::read_dir(&self.root)?;

        for entry_result in entries {
            let entry = entry_result?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    folders.push(name.to_string_lossy().to_string());
                }
            }
        }

        folders.sort();

        Ok(folders)
    }

    pub fn create_note(&self, folder: &str, note_name: &str) -> Result<()> {
        let path = self.root.join(folder).join(note_name);

        if path.exists() {
            return Ok(());
        }

        fs::write(path, "# New Note\n")?;

        Ok(())
    }

    pub fn notes_in_folder(&self, folder: &str) -> Result<Vec<String>> {
        let mut notes = Vec::new();
        let folder_path = self.root.join(folder);

        if !folder_path.exists() {
            return Ok(notes);
        }

        let entries = fs::read_dir(folder_path)?;

        for entry_result in entries {
            let entry = entry_result?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "md" {
                        if let Some(name) = path.file_name() {
                            notes.push(name.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        notes.sort();

        Ok(notes)
    }

    pub fn note_path(&self, folder: &str, note: &str) -> PathBuf {
        self.root.join(folder).join(note)
    }

    fn ensure_default_folders(&self) -> Result<()> {
        let defaults = ["daily", "projects", "snippets", "archive"];

        for folder in defaults {
            let path = self.root.join(folder);
            fs::create_dir_all(path)?;
        }

        Ok(())
    }
}
