extern crate dirs;

use crate::env::{Env, EnvImpl};
use std::path::PathBuf;

pub const NOTES_STORAGE_DIRECTORY: &str = "NOTES_STORAGE_DIRECTORY";

pub struct Config {
    pub storage_directory: PathBuf,
    pub template_path: PathBuf,
}

impl<'a> Config {
    pub fn new(env: &'a dyn Env) -> Self {
        let storage_directory = Config::get_storage_path(env);
        Config::from_path(&storage_directory)
    }

    pub fn from_path(path: &PathBuf) -> Self {
        let storage_directory = path.clone();
        let template_path: PathBuf = [storage_directory.to_str().unwrap(), ".template.md"].iter().collect();

        Config {
            storage_directory,
            template_path,
        }
    }

    fn get_storage_path(env: &'a dyn Env) -> PathBuf {
        let env_path = env.get(NOTES_STORAGE_DIRECTORY).map(PathBuf::from);
        let mut alternative = dirs::home_dir().unwrap_or_else(|| "/tmp".into());
        alternative.push(".notes");

        match env_path {
            Ok(repository_path) => repository_path,
            _ => alternative,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(&EnvImpl::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::MockEnv;
    use mockall::predicate::*;
    use std::env::VarError;
    use std::path::PathBuf;

    #[test]
    fn should_return_path_from_env_var() {
        let mut mock_env = MockEnv::new();
        mock_env
            .expect_get()
            .with(eq(NOTES_STORAGE_DIRECTORY))
            .times(1)
            .returning(|_| Ok("/path/to/dir".to_string()));

        let config = Config::new(&mock_env);
        assert_eq!(config.storage_directory, PathBuf::from("/path/to/dir"))
    }

    #[test]
    fn should_return_path_from_home() {
        let mut mock_env = MockEnv::new();
        mock_env
            .expect_get()
            .with(eq(NOTES_STORAGE_DIRECTORY))
            .times(1)
            .returning(|_| Err(VarError::NotPresent));

        let config = Config::new(&mock_env);
        let path_str: String = config.storage_directory.to_str().unwrap().to_string();
        assert!(
            path_str.starts_with("/home") || path_str.starts_with("/root"),
            format!("Path must start with /home actual={}", path_str)
        );
        assert!(path_str.ends_with(".notes"), format!("Path must end with .notes {}", path_str));
    }
}
