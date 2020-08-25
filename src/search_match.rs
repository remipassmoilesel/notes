use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct SearchMatch {
    pub id: usize,
    pub score: usize,
    pub path: PathBuf,
    pub title: String,
    pub matched_lines: Vec<MatchedLine>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MatchedLine {
    pub display_number: usize,
    pub line_number: usize,
    pub content: String,
    pub matched: String,
    pub previous: Option<String>,
    pub next: Option<String>,
}
