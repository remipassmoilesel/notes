extern crate colored;

use colored::*;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Logger {
    fn info(&self, data: &str);
    fn warn(&self, data: &str);
    fn dimmed(&self, data: &str);
    fn log(&self, data: &str);
    fn error(&self, data: &str);
}

pub struct LoggerImpl;

impl LoggerImpl {
    pub fn new() -> LoggerImpl {
        LoggerImpl {}
    }
}

impl Logger for LoggerImpl {
    fn info(&self, data: &str) {
        self.log(&format!("{}\n", data.cyan()));
    }

    fn warn(&self, data: &str) {
        self.log(&format!("{}\n", data.yellow()));
    }

    fn dimmed(&self, data: &str) {
        self.log(&format!("{}\n", data.dimmed()));
    }

    fn log(&self, data: &str) {
        println!("{}", data);
    }

    fn error(&self, data: &str) {
        eprint!("{}\n", data.red());
    }
}
