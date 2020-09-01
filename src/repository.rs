extern crate walkdir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[cfg(test)]
use mockall::automock;

use self::walkdir::{DirEntry, WalkDir};
use crate::config::Config;
use crate::console_output::ConsoleOutput;
use crate::default_error::DefaultError;
use crate::git::Git;
use crate::note::Note;
use crate::shell::Shell;

#[cfg_attr(test, automock)]
pub trait Repository {
    fn init(&self) -> Result<ConsoleOutput, DefaultError>;
    fn new_note(&self, id: usize, path: &str) -> Result<Note, DefaultError>;
    fn edit_note(&self, note: &Note) -> Result<ConsoleOutput, DefaultError>;
    fn find_note_by_id(&self, id: usize) -> Option<Note>;
    fn load_repository_tree(&self) -> Vec<RepositoryDir>;
    fn load_notes(&self) -> Vec<Note>;
    fn delete_note(&self, note: &Note) -> Result<ConsoleOutput, DefaultError>;
    fn push_repo(&self) -> Result<ConsoleOutput, DefaultError>;
    fn pull_repo(&self) -> Result<ConsoleOutput, DefaultError>;
}

#[derive(Debug)]
pub struct RepositoryDir {
    pub name: String,
    pub path: PathBuf,
    pub notes: Vec<Note>,
    pub level: usize,
}

pub struct RepositoryImpl<'a> {
    config: &'a Config,
    shell: &'a dyn Shell,
    git: &'a dyn Git,
    ignored_dirs: Vec<&'a str>,
}

impl<'a> RepositoryImpl<'a> {
    pub fn new(config: &'a Config, shell: &'a dyn Shell, git: &'a dyn Git) -> RepositoryImpl<'a> {
        let ignored_dirs: Vec<&str> = vec![".git", ".idea"];
        RepositoryImpl {
            config,
            shell,
            git,
            ignored_dirs,
        }
    }
}

impl<'a> Repository for RepositoryImpl<'a> {
    fn init(&self) -> Result<ConsoleOutput, DefaultError> {
        let mut output = ConsoleOutput::empty();
        if !self.config.template_path.exists() {
            fs::create_dir_all(&self.config.storage_directory)?;
            output.append(self.git.init()?);

            let note = Note::from(0, self.config.template_path.clone(), "# Note template\n\nHere we go !\n\n".to_string())?;
            let mut file = File::create(&note.path)?;
            file.write_all(note.content().as_bytes())?;
            output.append(self.git.commit(&note, "Create note template")?);
        }
        Ok(output)
    }

    fn new_note(&self, id: usize, partial_path: &str) -> Result<Note, DefaultError> {
        let path: PathBuf = [self.config.storage_directory.to_str().unwrap(), partial_path].iter().collect();

        if path.exists() {
            return Err(DefaultError::new(format!("Already exists: {}", path.to_str().unwrap())));
        }

        fs::create_dir_all(path.parent().unwrap())?;
        fs::copy(&self.config.template_path, &path)?;

        let content = fs::read_to_string(&path)?;
        let note = Note::from(id, path, content)?;
        Ok(note)
    }

    fn edit_note(&self, note: &Note) -> Result<ConsoleOutput, DefaultError> {
        let mut out = ConsoleOutput::empty();
        let path = note.path.to_str().unwrap();
        self.shell.execute_interactive_in_repo(format!("$EDITOR {}", path).as_str())?;
        let file_has_changed = self.git.has_changed(note);
        if file_has_changed {
            let message = format!("Update note {}", note.path.file_name().unwrap().to_str().unwrap());
            out.append(self.git.commit(&note, message.as_str())?);
        }
        Ok(out)
    }

    fn find_note_by_id(&self, id: usize) -> Option<Note> {
        let notes = self.load_notes();
        notes.get(id - 1).map(|note| (*note).clone())
    }

    /// This method loads all notes sorted so that the ids are correctly displayed in tree view
    fn load_repository_tree(&self) -> Vec<RepositoryDir> {
        let directories = WalkDir::new(&self.config.storage_directory)
            .sort_by(|a, b| a.path().cmp(&b.path()))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let is_directory = e.path().is_dir();
                let not_ignored = self.ignored_dirs.iter().find(|dir| e.path().to_str().unwrap().contains(*dir)).is_none();
                is_directory && not_ignored
            })
            .collect::<Vec<DirEntry>>();

        let mut current_note_id = 0;
        let notes: Vec<(PathBuf, Vec<Note>)> = directories
            .iter()
            .map(|dir| {
                let notes: Vec<Note> = WalkDir::new(dir.path())
                    .max_depth(1)
                    .sort_by(|a, b| a.path().cmp(&b.path()))
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        let not_ignored = self.ignored_dirs.iter().find(|dir| e.path().to_str().unwrap().contains(*dir)).is_none();
                        let is_markdown = e.path().to_str().unwrap().ends_with(".md");
                        not_ignored && is_markdown
                    })
                    .map(|file| {
                        current_note_id += 1;
                        (current_note_id, file)
                    })
                    .filter_map(|(index, entry)| Note::from_file(index, entry.path().to_path_buf()).ok())
                    .collect();
                (dir.path().to_path_buf(), notes)
            })
            .collect();

        let repo_level = self.config.storage_directory.iter().count();
        notes
            .iter()
            .map(|(dir, notes)| {
                let level = dir.iter().count() - repo_level;
                let mut dir_name = String::from(dir.clone().strip_prefix(&self.config.storage_directory).unwrap().to_str().unwrap());

                // If directory does not have a name, it is the top level directory, so we assign full repository path
                if dir_name.is_empty() {
                    dir_name = String::from(self.config.storage_directory.to_str().unwrap());
                }

                RepositoryDir {
                    name: dir_name,
                    path: dir.clone(),
                    notes: notes.to_vec(),
                    level,
                }
            })
            .collect()
    }

    fn load_notes(&self) -> Vec<Note> {
        self.load_repository_tree().iter().flat_map(|dir| dir.notes.to_vec()).collect()
    }

    fn delete_note(&self, note: &Note) -> Result<ConsoleOutput, DefaultError> {
        fs::remove_file(&note.path)?;
        let message = format!("Delete note {}", note.path.to_str().unwrap());
        self.git.commit(&note, message.as_str())
    }

    fn push_repo(&self) -> Result<ConsoleOutput, DefaultError> {
        self.git.push()
    }

    fn pull_repo(&self) -> Result<ConsoleOutput, DefaultError> {
        self.git.pull()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::GitImpl;
    use crate::git::MockGit;
    use crate::shell::ShellImpl;
    use crate::shell::{CommandOutput, MockShell};
    use crate::test_env::{new_sample_repo, new_test_root};
    use mockall::predicate::*;

    #[test]
    pub fn init() {
        let test_root = new_test_root();
        let repo_path = PathBuf::from(format!("{}/test-a/test-b", test_root.to_str().unwrap()));
        let config = Config {
            storage_directory: repo_path.clone(),
            template_path: PathBuf::from(format!("{}/.template.md", &repo_path.to_str().unwrap())),
        };
        let shell = ShellImpl::new(&config);

        let mut git_mock = MockGit::new();
        git_mock.expect_init().times(1).returning(|| Ok(ConsoleOutput::empty()));
        git_mock
            .expect_commit()
            .times(1)
            .withf(|n, msg| n.title.contains("Note template") && msg.contains("Create note template"))
            .returning(|_, _| Ok(ConsoleOutput::empty()));

        let repository = RepositoryImpl::new(&config, &shell, &git_mock);

        let result = repository.init();
        assert!(result.is_ok());

        let template_content = fs::read_to_string(config.template_path).unwrap();
        assert!(template_content.contains("Note template"));
    }

    #[test]
    pub fn new_note() {
        let config = new_sample_repo();
        let shell = ShellImpl::new(&config);
        let git = GitImpl::new(&shell);
        let repository = RepositoryImpl::new(&config, &shell, &git);

        let partial_path = "test-a/test-b/-test-c/test.md";
        let result = repository.new_note(99, partial_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "# Note template".to_string());
    }

    #[test]
    pub fn new_note_should_fail_if_path_exists() {
        let config = new_sample_repo();
        let shell = ShellImpl::new(&config);
        let git = GitImpl::new(&shell);
        let repository = RepositoryImpl::new(&config, &shell, &git);

        let partial_path = "test-a/test-b/-test-c/test.md";
        let result = repository.new_note(99, partial_path);
        assert!(result.is_ok());

        let result = repository.new_note(99, partial_path);
        assert!(result.is_err());
        assert!(result.err().unwrap().message.contains("Already exists"));
    }

    #[test]
    pub fn edit_note() {
        let fake_note = Note {
            id: 0,
            title: "Fake note".into(),
            path: "/tmp/fake-note.md".into(),
            raw: vec![],
            body: vec![],
        };

        let config = Config {
            storage_directory: "/tmp".into(),
            template_path: "/tmp".into(),
        };

        let mut shell_mock = MockShell::new();

        let exp_command = format!("$EDITOR {}", fake_note.path.to_str().unwrap());
        shell_mock
            .expect_execute_interactive_in_repo()
            .times(1)
            .withf(move |c| c == exp_command)
            .returning(|_| Ok(CommandOutput::default()));

        let mut git_mock = MockGit::new();
        git_mock.expect_has_changed().times(1).with(eq(fake_note.clone())).returning(|_| true);

        let exp_message = format!("Update note {}", fake_note.path.file_name().unwrap().to_str().unwrap());
        let exp_note = fake_note.clone();
        git_mock
            .expect_commit()
            .times(1)
            .withf(move |n, m| n == &exp_note && m == exp_message)
            .returning(|_, _| Ok(ConsoleOutput::empty()));

        let repository = RepositoryImpl::new(&config, &shell_mock, &git_mock);

        let result = repository.edit_note(&fake_note);
        assert!(result.is_ok());
    }

    #[test]
    pub fn edit_note_should_not_commit() {
        let fake_note = Note {
            id: 0,
            title: "Fake note".into(),
            path: "/tmp/fake-note.md".into(),
            raw: vec![],
            body: vec![],
        };

        let config = Config {
            storage_directory: "/tmp".into(),
            template_path: "/tmp".into(),
        };

        let mut shell_mock = MockShell::new();

        let exp_command = format!("$EDITOR {}", fake_note.path.to_str().unwrap());
        shell_mock
            .expect_execute_interactive_in_repo()
            .times(1)
            .withf(move |c| c == exp_command)
            .returning(|_| Ok(CommandOutput::default()));

        let mut git_mock = MockGit::new();
        git_mock.expect_has_changed().times(1).with(eq(fake_note.clone())).returning(|_| false);

        let repository = RepositoryImpl::new(&config, &shell_mock, &git_mock);

        let result = repository.edit_note(&fake_note);
        assert!(result.is_ok());
    }

    #[test]
    pub fn find_note_by_id() {
        let config = new_sample_repo();
        let shell = ShellImpl::new(&config);
        let git = GitImpl::new(&shell);
        let repository = RepositoryImpl::new(&config, &shell, &git);

        let result = repository.find_note_by_id(2);
        assert!(result.is_some());
        assert_eq!(result.unwrap().title, "# test/assets/sample-repo/a.md".to_string());
    }

    #[test]
    pub fn load_repository_tree() {
        let config = new_sample_repo();

        let shell = ShellImpl::new(&config);
        let git = GitImpl::new(&shell);
        let repository = RepositoryImpl::new(&config, &shell, &git);

        let result = repository.load_repository_tree();
        let tree: Vec<&str> = result
            .iter()
            .flat_map(|dir| {
                let mut result = vec![&dir.path];
                let note_paths: Vec<&PathBuf> = dir.notes.iter().map(|n| &n.path).collect();
                result.extend(note_paths.iter().cloned());
                result
            })
            .map(|p| p.strip_prefix(&config.storage_directory).unwrap().to_str().unwrap())
            .collect();

        assert_eq!(
            tree,
            vec![
                "", // This is the repository root
                ".template.md",
                "a.md",
                "b.md",
                "a",
                "a/aa.md",
                "a/ab.md",
                "a/a",
                "a/a/aaa.md",
                "a/a/aab.md",
                "b",
                "b/bb.md"
            ]
        );
    }

    #[test]
    pub fn load_notes() {
        let config = new_sample_repo();
        let shell = ShellImpl::new(&config);
        let git = GitImpl::new(&shell);
        let repository = RepositoryImpl::new(&config, &shell, &git);

        let result = repository.load_notes();
        let paths: Vec<&str> = result
            .iter()
            .map(|note| &note.path)
            .map(|p| p.strip_prefix(&config.storage_directory).unwrap().to_str().unwrap())
            .collect();

        assert_eq!(
            paths,
            vec![".template.md", "a.md", "b.md", "a/aa.md", "a/ab.md", "a/a/aaa.md", "a/a/aab.md", "b/bb.md"]
        );
    }
}
