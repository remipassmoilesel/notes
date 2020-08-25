use crate::banners::Banners;
use crate::cli_format::CliFormat;
use crate::default_error::DefaultError;
use crate::logger::Logger;
use crate::note::Note;
use crate::repository::Repository;
use crate::search_match::SearchMatch;
use crate::usage::usage;
use regex::RegexBuilder;

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
    log: &'a dyn Logger,
    formatter: &'a dyn CliFormat,
}

impl<'a> CommandHandler<'a> {
    pub fn new(repository: &'a dyn Repository, log: &'a dyn Logger, formatter: &'a dyn CliFormat) -> CommandHandler<'a> {
        CommandHandler { repository, log, formatter }
    }

    pub fn apply_command(&self, command: Command) -> Result<(), DefaultError> {
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

    fn new_note(&self, path: String) -> Result<(), DefaultError> {
        let mut final_path = path.clone();
        if !final_path.ends_with(".md") {
            final_path = format!("{}.md", path)
        }
        let id = self.repository.load_notes().len();
        let note = self.repository.new_note(id, &final_path)?;
        self.repository.edit_note(&note)?;

        self.log.info(&format!("\nNote '{}' created", &final_path));
        Ok(())
    }

    fn search(&self, needle: String) -> Result<(), DefaultError> {
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
            .for_each(|search_m| self.log.log(&format!("{}", self.formatter.search_match(search_m))));
        if matches.is_empty() {
            self.log.info(&format!("Nothing found for: {}", needle));
        }

        Ok(())
    }

    fn list_notes(&self) -> Result<(), DefaultError> {
        let entries = self.repository.load_repository_tree();

        // TODO: move to CliFormat ?
        for entry in entries {
            let pad: Vec<&str> = vec![0; entry.level].iter().map(|_| "  ").collect();
            self.log.log(&format!("{}{}", pad.join(""), self.formatter.note_directory(&entry.name)));
            entry
                .notes
                .iter()
                .for_each(|n| self.log.log(&format!("{}{}", pad.join(""), self.formatter.note_list_item(n))));
            self.log.log(&"");
        }
        Ok(())
    }

    fn edit_note(&self, id: usize) -> Result<(), DefaultError> {
        match self.repository.find_note_by_id(id) {
            Some(n) => self.repository.edit_note(&n),
            None => Err(DefaultError::new(format!("Note with id {} not found.", id))),
        }
    }

    fn delete_note(&self, id: usize) -> Result<(), DefaultError> {
        match self.repository.find_note_by_id(id) {
            Some(n) => self.repository.delete_note(&n),
            None => Err(DefaultError::new(format!("Note with id {} not found.", id))),
        }
    }

    fn push_repo(&self) -> Result<(), DefaultError> {
        self.log.log(&Banners::big());
        self.repository.push_repo()
    }

    fn pull_repo(&self) -> Result<(), DefaultError> {
        self.log.log(&Banners::big());
        self.repository.pull_repo()
    }

    fn help(&self) -> Result<(), DefaultError> {
        self.log.log(&Banners::big());
        self.log.log(&usage());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli_format::MockCliFormat;
    use crate::logger::MockLogger;
    use crate::repository::MockRepository;
    use mockall::predicate::*;

    fn test_notes() -> Vec<Note> {
        vec![
            Note::from(0, "0.md".into(), "# Note 0 title \n\n Note 0 content".to_string()).unwrap(),
            Note::from(1, "0.md".into(), "# Note 1 title \n\n Note 1 content".to_string()).unwrap(),
            Note::from(2, "0.md".into(), "# Note 2 title \n\n Note 2 content".to_string()).unwrap(),
        ]
    }

    #[test]
    fn new_note_should_add_suffix() {
        let path = "new/note".to_string();

        let mut repo_mock = MockRepository::new();
        repo_mock.expect_load_notes().times(1).returning(|| test_notes());

        repo_mock
            .expect_new_note()
            .with(eq(3), eq("new/note.md".to_string()))
            .times(1)
            .returning(|_, _| Ok(test_notes()[0].clone()));

        repo_mock.expect_edit_note().with(eq(test_notes()[0].clone())).times(1).returning(|_| Ok(()));

        let mut log_mock = MockLogger::new();
        log_mock
            .expect_info()
            .withf(|out| out.contains("Note 'new/note.md' created"))
            .times(1)
            .returning(|_| ());

        let fmt_mock = MockCliFormat::new();

        let handler = CommandHandler::new(&repo_mock, &log_mock, &fmt_mock);

        let res = handler.apply_command(Command::New { path });
        assert!(res.is_ok())
    }

    #[test]
    fn search_notes_should_find_nothing() {
        let mut repo_mock = MockRepository::new();
        repo_mock.expect_load_notes().times(1).returning(|| test_notes());

        let mut log_mock = MockLogger::new();
        log_mock
            .expect_info()
            .times(1)
            .withf(|out| out.contains("Nothing found for: abcdef"))
            .returning(|_| ());

        let mut fmt_mock = MockCliFormat::new();
        fmt_mock.expect_search_match().times(0).returning(|_| "".to_string());

        let handler = CommandHandler::new(&repo_mock, &log_mock, &fmt_mock);

        let res = handler.apply_command(Command::Search { needle: "abcdef".to_string() });
        assert!(res.is_ok())
    }

    #[test]
    fn search_notes_should_find_note() {
        let mut repo_mock = MockRepository::new();
        repo_mock.expect_load_notes().times(1).returning(|| test_notes());

        let mut log_mock = MockLogger::new();
        log_mock.expect_log().times(1).withf(|out| out.contains("# Note 2 title")).returning(|_| ());

        let mut fmt_mock = MockCliFormat::new();
        fmt_mock
            .expect_search_match()
            .times(1)
            .withf(|search_m| search_m.id == 2)
            .returning(|search_m| search_m.title.clone());

        let handler = CommandHandler::new(&repo_mock, &log_mock, &fmt_mock);

        let res = handler.apply_command(Command::Search { needle: "2".to_string() });
        assert!(res.is_ok())
    }
}
