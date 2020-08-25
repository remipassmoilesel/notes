use colored::*;
#[cfg(test)]
use mockall::automock;

use crate::note::Note;
use crate::search_match::{MatchedLine, SearchMatch};

#[cfg_attr(test, automock)]
pub trait CliFormat {
    fn search_match(&self, search_m: &SearchMatch) -> String;
    fn note_list_item(&self, note: &Note) -> String;
    fn match_score(&self, score: &usize) -> String;
    fn note_id(&self, id: &usize) -> String;
    fn note_title(&self, title: &String) -> String;
    fn note_directory(&self, name: &String) -> String;
}

pub struct CliFormatImpl;

impl CliFormatImpl {
    pub fn new() -> CliFormatImpl {
        CliFormatImpl {}
    }
}

impl CliFormat for CliFormatImpl {
    fn search_match(&self, search_m: &SearchMatch) -> String {
        let id = self.note_id(&search_m.id);
        let title = self.note_title(&search_m.title);
        let score = self.match_score(&search_m.score);
        let header = format!("{} {} {} \n", id, title, score);

        let mut body: Vec<String> = search_m
            .matched_lines
            .iter()
            .map(|raw_line: &MatchedLine| {
                let match_highlighted = raw_line.matched.yellow().to_string();
                let line_nbr = format!("{}.", raw_line.display_number).dimmed();
                let previous_nbr = format!("{}.", raw_line.display_number - 1).dimmed();
                let next_nbr = format!("{}.", raw_line.display_number + 1).dimmed();

                let previous = &raw_line
                    .previous
                    .as_ref()
                    .map(|l| format!("{} {}\n", previous_nbr, l.dimmed()))
                    .unwrap_or("".to_string());
                let next = &raw_line
                    .next
                    .as_ref()
                    .map(|l| format!("\n{} {}", next_nbr, l.dimmed()))
                    .unwrap_or("".to_string());

                let line = format!("{:2} {}", line_nbr, raw_line.content.replace(&raw_line.matched, &match_highlighted));
                format!("{}{}{}", previous, line, next)
            })
            .collect();

        // If no match was provided, this is because note is empty, otherwise we have the first lines of note
        if body.len() < 1 {
            body = vec!["... This note is empty ...".to_string()]
        }

        format!("{}{}\n", header, body.join("\n"))
    }

    fn note_list_item(&self, note: &Note) -> String {
        format!(" {} - {}", self.note_id(&note.id), self.note_title(&note.title))
    }

    fn match_score(&self, score: &usize) -> String {
        format!("(Score: {})", score.to_string()).dimmed().to_string()
    }

    fn note_id(&self, id: &usize) -> String {
        format!("@{}", id).green().to_string()
    }

    fn note_title(&self, title: &String) -> String {
        format!("{}", title.cyan())
    }

    fn note_directory(&self, name: &String) -> String {
        format!(" 🗁  {}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        // We disable colors for test
        control::set_override(false);
    }

    #[test]
    pub fn search_match_no_previous_no_next() -> () {
        init();
        let search_m = SearchMatch {
            id: 0,
            score: 4,
            path: "/tmp/note-1.txt".into(),
            title: "# What a note !".to_string(),
            matched_lines: vec![
                MatchedLine {
                    display_number: 3,
                    line_number: 2,
                    content: "A very interesting one".to_string(),
                    matched: "".to_string(),
                    previous: None,
                    next: None,
                },
                MatchedLine {
                    display_number: 4,
                    line_number: 3,
                    content: "With very interesting things inside".to_string(),
                    matched: "".to_string(),
                    previous: None,
                    next: None,
                },
            ],
        };

        let fmt = CliFormatImpl::new();
        let actual = fmt.search_match(&search_m);
        let expected = "@0 # What a note ! (Score: 4) \n3. A very interesting one\n4. With very interesting things inside\n".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn search_match_previous_no_next() -> () {
        init();
        let search_m = SearchMatch {
            id: 0,
            score: 4,
            path: "/tmp/note-1.txt".into(),
            title: "# What a note !".to_string(),
            matched_lines: vec![
                MatchedLine {
                    display_number: 3,
                    line_number: 2,
                    content: "A very interesting one".to_string(),
                    matched: "".to_string(),
                    previous: Some("Previous line 1".to_string()),
                    next: None,
                },
                MatchedLine {
                    display_number: 4,
                    line_number: 3,
                    content: "With very interesting things inside".to_string(),
                    matched: "".to_string(),
                    previous: Some("Previous line 2".to_string()),
                    next: None,
                },
            ],
        };

        let fmt = CliFormatImpl::new();
        let actual = fmt.search_match(&search_m);
        let expected =
            "@0 # What a note ! (Score: 4) \n2. Previous line 1\n3. A very interesting one\n3. Previous line 2\n4. With very interesting things inside\n"
                .to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn search_match_no_previous_next() -> () {
        init();
        let search_m = SearchMatch {
            id: 0,
            score: 4,
            path: "/tmp/note-1.txt".into(),
            title: "# What a note !".to_string(),
            matched_lines: vec![
                MatchedLine {
                    display_number: 3,
                    line_number: 2,
                    content: "A very interesting one".to_string(),
                    matched: "".to_string(),
                    previous: None,
                    next: Some("Next line 1".to_string()),
                },
                MatchedLine {
                    display_number: 4,
                    line_number: 3,
                    content: "With very interesting things inside".to_string(),
                    matched: "".to_string(),
                    previous: None,
                    next: Some("Next line 2".to_string()),
                },
            ],
        };

        let fmt = CliFormatImpl::new();
        let actual = fmt.search_match(&search_m);
        let expected =
            "@0 # What a note ! (Score: 4) \n3. A very interesting one\n4. Next line 1\n4. With very interesting things inside\n5. Next line 2\n".to_string();

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn search_match_previous_next() -> () {
        init();
        let search_m = SearchMatch {
            id: 0,
            score: 4,
            path: "/tmp/note-1.txt".into(),
            title: "# What a note !".to_string(),
            matched_lines: vec![
                MatchedLine {
                    display_number: 3,
                    line_number: 2,
                    content: "A very interesting one".to_string(),
                    matched: "".to_string(),
                    previous: Some("Previous line 1".to_string()),
                    next: Some("Next line 1".to_string()),
                },
                MatchedLine {
                    display_number: 4,
                    line_number: 3,
                    content: "With very interesting things inside".to_string(),
                    matched: "".to_string(),
                    previous: Some("Previous line 2".to_string()),
                    next: Some("Next line 2".to_string()),
                },
            ],
        };

        let fmt = CliFormatImpl::new();
        let actual = fmt.search_match(&search_m);
        let expected = "@0 # What a note ! (Score: 4) \n2. Previous line 1\n3. A very interesting one\n4. Next line 1\n3. Previous line 2\n4. With very interesting things inside\n5. Next line 2\n".to_string();

        assert_eq!(actual, expected);
    }
}
