use std::path::PathBuf;
use std::process::Command;

use crate::config::Config;
use crate::default_error::DefaultError;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Shell {
    fn execute_in_repo(&self, command: String) -> Result<(), DefaultError>;
    fn execute(&self, command: String, current_dir: &PathBuf) -> Result<(), DefaultError>;
}

#[derive(Clone)]
pub struct ShellImpl<'a> {
    config: &'a Config,
}

impl<'a> ShellImpl<'a> {
    pub fn new(config: &'a Config) -> ShellImpl {
        ShellImpl { config }
    }
}

impl<'a> Shell for ShellImpl<'a> {
    fn execute_in_repo(&self, command: String) -> Result<(), DefaultError> {
        self.execute(command, &self.config.storage_directory)
    }

    fn execute(&self, command: String, current_dir: &PathBuf) -> Result<(), DefaultError> {
        shell_command(command, current_dir)
    }
}

pub fn shell_command(command: String, current_dir: &PathBuf) -> Result<(), DefaultError> {
    let mut shell_command = Command::new("sh");
    shell_command.args(&["-c", command.as_str()]);
    shell_command.current_dir(current_dir);

    // println!("{}", command);

    let status_code = shell_command.status()?;
    match status_code.success() {
        true => Ok(()),
        false => Err(DefaultError::new(format!("Exited with code {}", status_code.code().unwrap_or(-1)))),
    }
}
