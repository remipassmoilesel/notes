use crate::config::Config;
use crate::default_error::DefaultError;
use crate::note::Note;
use crate::shell::Shell;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Git {
    fn init(&self) -> Result<(), DefaultError>;
    fn commit(&self, note: &Note, message: &str) -> Result<(), DefaultError>;
    fn has_changed(&self, note: &Note) -> bool;
    fn push(&self) -> Result<(), DefaultError>;
    fn pull(&self) -> Result<(), DefaultError>;
}

pub struct GitImpl<'a> {
    config: &'a Config,
    shell: &'a dyn Shell,
}

impl<'a> GitImpl<'a> {
    pub fn new(shell: &'a dyn Shell, config: &'a Config) -> GitImpl<'a> {
        GitImpl { shell, config }
    }
}

impl<'a> Git for GitImpl<'a> {
    fn init(&self) -> Result<(), DefaultError> {
        self.shell.execute("git init", &self.config.storage_directory)
    }

    fn commit(&self, note: &Note, message: &str) -> Result<(), DefaultError> {
        let path = &note.path.to_str().unwrap();
        self.shell.execute_in_repo(format!("git add '{}'", path).as_str())?;
        self.shell.execute_in_repo(format!("git commit -m '{}' {}", message, path).as_str())
    }

    fn has_changed(&self, note: &Note) -> bool {
        let path = note.path.to_str().unwrap();
        self.shell
            .execute_in_repo(format!("git add {p} && git diff --exit-code HEAD {p} > /dev/null", p = path).as_str())
            .is_err()
    }

    fn push(&self) -> Result<(), DefaultError> {
        self.shell.execute_in_repo("git push")
    }

    fn pull(&self) -> Result<(), DefaultError> {
        self.shell.execute_in_repo("git pull")
    }
}
