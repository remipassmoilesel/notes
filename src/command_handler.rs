use crate::banners::Banners;
use crate::default_error::DefaultError;
use crate::logger::Logger;
use crate::note::Note;
use crate::repository::Repository;
use crate::usage::USAGE;

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
}

impl<'a> CommandHandler<'a> {
    pub fn new(repository: &'a dyn Repository, log: &'a dyn Logger) -> CommandHandler<'a> {
        CommandHandler { repository, log }
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
        self.repository.new_note(&final_path)?;

        self.log.info(format!("Note '{}' created", &final_path));
        Ok(())
    }

    fn search(&self, needle: String) -> Result<(), DefaultError> {
        let notes: Vec<Note> = self.repository.get_notes();
        let mut scored: Vec<(usize, &Note)> = notes.iter().map(|note| (note.match_score(&needle), note)).filter(|(score, _)| score.ne(&0)).collect();
        scored.sort_by(|(score_a, _), (score_b, _)| score_b.cmp(&score_a));
        scored
            .iter()
            .map(|(score, note)| note.to_search_result(&needle, *score))
            .for_each(|search_result| self.log.log(format!("{}", search_result)));
        if scored.is_empty() {
            self.log.info(format!("Nothing found for: {}", needle));
        }

        Ok(())
    }

    fn list_notes(&self) -> Result<(), DefaultError> {
        let files = self.repository.get_notes();
        for file in files {
            self.log.log(format!("{}", file.format_for_list()));
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
        self.log.log(Banners::big());
        self.repository.push_repo()
    }

    fn pull_repo(&self) -> Result<(), DefaultError> {
        self.log.log(Banners::big());
        self.repository.pull_repo()
    }

    fn help(&self) -> Result<(), DefaultError> {
        self.log.log(Banners::big());
        self.log.log(USAGE.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logger::MockLogger;
    use crate::repository::MockRepository;
    use mockall::predicate::*;
    use std::path::PathBuf;

    fn test_notes() -> Vec<Note> {
        vec![
            Note::from(0, PathBuf::from("0.md"), String::from("# Note 0 title \n\n Note 0 content")).unwrap(),
            Note::from(0, PathBuf::from("0.md"), String::from("# Note 1 title \n\n Note 1 content")).unwrap(),
            Note::from(0, PathBuf::from("0.md"), String::from("# Note 2 title \n\n Note 2 content")).unwrap(),
        ]
    }

    #[test]
    fn new_note_should_add_suffix() {
        let path = String::from("new/note");

        let mut repo_mock = MockRepository::new();
        repo_mock.expect_new_note().with(eq(String::from("new/note.md"))).times(1).returning(|_| Ok(()));

        let mut log_mock = MockLogger::new();
        log_mock
            .expect_info()
            .with(eq(String::from("Note 'new/note.md' created")))
            .times(1)
            .returning(|_| ());

        let handler = CommandHandler::new(&repo_mock, &log_mock);

        let res = handler.apply_command(Command::New { path });
        assert!(res.is_ok())
    }

    #[test]
    fn search_notes_should_find_nothing() {
        let mut repo_mock = MockRepository::new();
        repo_mock.expect_get_notes().times(1).returning(|| test_notes());

        let mut log_mock = MockLogger::new();
        log_mock
            .expect_info()
            .times(1)
            .withf(|out| out.contains("Nothing found for: abcdef"))
            .returning(|_| ());

        let handler = CommandHandler::new(&repo_mock, &log_mock);

        let res = handler.apply_command(Command::Search {
            needle: String::from("abcdef"),
        });
        assert!(res.is_ok())
    }

    #[test]
    fn search_notes_should_find_note() {
        let mut repo_mock = MockRepository::new();
        repo_mock.expect_get_notes().times(1).returning(|| test_notes());

        let mut log_mock = MockLogger::new();
        log_mock.expect_log().times(1).withf(|out| out.contains("# Note 2 title")).returning(|_| ());

        let handler = CommandHandler::new(&repo_mock, &log_mock);

        let res = handler.apply_command(Command::Search { needle: String::from("2") });
        assert!(res.is_ok())
    }
}
