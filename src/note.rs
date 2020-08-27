extern crate regex;

use std::cmp::min;
use std::fs;
use std::path::PathBuf;

use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

use crate::default_error::DefaultError;
use crate::search_match::{MatchedLine, SearchMatch};

lazy_static! {
    static ref HAS_CONTENT: Regex = RegexBuilder::new("\\w").case_insensitive(true).build().unwrap();
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Note {
    pub id: usize,
    pub path: PathBuf,
    pub title: String,
    /// Contains only non empty lines of note, without title
    pub body: Vec<String>,
    /// Contains all note lines
    pub raw: Vec<String>,
}

impl Note {
    pub fn from(id: usize, path: PathBuf, raw_content: String) -> Result<Note, DefaultError> {
        let all_lines: Vec<String> = raw_content.split("\n").map(|s| String::from(s)).collect();

        let non_empty_lines: Vec<String> = all_lines
            .iter()
            .filter(|l| !String::is_empty(&l.to_string()))
            .map(|l| String::from(l))
            .collect();

        if non_empty_lines.len() < 1 {
            return Err(DefaultError {
                message: "Not enough lines".to_string(),
                backtrace: None,
            });
        }

        let title = non_empty_lines.get(0).unwrap().to_string();
        let body = non_empty_lines.into_iter().skip(1).map(|s| s.to_string()).collect();

        Ok(Note {
            id,
            path,
            title,
            body,
            raw: all_lines,
        })
    }

    pub fn from_file(id: usize, path: PathBuf) -> Result<Note, DefaultError> {
        let content = fs::read_to_string(&path)?;
        Note::from(id, path, content)
    }

    pub fn search_match(&self, needle_regex: &Regex) -> SearchMatch {
        let score = self.match_score(needle_regex);
        let title_position = self.raw.iter().position(|l| &self.title == l).unwrap();

        let mut matching_lines: Vec<MatchedLine> = self
            .raw
            .iter()
            .enumerate()
            .skip(title_position + 1)
            .filter_map(|(idx, line)| match needle_regex.captures(line) {
                Some(captures) => {
                    let matched = String::from(captures.get(1).map_or("", |m| m.as_str()));
                    let previous: Option<String> = match idx > 1 {
                        // Title must not appear in previous line
                        true => self.raw.get(idx - 1).filter(|s| HAS_CONTENT.is_match(s)).map(|s| String::from(s)),
                        false => None,
                    };
                    let next: Option<String> = self.raw.get(idx + 1).filter(|s| HAS_CONTENT.is_match(s)).map(|s| String::from(s));
                    Some(MatchedLine {
                        display_number: idx + 1,
                        line_number: idx,
                        content: String::from(line),
                        matched,
                        previous,
                        next,
                    })
                }
                None => None,
            })
            .collect();

        // Title can match without match in content. In this case we return the first lines of note.
        if score > 0 && matching_lines.len() < 1 {
            let show_lines = 6;
            let first_lines = min(title_position + show_lines, self.raw.len());

            matching_lines = self
                .raw
                .iter()
                .enumerate()
                .skip(title_position + 1)
                .take(first_lines)
                .filter(|(_, line)| HAS_CONTENT.is_match(line))
                .map(|(idx, line)| MatchedLine {
                    display_number: idx + 1,
                    line_number: idx,
                    content: String::from(line),
                    matched: "".to_string(),
                    previous: None,
                    next: None,
                })
                .collect();
        }

        SearchMatch {
            id: self.id,
            score,
            path: self.path.clone(),
            title: self.title.clone(),
            matched_lines: matching_lines,
        }
    }

    fn match_score(&self, needle_regex: &Regex) -> usize {
        let match_in_title = match needle_regex.is_match(&self.title) {
            true => 4,
            false => 0,
        };
        let match_in_body: usize = self
            .body
            .iter()
            .map(|line| match needle_regex.is_match(line) {
                true => 1,
                false => 0,
            })
            .sum();
        match_in_title + match_in_body
    }

    pub fn content(&self) -> String {
        self.raw.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use regex::{Regex, RegexBuilder};

    use super::*;

    const SAMPLE_NOTE_1: &str = "\

# SSH

A note about SSH

";

    const SAMPLE_NOTE_2: &str = "\

# Rsync

A very interesting note
About Rsync
With very interesting things inside

";

    const SAMPLE_NOTE_3: &str = "\

# What a note !

A very interesting one
With very interesting things inside

";

    pub fn needle_regexp(needle: &str) -> Regex {
        RegexBuilder::new(&format!("({})", needle)).case_insensitive(true).build().unwrap()
    }

    #[test]
    pub fn from() -> () {
        let note = Note::from(0, "/tmp/note-1.txt".into(), SAMPLE_NOTE_1.to_string()).unwrap();
        assert_eq!(note.id, 0);
        assert_eq!(note.title, "# SSH");
        assert_eq!(note.body.len(), 1);
        assert_eq!(note.body[0], "A note about SSH");
        assert_eq!(note.path, PathBuf::from("/tmp/note-1.txt"));
    }

    #[test]
    pub fn match_score() -> () {
        let note = Note::from(0, "/tmp/note-1.txt".into(), SAMPLE_NOTE_1.to_string()).unwrap();
        let needle_regex = needle_regexp("ssh");
        assert_eq!(note.match_score(&needle_regex), 5);
    }

    #[test]
    pub fn match_score_should_score_0() -> () {
        let note = Note::from(0, "/tmp/note-1.txt".into(), SAMPLE_NOTE_1.to_string()).unwrap();
        let needle_regex = needle_regexp("something-else");
        assert_eq!(note.match_score(&needle_regex), 0);
    }

    #[test]
    pub fn search_match() -> () {
        let note = Note::from(0, "/tmp/note-1.txt".into(), SAMPLE_NOTE_2.to_string()).unwrap();
        let needle_regex = needle_regexp("rsync");
        let actual = note.search_match(&needle_regex);
        let expected = SearchMatch {
            id: 0,
            score: 5,
            path: "/tmp/note-1.txt".into(),
            title: "# Rsync".into(),
            matched_lines: vec![MatchedLine {
                display_number: 4,
                line_number: 3,
                content: "About Rsync".into(),
                matched: "Rsync".into(),
                previous: Some("A very interesting note".into()),
                next: Some("With very interesting things inside".into()),
            }],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn search_match_only_title() -> () {
        let note = Note::from(0, "/tmp/note-1.txt".into(), SAMPLE_NOTE_3.to_string()).unwrap();
        let needle_regex = needle_regexp("note");
        let actual = note.search_match(&needle_regex);
        let expected = SearchMatch {
            id: 0,
            score: 4,
            path: "/tmp/note-1.txt".into(),
            title: "# What a note !".into(),
            matched_lines: vec![
                MatchedLine {
                    display_number: 3,
                    line_number: 2,
                    content: "A very interesting one".into(),
                    matched: "".to_string(),
                    previous: None,
                    next: None,
                },
                MatchedLine {
                    display_number: 4,
                    line_number: 3,
                    content: "With very interesting things inside".into(),
                    matched: "".to_string(),
                    previous: None,
                    next: None,
                },
            ],
        };
        assert_eq!(actual, expected);
    }
}
