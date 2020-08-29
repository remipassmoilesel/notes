#![feature(backtrace)]

use notes::banners::Banners;
use notes::config::Config;
use notes::logger::{Logger, LoggerImpl};
use notes::parse_and_apply_command;
use std::process;

mod logger;

fn main() {
    let logger = LoggerImpl::default();
    logger.log(&Banners::small());

    let config = Config::default();
    let args: Vec<String> = std::env::args().collect();

    if let Err(error) = parse_and_apply_command(args, &config, &logger) {
        logger.error(format!("{}", error).as_str());
        logger.error(error.backtrace.unwrap_or_else(|| "".to_string()).as_str());
        process::exit(1);
    }
}
