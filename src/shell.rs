use std::path::PathBuf;
use std::process::Command;
use std::str;

#[cfg(test)]
use mockall::automock;

use crate::config::Config;
use crate::default_error::DefaultError;

#[derive(Debug)]
pub struct CommandOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl CommandOutput {
    pub fn new(code: i32, stdout: String, stderr: String) -> Self {
        CommandOutput { status: code, stdout, stderr }
    }
}

impl Default for CommandOutput {
    fn default() -> Self {
        CommandOutput::new(0, "".to_string(), "".to_string())
    }
}

#[cfg_attr(test, automock)]
pub trait Shell {
    /// Execute specified command in user shell, and capture outputs
    /// If command succeed, return a CommandOutput
    /// If command fail, return an error
    /// If command cannot be run, return an error
    fn execute(&self, command: &str, current_dir: &PathBuf) -> Result<CommandOutput, DefaultError>;

    /// Execute specified command in user shell, and capture outputs
    /// If command succeed, return a CommandOutput
    /// If command fail, return an error
    /// If command cannot be run, return an error
    fn execute_in_repo(&self, command: &str) -> Result<CommandOutput, DefaultError>;

    /// Execute specified command in user shell, and capture outputs
    /// If command succeed, return a CommandOutput
    /// If command fail, return an error
    /// If command cannot be run, return an error
    fn execute_interactive(&self, command: &str, current_dir: &PathBuf) -> Result<CommandOutput, DefaultError>;

    /// Execute specified command in user shell, and capture outputs
    /// If command succeed, return a CommandOutput
    /// If command fail, return an error
    /// If command cannot be run, return an error
    fn execute_interactive_in_repo(&self, command: &str) -> Result<CommandOutput, DefaultError>;
}

#[derive(Clone)]
pub struct ShellImpl<'a> {
    config: &'a Config,
    executor: fn(command: &str, current_dir: &PathBuf) -> Result<CommandOutput, DefaultError>,
    interactive_executor: fn(command: &str, current_dir: &PathBuf) -> Result<CommandOutput, DefaultError>,
}

impl<'a> ShellImpl<'a> {
    pub fn new(config: &'a Config) -> ShellImpl {
        ShellImpl {
            config,
            executor: command,
            interactive_executor: command_interactive,
        }
    }
}

impl<'a> Shell for ShellImpl<'a> {
    fn execute(&self, command: &str, current_dir: &PathBuf) -> Result<CommandOutput, DefaultError> {
        match (self.executor)(command, current_dir) {
            Ok(o) if o.status == 0 => Ok(o),
            Ok(o) if o.status != 0 => Err(DefaultError::new(format!(
                "Command failed: '{}'\nExit code='{}'\nstdout='{}'\nstderr='{}'",
                command, o.status, o.stdout, o.stderr
            ))),
            Ok(_) => Err(DefaultError::new(String::from("Unexpected return value"))),
            Err(e) => Err(e),
        }
    }

    fn execute_in_repo(&self, command: &str) -> Result<CommandOutput, DefaultError> {
        self.execute(command, &self.config.storage_directory)
    }

    fn execute_interactive(&self, command: &str, current_dir: &PathBuf) -> Result<CommandOutput, DefaultError> {
        match (self.interactive_executor)(command, current_dir) {
            Ok(o) if o.status == 0 => Ok(o),
            Ok(o) if o.status != 0 => Err(DefaultError::new(format!("Command failed: '{}'\nExit code='{}'\n", command, o.status))),
            Ok(_) => Err(DefaultError::new(String::from("Unexpected return value"))),
            Err(e) => Err(e),
        }
    }

    fn execute_interactive_in_repo(&self, command: &str) -> Result<CommandOutput, DefaultError> {
        self.execute_interactive(command, &self.config.storage_directory)
    }
}

/// Execute specified command in user shell, and capture outputs
/// If command succeed, return a CommandOutput
/// If command fail, return a CommandOutput
/// If command cannot be run, return an error
pub fn command(command: &str, current_dir: &PathBuf) -> Result<CommandOutput, DefaultError> {
    let mut s_comm = Command::new("sh");
    s_comm.args(&["-c", command]);
    s_comm.current_dir(current_dir);

    // println!("{}", command);

    if let Ok(out) = s_comm.output() {
        let stderr = str::from_utf8(&out.stderr[..]).unwrap_or_else(|_| "Bad stderr");
        let stdout = str::from_utf8(&out.stdout[..]).unwrap_or_else(|_| "Bad stdout");
        Ok(CommandOutput::new(
            out.status.code().unwrap_or_else(|| -1),
            String::from(stdout),
            String::from(stderr),
        ))
    } else {
        Err(DefaultError::new(format!("Cannot run command '{}'", command)))
    }
}

/// Execute specified command in user shell
/// If command succeed, return a CommandOutput
/// If command fail, return a CommandOutput
/// If command cannot be run, return an error
pub fn command_interactive(command: &str, current_dir: &PathBuf) -> Result<CommandOutput, DefaultError> {
    let mut s_comm = Command::new("sh");
    s_comm.args(&["-c", command]);
    s_comm.current_dir(current_dir);

    // println!("{}", command);

    if let Ok(out) = s_comm.status() {
        Ok(CommandOutput::new(out.code().unwrap_or_else(|| -1), "".to_string(), "".to_string()))
    } else {
        Err(DefaultError::new(format!("Cannot run command '{}'", command)))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    pub fn new_command_output() {
        let res = CommandOutput::new(5, "out".to_string(), "err".to_string());
        assert_eq!(res.status, 5);
        assert_eq!(res.stdout, "out");
        assert_eq!(res.stderr, "err");
    }

    #[test]
    pub fn correct_command() {
        let out = command("ls", &PathBuf::from("/")).unwrap();
        assert_eq!(out.status, 0);
        assert!(!out.stdout.is_empty());
        assert!(out.stderr.is_empty());
    }

    #[test]
    pub fn incorrect_command() {
        let out = command("aaaaa", &PathBuf::from("/")).unwrap();
        assert_ne!(out.status, 0);
        assert!(out.stdout.is_empty());
        assert!(!out.stderr.is_empty());
    }

    #[test]
    pub fn interactive_correct_command() {
        let out = command_interactive("ls", &PathBuf::from("/")).unwrap();
        assert_eq!(out.status, 0);
        assert!(out.stdout.is_empty());
        assert!(out.stderr.is_empty());
    }

    #[test]
    pub fn interactive_incorrect_command() {
        let out = command_interactive("aaaaa", &PathBuf::from("/")).unwrap();
        assert_ne!(out.status, 0);
        assert!(out.stdout.is_empty());
        assert!(out.stderr.is_empty());
    }

    #[test]
    pub fn shell_impl_execute_correct_command() {
        let config = Config {
            storage_directory: PathBuf::from("/storage"),
            template_path: PathBuf::from("/template.md"),
        };
        fn executor(c: &str, p: &PathBuf) -> Result<CommandOutput, DefaultError> {
            assert_eq!(p, &PathBuf::from("/storage"));
            assert_eq!(c, "test-command");
            Ok(CommandOutput {
                status: 0,
                stderr: "err".to_string(),
                stdout: "out".to_string(),
            })
        }
        let shell = ShellImpl {
            config: &config,
            executor,
            interactive_executor: command_interactive,
        };

        let res = shell.execute_in_repo("test-command").unwrap();
        assert_eq!(res.status, 0);
        assert_eq!(res.stderr, "err");
        assert_eq!(res.stdout, "out");
    }

    #[test]
    pub fn shell_impl_execute_bad_command() {
        let config = Config {
            storage_directory: PathBuf::from("/storage"),
            template_path: PathBuf::from("/template.md"),
        };
        fn executor(c: &str, p: &PathBuf) -> Result<CommandOutput, DefaultError> {
            assert_eq!(p, &PathBuf::from("/storage"));
            assert_eq!(c, "test-command");
            Ok(CommandOutput {
                status: 1,
                stderr: "err".to_string(),
                stdout: "out".to_string(),
            })
        }
        let shell = ShellImpl {
            config: &config,
            executor,
            interactive_executor: command_interactive,
        };

        let res = shell.execute_in_repo("test-command").unwrap_err();
        assert_eq!(res.message, "Command failed: \'test-command\'\nExit code=\'1\'\nstdout=\'out\'\nstderr=\'err\'");
    }

    #[test]
    pub fn shell_impl_execute_error() {
        let config = Config {
            storage_directory: PathBuf::from("/storage"),
            template_path: PathBuf::from("/template.md"),
        };
        fn executor(c: &str, p: &PathBuf) -> Result<CommandOutput, DefaultError> {
            assert_eq!(p, &PathBuf::from("/storage"));
            assert_eq!(c, "test-command");
            Err(DefaultError::new("test error".to_string()))
        }
        let shell = ShellImpl {
            config: &config,
            executor,
            interactive_executor: command_interactive,
        };

        let res = shell.execute_in_repo("test-command").unwrap_err();
        assert_eq!(res.message, "test error");
    }

    #[test]
    pub fn shell_impl_execute_interactive_correct_command() {
        let config = Config {
            storage_directory: PathBuf::from("/storage"),
            template_path: PathBuf::from("/template.md"),
        };
        fn executor(c: &str, p: &PathBuf) -> Result<CommandOutput, DefaultError> {
            assert_eq!(p, &PathBuf::from("/storage"));
            assert_eq!(c, "test-command");
            Ok(CommandOutput {
                status: 0,
                stderr: "".to_string(),
                stdout: "".to_string(),
            })
        }
        let shell = ShellImpl {
            config: &config,
            executor: command,
            interactive_executor: executor,
        };

        let res = shell.execute_interactive_in_repo("test-command").unwrap();
        assert_eq!(res.status, 0);
        assert!(res.stderr.is_empty());
        assert!(res.stdout.is_empty());
    }

    #[test]
    pub fn shell_impl_execute_interactive_bad_command() {
        let config = Config {
            storage_directory: PathBuf::from("/storage"),
            template_path: PathBuf::from("/template.md"),
        };
        fn executor(c: &str, p: &PathBuf) -> Result<CommandOutput, DefaultError> {
            assert_eq!(p, &PathBuf::from("/storage"));
            assert_eq!(c, "test-command");
            Ok(CommandOutput {
                status: 1,
                stderr: "".to_string(),
                stdout: "".to_string(),
            })
        }
        let shell = ShellImpl {
            config: &config,
            executor: command,
            interactive_executor: executor,
        };

        let res = shell.execute_interactive_in_repo("test-command").unwrap_err();
        assert_eq!(res.message, "Command failed: \'test-command\'\nExit code=\'1\'\n");
    }

    #[test]
    pub fn shell_impl_execute_interactive_error() {
        let config = Config {
            storage_directory: PathBuf::from("/storage"),
            template_path: PathBuf::from("/template.md"),
        };
        fn executor(c: &str, p: &PathBuf) -> Result<CommandOutput, DefaultError> {
            assert_eq!(p, &PathBuf::from("/storage"));
            assert_eq!(c, "test-command");
            Err(DefaultError::new("test error".to_string()))
        }
        let shell = ShellImpl {
            config: &config,
            executor: command,
            interactive_executor: executor,
        };

        let res = shell.execute_interactive_in_repo("test-command").unwrap_err();
        assert_eq!(res.message, "test error");
    }
}
