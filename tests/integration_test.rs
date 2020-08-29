extern crate notes;

use notes::config::Config;
use notes::logger::LoggerImpl;
use notes::parse_and_apply_command;
use std::fs;
use uuid::Uuid;

#[test]
fn bad_command() {
    let logger = LoggerImpl::default();
    let config = Config::default();

    let args = args(vec![]);
    let res = parse_and_apply_command(args, &config, &logger);
    assert_eq!(res.unwrap_err().message, "Bad command, try: $ notes help");
}

#[test]
fn new_note() {
    let logger = LoggerImpl::default();
    let config = Config::default();

    let note_path = format!("test/note-{}.md", Uuid::new_v4().to_string());
    let args = args(vec!["new", note_path.as_str()]);
    let res = parse_and_apply_command(args, &config, &logger);
    assert!(res.is_ok());
    assert_edited(&config, &note_path);
}

#[test]
fn search() {
    let logger = LoggerImpl::default();
    let config = Config::default();

    let args = args(vec!["search", "needle"]);
    let res = parse_and_apply_command(args, &config, &logger);
    assert!(res.is_ok());
}

#[test]
fn list() {
    let logger = LoggerImpl::default();
    let config = Config::default();

    let args = args(vec!["list"]);
    let res = parse_and_apply_command(args, &config, &logger);
    assert!(res.is_ok());
}

fn assert_edited(config: &Config, path: &str) {
    let mut note_path = config.storage_directory.clone();
    note_path.push(path);

    let content = fs::read_to_string(note_path).unwrap();
    assert_eq!(content, "### File was just edited ###\n");
}

fn args(args: Vec<&str>) -> Vec<String> {
    let mut res = vec!["/intergation-test/note".to_string()];
    args.iter().for_each(|a| res.push(String::from(*a)));
    res
}
