#![feature(backtrace)]

use notes::config::Config;
use notes::logger::{Logger, LoggerImpl};
use notes::parse_and_apply_command;
use std::process;

mod logger;

fn main() {
    let logger = LoggerImpl::default();
    let config = Config::default();
    let args: Vec<String> = std::env::args().collect();

    match parse_and_apply_command(args, &config) {
        Ok(output) => {
            if !output.stdout.is_empty() {
                logger.stdout(&output.stdout)
            }
            if !output.stderr.is_empty() {
                logger.stderr(&output.stderr)
            }
        }
        Err(error) => {
            logger.error(format!("{}", error).as_str());
            logger.error(error.backtrace.unwrap_or_else(|| "".to_string()).as_str());
            process::exit(1);
        }
    }
}
