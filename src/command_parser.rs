extern crate clap;

use clap::{App, Arg};

use crate::command_handler::Command;
use crate::default_error::DefaultError;
use crate::{PKG_AUTHORS, PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

use self::clap::ArgMatches;

pub struct CommandParser;

impl CommandParser {
    pub fn new() -> CommandParser {
        CommandParser
    }

    pub fn parse_arguments(&self, args: Vec<String>) -> Result<Command, DefaultError> {
        let matches = App::new(PKG_NAME)
            .version(PKG_VERSION)
            .author(PKG_AUTHORS)
            .about(PKG_DESCRIPTION)
            .subcommand(
                App::new("new")
                    .alias("n")
                    .about("Create a new note")
                    .arg(Arg::with_name("title").help("The note title in one word")),
            )
            .subcommand(App::new("list").alias("l").about("List all notes from repository"))
            .subcommand(
                App::new("search")
                    .alias("s")
                    .about("Search in all repository")
                    .arg(Arg::with_name("needle").help("The pattern to search. You can use regular expressions")),
            )
            .subcommand(
                App::new("edit")
                    .alias("e")
                    .about("Edit a note with the default editor")
                    .arg(Arg::with_name("id").help("The id of the note to edit")),
            )
            .subcommand(
                App::new("delete")
                    .alias("d")
                    .about("Delete a note from repository")
                    .arg(Arg::with_name("id").help("The id of the note to delete")),
            )
            .subcommand(App::new("pull").alias("ll").about("Pull note repository"))
            .subcommand(App::new("push").alias("p").about("Push note repository"))
            .subcommand(App::new("help").alias("h").about("Show help"))
            .get_matches_from(args);
        self.build_command(matches)
    }

    fn build_command(&self, matches: ArgMatches) -> Result<Command, DefaultError> {
        if let Some(cmd_matches) = matches.subcommand_matches("new") {
            match cmd_matches.value_of("title") {
                Some(title) => return Ok(Command::New { path: title.to_string() }),
                None => return Err(DefaultError::new("You must specify a title".to_string())),
            }
        }
        if let Some(_) = matches.subcommand_matches("list") {
            return Ok(Command::List);
        }
        if let Some(cmd_matches) = matches.subcommand_matches("search") {
            match cmd_matches.value_of("needle") {
                Some(needle) => return Ok(Command::Search { needle: needle.to_string() }),
                None => return Err(DefaultError::new("You must specify something to search".to_string())),
            }
        }
        if let Some(cmd_matches) = matches.subcommand_matches("edit") {
            match cmd_matches.value_of("id") {
                Some(id) => {
                    let numeric_id = id.parse::<usize>()?;
                    return Ok(Command::Edit { id: numeric_id });
                }
                None => return Err(DefaultError::new("You must specify a note id".to_string())),
            }
        }
        if let Some(cmd_matches) = matches.subcommand_matches("delete") {
            match cmd_matches.value_of("id") {
                Some(id) => {
                    let numeric_id = id.parse::<usize>()?;
                    return Ok(Command::Delete { id: numeric_id });
                }
                None => return Err(DefaultError::new("You must specify a note id".to_string())),
            }
        }
        if let Some(_) = matches.subcommand_matches("pull") {
            return Ok(Command::Pull);
        }
        if let Some(_) = matches.subcommand_matches("push") {
            return Ok(Command::Push);
        }
        if let Some(_) = matches.subcommand_matches("help") {
            return Ok(Command::Help);
        }
        return Err(DefaultError::new("Bad command, try: $ notes help".to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match_new() {
        let args: Vec<String> = vec!["notes".to_string(), "new".to_string(), "one-word-title".to_string()];
        let cp = CommandParser::new();
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(
            command,
            Command::New {
                path: "one-word-title".to_string()
            }
        );

        let args: Vec<String> = vec!["notes".to_string(), "n".to_string(), "one-word-title".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(
            command,
            Command::New {
                path: "one-word-title".to_string()
            }
        );
    }

    #[test]
    fn should_match_list() {
        let cp = CommandParser::new();
        let args: Vec<String> = vec!["notes".to_string(), "list".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::List);

        let args: Vec<String> = vec!["notes".to_string(), "l".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::List);
    }

    #[test]
    fn should_match_search() {
        let cp = CommandParser::new();
        let args: Vec<String> = vec!["notes".to_string(), "search".to_string(), "needle".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Search { needle: "needle".to_string() });

        let args: Vec<String> = vec!["notes".to_string(), "s".to_string(), "needle".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Search { needle: "needle".to_string() });
    }

    #[test]
    fn should_match_edit() {
        let cp = CommandParser::new();
        let args: Vec<String> = vec!["notes".to_string(), "edit".to_string(), "111".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Edit { id: 111 });

        let args: Vec<String> = vec!["notes".to_string(), "e".to_string(), "111".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Edit { id: 111 });
    }

    #[test]
    fn should_match_delete() {
        let cp = CommandParser::new();
        let args: Vec<String> = vec!["notes".to_string(), "delete".to_string(), "111".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Delete { id: 111 });

        let args: Vec<String> = vec!["notes".to_string(), "d".to_string(), "111".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Delete { id: 111 });
    }

    #[test]
    fn should_match_pull() {
        let cp = CommandParser::new();
        let args: Vec<String> = vec!["notes".to_string(), "pull".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Pull);

        let args: Vec<String> = vec!["notes".to_string(), "ll".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Pull);
    }

    #[test]
    fn should_match_push() {
        let cp = CommandParser::new();
        let args: Vec<String> = vec!["notes".to_string(), "push".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Push);

        let args: Vec<String> = vec!["notes".to_string(), "p".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Push);
    }

    #[test]
    fn should_match_help() {
        let cp = CommandParser::new();
        let args: Vec<String> = vec!["notes".to_string(), "help".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Help);

        let args: Vec<String> = vec!["notes".to_string(), "h".to_string()];
        let command = cp.parse_arguments(args).unwrap();
        assert_eq!(command, Command::Help);
    }
}
