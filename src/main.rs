#![feature(backtrace)]
extern crate lazy_static;

use ::std::process;

use crate::banners::Banners;
use crate::command_handler::CommandHandler;
use crate::command_parser::CommandParser;
use crate::config::Config;
use crate::default_error::DefaultError;
use crate::git::GitImpl;
use crate::logger::{Logger, LoggerImpl};
use crate::repository::{Repository, RepositoryImpl};
use crate::shell::ShellImpl;

mod banners;
mod command_handler;
mod command_parser;
mod config;
mod default_error;
mod git;
mod logger;
mod note;
mod repository;
mod shell;
mod usage;

pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let logger_impl = LoggerImpl::new();
    let logger = &logger_impl as &dyn Logger;

    logger.log(Banners::small());
    let config = Config::new();
    let result = parse_and_apply_command(&config, logger);
    if result.is_err() {
        terminate(logger, result.unwrap_err())
    }
}

fn parse_and_apply_command(config: &Config, logger: &dyn Logger) -> Result<(), DefaultError> {
    let shell = ShellImpl::new(config);
    let git = GitImpl::new(&shell, config);
    let repository_impl = RepositoryImpl::new(config, &shell, &git);
    let repository = &repository_impl as &dyn Repository;
    repository.init()?;

    let args: Vec<String> = std::env::args().collect();
    let command = CommandParser::new().parse_arguments(args)?;
    CommandHandler::new(repository, logger).apply_command(command)?;

    Ok(())
}

fn terminate(logger: &dyn Logger, error: DefaultError) {
    logger.error(format!("{}", error));
    logger.error(format!("{}", error.backtrace.unwrap_or("".to_string())));
    process::exit(1);
}
