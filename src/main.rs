#![feature(backtrace)]

use ::std::process;

use crate::banners::Banners;
use crate::cli_format::CliFormatImpl;
use crate::command_handler::CommandHandler;
use crate::command_parser::CommandParser;
use crate::config::Config;
use crate::default_error::DefaultError;
use crate::env::EnvImpl;
use crate::git::GitImpl;
use crate::logger::{Logger, LoggerImpl};
use crate::repository::{Repository, RepositoryImpl};
use crate::shell::ShellImpl;

mod banners;
mod cli_format;
mod command_handler;
mod command_parser;
mod config;
mod default_error;
mod env;
mod git;
mod logger;
mod note;
mod repository;
mod search_match;
mod shell;
mod usage;

pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const PKG_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
pub const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let logger = LoggerImpl::new();
    let config = Config::new(&EnvImpl::new());

    logger.log(&Banners::small());

    let result = parse_and_apply_command(&config, &logger);
    if result.is_err() {
        terminate(&logger, result.unwrap_err())
    }
}

fn parse_and_apply_command(config: &Config, logger: &dyn Logger) -> Result<(), DefaultError> {
    let args: Vec<String> = std::env::args().collect();
    let command = CommandParser::new().parse_arguments(args)?;

    let shell = ShellImpl::new(config);
    let git = GitImpl::new(&shell, config);
    let repository = RepositoryImpl::new(config, &shell, &git);

    &repository.init()?;

    CommandHandler::new(&repository, logger, &CliFormatImpl::new()).apply_command(command)
}

fn terminate(logger: &dyn Logger, error: DefaultError) {
    logger.error(format!("{}", error).as_str());
    logger.error(format!("{}", error.backtrace.unwrap_or("".to_string())).as_str());
    process::exit(1);
}
