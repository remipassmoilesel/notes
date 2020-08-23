use std::fs;
use std::fs::{DirEntry, File};
use std::io::Write;
use std::path::PathBuf;

#[cfg(test)]
use mockall::automock;

use crate::config::Config;
use crate::default_error::DefaultError;
use crate::git::Git;
use crate::note::Note;
use crate::shell::Shell;

#[cfg_attr(test, automock)]
pub trait Repository {
    fn init(&self) -> Result<(), DefaultError>;
    fn new_note(&self, path: &String) -> Result<(), DefaultError>;
    fn find_note_by_id(&self, id: usize) -> Option<Note>;
    fn get_notes(&self) -> Vec<Note>;
    fn write_note(&self, note: &Note) -> Result<(), DefaultError>;
    fn edit_note(&self, note: &Note) -> Result<(), DefaultError>;
    fn delete_note(&self, note: &Note) -> Result<(), DefaultError>;
    fn push_repo(&self) -> Result<(), DefaultError>;
    fn pull_repo(&self) -> Result<(), DefaultError>;
}

pub struct RepositoryImpl<'a> {
    config: &'a Config,
    shell: &'a dyn Shell,
    git: &'a dyn Git,
}

impl<'a> RepositoryImpl<'a> {
    pub fn new(config: &'a Config, shell: &'a dyn Shell, git: &'a dyn Git) -> RepositoryImpl<'a> {
        RepositoryImpl { config, shell, git }
    }
}

impl<'a> Repository for RepositoryImpl<'a> {
    fn init(&self) -> Result<(), DefaultError> {
        if !self.config.template_path.exists() {
            fs::create_dir_all(&self.config.storage_directory)?;

            self.git.init()?;

            let content = "# Note template\n\nHere we go !\n\n";
            let note = Note::from(0, self.config.template_path.clone(), content.to_string())?;
            self.write_note(&note)?;
            self.git.commit(&note, "Create note template".to_string())?
        }
        Ok(())
    }

    fn new_note(&self, path: &String) -> Result<(), DefaultError> {
        let path: PathBuf = [self.config.storage_directory.to_str().unwrap(), path.as_str()].iter().collect();

        if path.exists() {
            return Err(DefaultError::new(format!("Already exists: {}", path.to_str().unwrap())));
        }

        fs::create_dir_all(path.parent().unwrap())?;
        fs::copy(&self.config.template_path, &path)?;

        let id = self.get_notes().len() + 1;
        let content = fs::read_to_string(&path)?;
        let note = Note::from(id, path, content)?;
        self.edit_note(&note)?;
        self.git
            .commit(&note, format!("Create note {}", note.path.file_name().unwrap().to_str().unwrap()))?;
        Ok(())
    }

    fn find_note_by_id(&self, id: usize) -> Option<Note> {
        let notes = self.get_notes();
        notes.get(id - 1).map(|note| (*note).clone())
    }

    // TODO: use https://docs.rs/walkdir/2.3.1/walkdir/
    fn get_notes(&self) -> Vec<Note> {
        let ignored_dirs = vec!([".git", ".idea"]);
        let mut dir_entries: Vec<DirEntry> = fs::read_dir(&self.config.storage_directory)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|file| !ignored_dirs.contains(file.file_name()))
            .collect();
        dir_entries.sort_by(|a, b| a.path().cmp(&b.path()));

        let mut index = 0;
        let res = dir_entries
            .iter()
            .map(|file| {
                index += 1;
                (index, file)
            })
            .map(|(index, file)| {
                let path: PathBuf = file.path();
                let content = fs::read_to_string(file.path()).unwrap_or(format!("Error while reading file: {}", path.to_str().unwrap()));
                Note::from(index, path, content)
            })
            .filter_map(Result::ok)
            .collect();
        res
    }

    fn write_note(&self, note: &Note) -> Result<(), DefaultError> {
        let mut file = File::create(&note.path)?;
        file.write_all(note.format_for_write().as_bytes())?;
        Ok(())
    }

    fn edit_note(&self, note: &Note) -> Result<(), DefaultError> {
        let path = note.path.to_str().unwrap();
        self.shell.execute_in_repo(format!("$EDITOR {}", path))?;
        let file_has_changed = self.shell.execute_in_repo(format!("git diff --exit-code {} > /dev/null", path)).is_err();
        if file_has_changed {
            self.git
                .commit(&note, format!("Update note {}", note.path.file_name().unwrap().to_str().unwrap()))?;
        }
        Ok(())
    }

    fn delete_note(&self, note: &Note) -> Result<(), DefaultError> {
        let path = &note.path;
        fs::remove_file(path)?;
        self.git.commit(&note, format!("Delete note {}", path.file_name().unwrap().to_str().unwrap()))
    }

    fn push_repo(&self) -> Result<(), DefaultError> {
        self.git.push()
    }

    fn pull_repo(&self) -> Result<(), DefaultError> {
        self.git.pull()
    }
}
