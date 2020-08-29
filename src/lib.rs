#![feature(backtrace)]

use crate::cli_format::CliFormatImpl;
use crate::command_handler::CommandHandler;
use crate::command_parser::CommandParser;
use crate::config::Config;
use crate::default_error::DefaultError;
use crate::git::GitImpl;
use crate::logger::Logger;
use crate::repository::{Repository, RepositoryImpl};
use crate::shell::ShellImpl;

pub mod banners;
pub mod cli_format;
pub mod command_handler;
pub mod command_parser;
pub mod config;
pub mod default_error;
pub mod env;
pub mod git;
pub mod logger;
pub mod note;
pub mod repository;
pub mod search_match;
pub mod shell;
pub mod usage;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn parse_and_apply_command(args: Vec<String>, config: &Config, logger: &dyn Logger) -> Result<(), DefaultError> {
    let command = CommandParser::new().parse_arguments(args)?;

    let shell = ShellImpl::new(config);
    let git = GitImpl::new(&shell, config);
    let repository = RepositoryImpl::new(config, &shell, &git);

    repository.init()?;

    CommandHandler::new(&repository, logger, &CliFormatImpl::new()).apply_command(command)
}
