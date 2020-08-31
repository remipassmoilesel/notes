extern crate uuid;

use crate::config::Config;
use crate::shell::shell_command;
use std::env;
use std::path::PathBuf;
use uuid::Uuid;

pub fn new_test_root() -> PathBuf {
    let test_root = format!("/tmp/note-test-{}", Uuid::new_v4());
    let cwd = env::current_dir().unwrap();
    shell_command(format!("mkdir -p {}", test_root).as_str(), &cwd).unwrap();
    PathBuf::from(test_root)
}

pub fn new_sample_repo() -> Config {
    let test_root = new_test_root();
    let repo_root = PathBuf::from(format!("{}/sample-repo", test_root.to_str().unwrap()));
    let cwd = env::current_dir().unwrap();
    shell_command(
        format!("tar -xf tests/assets/sample-repo.tar -C {}", test_root.to_str().unwrap()).as_str(),
        &cwd,
    )
    .unwrap();
    shell_command("git config user.email 'test@notes.com' && git config user.name 'Test notes'", &repo_root).unwrap();
    Config::from_path(&repo_root)
}
