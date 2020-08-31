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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_value() {
        let env = EnvImpl::new();
        assert!(env.get("HOME").unwrap().contains('/'))
    }

    #[test]
    fn should_not_return_value() {
        let env = EnvImpl::new();
        assert!(env.get("NON_EXISTING_VAR").is_err())
    }
}
