use regex::RegexBuilder;

use crate::banners::Banners;
use crate::cli_format::CliFormat;
use crate::console_output::ConsoleOutput;
use crate::default_error::DefaultError;
use crate::note::Note;
use crate::repository::Repository;
use crate::search_match::SearchMatch;
use crate::usage::usage;

#[derive(Debug, PartialEq)]
pub enum Command {
    New { path: String },
    List,
    Search { needle: String },
    Edit { id: usize },
    Delete { id: usize },
    Push,
    Pull,
    Help,
}

pub struct CommandHandler<'a> {
    repository: &'a dyn Repository,
    formatter: &'a dyn CliFormat,
}

impl<'a> CommandHandler<'a> {
    pub fn new(repository: &'a dyn Repository, formatter: &'a dyn CliFormat) -> CommandHandler<'a> {
        CommandHandler { repository, formatter }
    }

    pub fn apply_command(&self, command: Command) -> Result<ConsoleOutput, DefaultError> {
        match command {
            Command::New { path } => self.new_note(path),
            Command::List => self.list_notes(),
            Command::Search { needle } => self.search(needle),
            Command::Edit { id } => self.edit_note(id),
            Command::Delete { id } => self.delete_note(id),
            Command::Push => self.push_repo(),
            Command::Pull => self.pull_repo(),
            Command::Help => self.help(),
        }
    }

    fn new_note(&self, path: String) -> Result<ConsoleOutput, DefaultError> {
        let mut out = ConsoleOutput::empty();
        let mut final_path = path.clone();
        if !final_path.ends_with(".md") {
            final_path = format!("{}.md", path)
        }
        let id = self.repository.load_notes().len();
        let note = self.repository.new_note(id, &final_path)?;
        out.append(self.repository.edit_note(&note)?);

        out.append_stdout(&format!("\nNote '{}' created\n", &note.path.to_str().unwrap()));
        Ok(out)
    }

    fn search(&self, needle: String) -> Result<ConsoleOutput, DefaultError> {
        let mut out = ConsoleOutput::empty();
        let notes: Vec<Note> = self.repository.load_notes();
        let needle_regex = RegexBuilder::new(&format!("({})", needle)).case_insensitive(true).build().unwrap();

        let mut matches: Vec<SearchMatch> = notes
            .iter()
            .map(|note| note.search_match(&needle_regex))
            .filter(|search_m| search_m.score.gt(&0))
            .collect();
        matches.sort_by(|a, b| b.score.cmp(&a.score));

        matches
            .iter()
            .for_each(|search_m| out.append_stdout(&format!("{}\n\n", self.formatter.search_match(search_m))));

        out.append_stdout(&format!("{} results found for '{}'\n", matches.len(), needle));

        Ok(out)
    }

    fn list_notes(&self) -> Result<ConsoleOutput, DefaultError> {
        let mut out = ConsoleOutput::empty();
        let entries = self.repository.load_repository_tree();

        for entry in entries {
            let pad: Vec<&str> = vec![0; entry.level].iter().map(|_| "  ").collect();
            out.append_stdout(&format!("{}{}\n", pad.join(""), self.formatter.note_directory(&entry.name)));
            entry
                .notes
                .iter()
                .for_each(|n| out.append_stdout(&format!("{}{}\n", pad.join(""), self.formatter.note_list_item(n))));
            out.append_stdout("\n");
        }
        Ok(out)
    }

    fn edit_note(&self, id: usize) -> Result<ConsoleOutput, DefaultError> {
        let mut out = ConsoleOutput::empty();
        match self.repository.find_note_by_id(id) {
            Some(n) => {
                out.append(self.repository.edit_note(&n)?);
                out.append_stdout(&format!("\nNote '{}' edited\n", n.path.to_str().unwrap()));
                Ok(out)
            }
            None => Err(DefaultError::new(format!("Note with id {} not found.", id))),
        }
    }

    fn delete_note(&self, id: usize) -> Result<ConsoleOutput, DefaultError> {
        let mut out = ConsoleOutput::empty();
        match self.repository.find_note_by_id(id) {
            Some(n) => {
                out.append(self.repository.delete_note(&n)?);
                out.append_stdout(&format!("\nNote '{}' deleted\n", n.path.to_str().unwrap()));
                Ok(out)
            }
            None => Err(DefaultError::new(format!("Note with id {} not found.", id))),
        }
    }

    fn push_repo(&self) -> Result<ConsoleOutput, DefaultError> {
        let mut out = ConsoleOutput::empty();
        out.append_stdout(&format!("{}\n", Banners::big()));
        out.append(self.repository.push_repo()?);
        Ok(out)
    }

    fn pull_repo(&self) -> Result<ConsoleOutput, DefaultError> {
        let mut out = ConsoleOutput::empty();
        out.append_stdout(&format!("{}\n", Banners::big()));
        out.append(self.repository.pull_repo()?);
        Ok(out)
    }

    fn help(&self) -> Result<ConsoleOutput, DefaultError> {
        let mut out = ConsoleOutput::empty();
        out.append_stdout(&format!("{}\n", Banners::big()));
        out.append_stdout(&format!("{}\n", &usage()));
        Ok(out)
    }
}

// TODO: better assertions on output

#[cfg(test)]
mod tests {
    use mockall::predicate::*;

    use crate::cli_format::MockCliFormat;
    use crate::repository::{MockRepository, RepositoryDir};

    use super::*;
    use std::path::PathBuf;

    fn test_notes() -> Vec<Note> {
        vec![
            Note::from(0, "0.md".into(), "# Note 0 title \n\n Note 0 content".to_string()).unwrap(),
            Note::from(1, "0.md".into(), "# Note 1 title \n\n Note 1 content".to_string()).unwrap(),
            Note::from(2, "0.md".into(), "# Note 2 title \n\n Note 2 content".to_string()).unwrap(),
        ]
    }

    fn test_note_tree() -> Vec<RepositoryDir> {
        vec![RepositoryDir {
            path: PathBuf::from("/path/to/dir"),
            name: "to/dir".to_string(),
            level: 2,
            notes: test_notes(),
        }]
    }

    #[test]
    fn new_note_should_add_suffix() {
        let path = "new/note".to_string();

        let mut repo_mock = MockRepository::new();
        repo_mock.expect_load_notes().times(1).returning(test_notes);

        repo_mock
            .expect_new_note()
            .with(eq(3), eq("new/note.md"))
            .times(1)
            .returning(|_, _| Ok(test_notes()[0].clone()));

        repo_mock
            .expect_edit_note()
            .with(eq(test_notes()[0].clone()))
            .times(1)
            .returning(|_| Ok(ConsoleOutput::empty()));

        let fmt_mock = MockCliFormat::new();
        let handler = CommandHandler::new(&repo_mock, &fmt_mock);

        let res = handler.apply_command(Command::New { path });
        assert!(res.is_ok())
    }

    #[test]
    fn search_notes_should_find_nothing() {
        let mut repo_mock = MockRepository::new();
        repo_mock.expect_load_notes().times(1).returning(test_notes);

        let mut fmt_mock = MockCliFormat::new();
        fmt_mock.expect_search_match().times(0).returning(|_| "".to_string());

        let handler = CommandHandler::new(&repo_mock, &fmt_mock);

        let res = handler.apply_command(Command::Search { needle: "abcdef".to_string() });
        assert!(res.is_ok())
    }

    #[test]
    fn search_notes_should_find_note() {
        let mut repo_mock = MockRepository::new();
        repo_mock.expect_load_notes().times(1).returning(test_notes);

        let mut fmt_mock = MockCliFormat::new();
        fmt_mock
            .expect_search_match()
            .times(1)
            .withf(|search_m| search_m.id == 2)
            .returning(|search_m| search_m.title.clone());

        let handler = CommandHandler::new(&repo_mock, &fmt_mock);

        let res = handler.apply_command(Command::Search { needle: "2".to_string() });
        assert!(res.is_ok())
    }

    #[test]
    fn list_notes() {
        let mut repo_mock = MockRepository::new();
        repo_mock.expect_load_repository_tree().times(1).returning(test_note_tree);

        let mut fmt_mock = MockCliFormat::new();
        fmt_mock.expect_note_directory().times(1).withf(|t| t == "to/dir").returning(|t| t.to_string());

        let note_titles: Vec<String> = test_notes().iter().map(|n| n.title.clone()).collect();
        fmt_mock
            .expect_note_list_item()
            .times(3)
            .withf(move |n| note_titles.iter().any(|tb| &n.title == tb))
            .returning(|n| n.title.clone());

        let handler = CommandHandler::new(&repo_mock, &fmt_mock);
        let res = handler.apply_command(Command::List);
        assert!(res.is_ok())
    }

    #[test]
    fn edit_note() {
        let mut repo_mock = MockRepository::new();
        let note = test_notes()[1].clone();
        let note_id = note.id;

        repo_mock
            .expect_find_note_by_id()
            .times(1)
            .withf(move |id| *id == note_id)
            .return_const(Some(note));
        repo_mock
            .expect_edit_note()
            .times(1)
            .withf(move |n| n.id == note_id)
            .returning(|_| Ok(ConsoleOutput::empty()));

        let fmt_mock = MockCliFormat::new();

        let handler = CommandHandler::new(&repo_mock, &fmt_mock);
        let res = handler.apply_command(Command::Edit { id: note_id });
        assert!(res.is_ok())
    }

    #[test]
    fn edit_note_should_find_nothing() {
        let mut repo_mock = MockRepository::new();
        let note_id = 5;

        repo_mock.expect_find_note_by_id().times(1).withf(move |id| *id == note_id).return_const(None);

        let fmt_mock = MockCliFormat::new();

        let handler = CommandHandler::new(&repo_mock, &fmt_mock);
        let res = handler.apply_command(Command::Edit { id: note_id });
        assert_eq!(res.unwrap_err().message, "Note with id 5 not found.")
    }

    #[test]
    fn delete_note() {
        let mut repo_mock = MockRepository::new();
        let note = test_notes()[1].clone();
        let note_id = note.id;

        repo_mock
            .expect_find_note_by_id()
            .times(1)
            .withf(move |id| *id == note_id)
            .return_const(Some(note));
        repo_mock
            .expect_delete_note()
            .times(1)
            .withf(move |n| n.id == note_id)
            .returning(|_| Ok(ConsoleOutput::empty()));

        let fmt_mock = MockCliFormat::new();

        let handler = CommandHandler::new(&repo_mock, &fmt_mock);
        let res = handler.apply_command(Command::Delete { id: note_id });
        assert!(res.is_ok())
    }

    #[test]
    fn delete_note_should_find_nothing() {
        let mut repo_mock = MockRepository::new();
        let note_id = 5;

        repo_mock.expect_find_note_by_id().times(1).withf(move |id| *id == note_id).return_const(None);

        let fmt_mock = MockCliFormat::new();

        let handler = CommandHandler::new(&repo_mock, &fmt_mock);
        let res = handler.apply_command(Command::Delete { id: note_id });
        assert_eq!(res.unwrap_err().message, "Note with id 5 not found.")
    }

    #[test]
    fn push_repo() {
        let mut repo_mock = MockRepository::new();
        repo_mock.expect_push_repo().times(1).returning(|| Ok(ConsoleOutput::empty()));

        let fmt_mock = MockCliFormat::new();

        let handler = CommandHandler::new(&repo_mock, &fmt_mock);
        let res = handler.apply_command(Command::Push);
        assert!(res.is_ok())
    }

    #[test]
    fn pull_repo() {
        let mut repo_mock = MockRepository::new();
        repo_mock.expect_pull_repo().times(1).returning(|| Ok(ConsoleOutput::empty()));

        let fmt_mock = MockCliFormat::new();

        let handler = CommandHandler::new(&repo_mock, &fmt_mock);
        let res = handler.apply_command(Command::Pull);
        assert!(res.is_ok())
    }
}
