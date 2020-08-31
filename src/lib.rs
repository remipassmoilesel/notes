#![feature(backtrace)]

use std::path::PathBuf;

use crate::cli_format::CliFormatImpl;
use crate::command_handler::CommandHandler;
use crate::command_parser::CommandParser;
use crate::config::Config;
use crate::console_output::ConsoleOutput;
use crate::default_error::DefaultError;
use crate::git::GitImpl;
use crate::repository::{Repository, RepositoryImpl};
use crate::shell::{shell_command, ShellImpl};

pub mod config;
pub mod console_output;
pub mod default_error;
pub mod logger;
#[doc(hidden)]
pub mod test_env;

mod banners;
mod cli_format;
mod command_handler;
mod command_parser;
mod env;
mod git;
mod note;
mod repository;
mod search_match;
mod shell;
mod usage;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn parse_and_apply_command(args: Vec<String>, config: &Config) -> Result<ConsoleOutput, DefaultError> {
    check_prerequisites()?;
    let command = CommandParser::new().parse_arguments(args)?;

    let shell = ShellImpl::new(config);
    let git = GitImpl::new(&shell);
    let format = CliFormatImpl::new();
    let repository = RepositoryImpl::new(config, &shell, &git);
    let handler = CommandHandler::new(&repository, &format);

    repository.init()?;
    handler.apply_command(command)
}

fn check_prerequisites() -> Result<(), DefaultError> {
    assert_exists("sh", "sh must be installed and in path variable")?;
    assert_exists("git", "Git must be installed and in path variable")?;
    assert_exists("$EDITOR", "$EDITOR variable must be set in your shell")
}

fn assert_exists(command: &str, message: &str) -> Result<(), DefaultError> {
    let cmd = shell_command(format!("which {}", command).as_str(), &PathBuf::from("/"));
    match cmd {
        Ok(o) if o.status == 0 => Ok(()),
        Ok(o) if o.status != 0 => Err(DefaultError::new(message.to_string())),
        _ => Err(DefaultError::new(message.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn assert_exists_should_work() {
        assert_exists("ls", "error-message").unwrap();
    }

    #[test]
    pub fn assert_exists_should_fail() {
        let res = assert_exists("non-existing-command", "error-message").unwrap_err();
        assert_eq!(res.message, "error-message");
    }
}
