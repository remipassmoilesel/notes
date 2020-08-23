extern crate colored;

use colored::*;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Logger {
    fn info(&self, message: String);
    fn warn(&self, message: String);
    fn dimmed(&self, message: String);
    fn log(&self, message: String);
    fn error(&self, message: String);
}

pub struct LoggerImpl;

impl LoggerImpl {
    pub fn new() -> LoggerImpl {
        LoggerImpl {}
    }
}

impl Logger for LoggerImpl {
    fn info(&self, message: String) {
        self.log(format!("{}\n", String::from(message).blue()));
    }

    fn warn(&self, message: String) {
        self.log(format!("{}\n", String::from(message).yellow()));
    }

    fn dimmed(&self, message: String) {
        self.log(format!("{}\n", String::from(message).dimmed()));
    }

    fn log(&self, message: String) {
        println!("{}", message);
    }

    fn error(&self, message: String) {
        eprint!("{}\n", String::from(message).red());
    }
}
