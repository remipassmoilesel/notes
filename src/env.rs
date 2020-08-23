use std::env;
use std::env::VarError;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Env {
    fn get(&self, key: &str) -> Result<String, VarError>;
}

pub struct EnvImpl;

impl EnvImpl {
    pub fn new() -> EnvImpl {
        EnvImpl {}
    }
}

impl Env for EnvImpl {
    fn get(&self, key: &str) -> Result<String, VarError> {
        env::var(key)
    }
}
