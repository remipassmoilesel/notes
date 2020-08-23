extern crate regex;

use std::path::PathBuf;

use regex::{Regex, RegexBuilder};

use crate::default_error::DefaultError;
use colored::*;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn match_score(&self, needle: &String) -> usize {
        let needle_regex = self.build_needle_regex(needle);
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

    pub fn to_search_result(&self, needle: &String, score: usize) -> String {
        let needle_regex = self.build_needle_regex(needle);
        let id = Format::note_id(&self.id);
        let title = Format::note_title(&self.title);
        let formatted_score = Format::score(score);

        let title_position = self.raw.iter().position(|l| &self.title == l).unwrap() + 1;
        let mut display_number = title_position;
        let mut matching_lines: Vec<String> = self
            .raw
            .iter()
            .skip(title_position)
            .map(|line| {
                display_number += 1;
                (line, display_number - 1, display_number)
            })
            .filter_map(|(line, line_id, display_number)| match needle_regex.captures(line) {
                Some(captures) => {
                    let matched = captures.get(1).map_or("", |m| m.as_str());
                    let previous = self.raw.get(line_id - 1);
                    let next = self.raw.get(line_id + 1);
                    Some(Format::search_result(display_number, line, matched, previous, next))
                }
                None => None,
            })
            .collect();

        // Title can match but not content. In this case we display the first lines of note.
        if score > 0 && matching_lines.len() < 1 {
            let first_lines = 6;
            let len = match self.body.len() < first_lines {
                true => self.body.len(),
                false => first_lines,
            };
            matching_lines = self.body[0..len].to_vec();
        }

        format!("\n{} {} {} \n\n{}", id, title, formatted_score, matching_lines.join("\n"))
    }

    pub fn format_for_list(&self) -> String {
        format!(
            " - {} - {} {}",
            Format::note_id(&self.id),
            Format::note_title(&self.title),
            // TODO: use relative path
            Format::note_path(&self.path.to_str().unwrap())
        )
    }

    pub fn format_for_write(&self) -> String {
        self.raw.join("\n")
    }

    fn build_needle_regex(&self, needle: &String) -> Regex {
        RegexBuilder::new(&format!("({})", needle)).case_insensitive(true).build().unwrap()
    }
}

struct Format;

impl Format {
    pub fn search_result(line_number: usize, raw_line: &String, matched: &str, previous_raw: Option<&String>, next_raw: Option<&String>) -> String {
        let highlight = matched.yellow().to_string();
        let line_nbr_formatted = format!("{}.", line_number.to_string()).dimmed();
        let line = format!("{:2} {}", line_nbr_formatted, raw_line.replace(matched, &highlight));

        let spaces: Vec<&str> = line_nbr_formatted.chars().map(|_| " ").collect();
        let previous = previous_raw.map(|l| format!("{} {}", spaces.join(""), l)).unwrap_or(String::from(""));
        let next = next_raw.map(|l| format!("{} {}", spaces.join(""), l)).unwrap_or(String::from(""));
        format!("{}\n{}\n{}\n", previous, line, next)
    }

    pub fn score(score: usize) -> String {
        format!("(Score: {})", score.to_string()).dimmed().to_string()
    }

    pub fn note_id(id: &usize) -> String {
        format!("@{}", id.to_string()).green().to_string()
    }

    pub fn note_title(title: &String) -> String {
        format!("{}", title.cyan())
    }

    pub fn note_path(path: &str) -> String {
        format!("({})", path.dimmed())
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    pub fn from() -> () {
        let note = Note::from(0, PathBuf::from("/tmp/note-1.txt"), SAMPLE_NOTE_1.to_string()).unwrap();
        assert_eq!(note.id, 0);
        assert_eq!(note.title, "# SSH");
        assert_eq!(note.body.len(), 1);
        assert_eq!(note.body[0], "A note about SSH");
        assert_eq!(note.path, PathBuf::from("/tmp/note-1.txt"));
    }

    #[test]
    pub fn match_score() -> () {
        let note = Note::from(0, PathBuf::from("/tmp/note-1.txt"), SAMPLE_NOTE_1.to_string()).unwrap();
        assert_eq!(note.match_score(&"ssh".to_string()), 5);
    }

    #[test]
    pub fn match_score_should_score_0() -> () {
        let note = Note::from(0, PathBuf::from("/tmp/note-1.txt"), SAMPLE_NOTE_1.to_string()).unwrap();
        assert_eq!(note.match_score(&"something-else".to_string()), 0);
    }

    #[test]
    pub fn to_search_result() -> () {
        let note = Note::from(0, PathBuf::from("/tmp/note-1.txt"), SAMPLE_NOTE_2.to_string()).unwrap();
        let actual = note.to_search_result(&"rsync".to_string(), 10);
        // Pipe
        let expected = "\n[32m@0[0m [36m# Rsync[0m [2m(Score: 10)[0m \n\n   A very interesting note\n[2m4.[0m About [33mRsync[0m\n   With very interesting things inside\n";
        assert_eq!(actual, expected);
    }
}
