use crate::config::Config;
use crate::default_error::DefaultError;
use crate::note::Note;
use crate::shell::Shell;

pub trait Git {
    fn init(&self) -> Result<(), DefaultError>;
    fn commit(&self, note: &Note, message: String) -> Result<(), DefaultError>;
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
        self.shell.execute(format!("git init"), &self.config.storage_directory)
    }

    fn commit(&self, note: &Note, message: String) -> Result<(), DefaultError> {
        let path = &note.path.to_str().unwrap();
        self.shell.execute_in_repo(format!("git add '{}'", path))?;
        self.shell.execute_in_repo(format!("git commit -m '{}' {}", message, path))
    }

    fn push(&self) -> Result<(), DefaultError> {
        self.shell.execute_in_repo(format!("git push"))
    }

    fn pull(&self) -> Result<(), DefaultError> {
        self.shell.execute_in_repo(format!("git pull"))
    }
}
