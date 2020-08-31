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
    fn stdout(&self, data: &str);
    fn stderr(&self, data: &str);
}

pub struct LoggerImpl;

impl LoggerImpl {
    pub fn new() -> Self {
        LoggerImpl {}
    }
}

impl Default for LoggerImpl {
    fn default() -> Self {
        LoggerImpl::new()
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

    fn stdout(&self, data: &str) {
        print!("{}", data);
    }

    fn stderr(&self, data: &str) {
        eprint!("{}", data);
    }
}
