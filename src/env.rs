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
    pub fn new() -> Self {
        EnvImpl {}
    }
}

impl Default for EnvImpl {
    fn default() -> Self {
        EnvImpl::new()
    }
}

impl Env for EnvImpl {
    fn get(&self, key: &str) -> Result<String, VarError> {
        env::var(key)
    }
}
