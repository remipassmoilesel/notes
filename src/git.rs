#[cfg(test)]
use mockall::automock;

use crate::console_output::ConsoleOutput;
use crate::default_error::DefaultError;
use crate::note::Note;
use crate::shell::Shell;

#[cfg_attr(test, automock)]
pub trait Git {
    fn init(&self) -> Result<ConsoleOutput, DefaultError>;
    fn commit(&self, note: &Note, message: &str) -> Result<ConsoleOutput, DefaultError>;
    fn has_changed(&self, note: &Note) -> bool;
    fn push(&self) -> Result<ConsoleOutput, DefaultError>;
    fn pull(&self) -> Result<ConsoleOutput, DefaultError>;
}

pub struct GitImpl<'a> {
    shell: &'a dyn Shell,
}

impl<'a> GitImpl<'a> {
    pub fn new(shell: &'a dyn Shell) -> GitImpl<'a> {
        GitImpl { shell }
    }
}

impl<'a> Git for GitImpl<'a> {
    fn init(&self) -> Result<ConsoleOutput, DefaultError> {
        match self.shell.execute_in_repo("git init") {
            Ok(o) => Ok(o.into()),
            Err(e) => Err(e),
        }
    }

    fn commit(&self, note: &Note, message: &str) -> Result<ConsoleOutput, DefaultError> {
        let path = &note.path.to_str().unwrap();
        let mut out = ConsoleOutput::empty();
        out.append_command_output(self.shell.execute_in_repo(format!("git add '{}'", path).as_str())?);
        out.append_command_output(self.shell.execute_in_repo(format!("git commit -m '{}' '{}'", message, path).as_str())?);
        Ok(out)
    }

    fn has_changed(&self, note: &Note) -> bool {
        let path = note.path.to_str().unwrap();
        self.shell
            .execute_in_repo(format!("git add '{p}' && git diff --exit-code HEAD '{p}' > /dev/null", p = path).as_str())
            .is_err()
    }

    fn push(&self) -> Result<ConsoleOutput, DefaultError> {
        match self.shell.execute_interactive_in_repo("git push") {
            Ok(o) => Ok(o.into()),
            Err(e) => Err(e),
        }
    }

    fn pull(&self) -> Result<ConsoleOutput, DefaultError> {
        match self.shell.execute_interactive_in_repo("git pull") {
            Ok(o) => Ok(o.into()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::{CommandOutput, MockShell};
    use std::path::PathBuf;

    fn test_note() -> Note {
        Note::from(0, PathBuf::from("/repository/test.md"), "# Title\nContent\n".to_string()).unwrap()
    }

    #[test]
    fn init() {
        let mut shell_mock = MockShell::new();
        let exp_command = "git init";
        shell_mock
            .expect_execute_in_repo()
            .times(1)
            .withf(move |c| c == exp_command)
            .returning(|_| Ok(CommandOutput::default()));

        let git = GitImpl::new(&shell_mock);
        git.init().unwrap();
    }

    #[test]
    fn commit() {
        let note = test_note();

        let mut shell_mock = MockShell::new();
        let exp_command = format!("git add '{}'", note.path.to_str().unwrap());
        shell_mock
            .expect_execute_in_repo()
            .times(1)
            .withf(move |c| c == exp_command)
            .returning(|_| Ok(CommandOutput::default()));

        let exp_command = format!("git commit -m 'message' '{}'", note.path.to_str().unwrap());
        shell_mock
            .expect_execute_in_repo()
            .times(1)
            .withf(move |c| c == exp_command)
            .returning(|_| Ok(CommandOutput::default()));

        let git = GitImpl::new(&shell_mock);
        git.commit(&note, "message").unwrap();
    }

    #[test]
    fn has_changed() {
        let note = test_note();

        let mut shell_mock = MockShell::new();
        let exp_command = format!("git add '{p}' && git diff --exit-code HEAD '{p}' > /dev/null", p = note.path.to_str().unwrap());
        shell_mock
            .expect_execute_in_repo()
            .times(1)
            .withf(move |c| c == exp_command)
            .returning(|_| Err(DefaultError::new("".to_string())));

        let git = GitImpl::new(&shell_mock);
        assert!(git.has_changed(&note));
    }

    #[test]
    fn push() {
        let mut shell_mock = MockShell::new();
        let exp_command = "git push";
        shell_mock
            .expect_execute_interactive_in_repo()
            .times(1)
            .withf(move |c| c == exp_command)
            .returning(|_| Ok(CommandOutput::default()));

        let git = GitImpl::new(&shell_mock);
        git.push().unwrap();
    }

    #[test]
    fn pull() {
        let mut shell_mock = MockShell::new();
        let exp_command = "git pull";
        shell_mock
            .expect_execute_interactive_in_repo()
            .times(1)
            .withf(move |c| c == exp_command)
            .returning(|_| Ok(CommandOutput::default()));

        let git = GitImpl::new(&shell_mock);
        git.pull().unwrap();
    }
}
