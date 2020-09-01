mod integration_tests {
    extern crate notes;

    use uuid::Uuid;

    use notes::console_output::ConsoleOutput;
    use notes::default_error::DefaultError;
    use notes::parse_and_apply_command;
    use notes::test_env::new_sample_repo;

    // TODO: improve tests

    #[test]
    fn bad_command() {
        let config = new_sample_repo();

        let args = fake_args(vec![]);
        let res: Result<ConsoleOutput, DefaultError> = parse_and_apply_command(args, &config);
        assert_eq!(res.unwrap_err().message, "Bad command, try: $ notes help");
    }

    #[test]
    fn help() {
        let config = new_sample_repo();

        let args = fake_args(vec!["help"]);
        let res = parse_and_apply_command(args, &config).unwrap();
        assert_eq!(res.stderr, "");
        assert!(res.stdout.contains("Clean all the brains !"));
        assert!(res.stdout.contains("Usage:"));
    }

    #[test]
    fn help_shortcut() {
        let config = new_sample_repo();

        let args = fake_args(vec!["h"]);
        let res = parse_and_apply_command(args, &config).unwrap();
        assert_eq!(res.stderr, "");
        assert!(res.stdout.contains("Clean all the brains !"));
        assert!(res.stdout.contains("Usage:"));
    }

    #[test]
    fn search() {
        let config = new_sample_repo();

        let args = fake_args(vec!["search", "ab"]);
        let res = parse_and_apply_command(args, &config).unwrap();
        assert!(res.stdout.contains("2 results found"));
        assert!(res.stderr.is_empty());
    }

    #[test]
    fn new_note() {
        let config = new_sample_repo();

        let note_path = format!("test/note-{}.md", Uuid::new_v4().to_string());
        let args = fake_args(vec!["new", note_path.as_str()]);
        let res = parse_and_apply_command(args, &config).unwrap();
        assert!(res.stdout.contains(&format!("Note '{}' created", note_path)));
        assert!(res.stderr.is_empty());
    }

    #[test]
    fn new_already_exists() {
        let config = new_sample_repo();

        let args = fake_args(vec!["new", "a.md"]);
        let res = parse_and_apply_command(args, &config).unwrap_err();
        assert!(res.message.contains("Already exists:"));
    }

    #[test]
    fn list() {
        let config = new_sample_repo();

        let args = fake_args(vec!["list"]);
        let res = parse_and_apply_command(args, &config);
        assert!(res.is_ok());
    }

    #[test]
    fn edit() {
        let config = new_sample_repo();

        let args = fake_args(vec!["edit", "2"]);
        let res = parse_and_apply_command(args, &config);
        assert!(res.is_ok());
    }

    #[test]
    fn edit_non_existing_note() {
        let config = new_sample_repo();

        let args = fake_args(vec!["edit", "999"]);
        let res = parse_and_apply_command(args, &config);
        assert_eq!(res.unwrap_err().message, "Note with id 999 not found.");
    }

    #[test]
    fn delete() {
        let config = new_sample_repo();

        let args = fake_args(vec!["delete", "2"]);
        let res = parse_and_apply_command(args, &config).unwrap();
        assert!(res.stdout.contains("sample-repo/a.md' deleted"));
        assert!(res.stderr.is_empty());
    }

    #[test]
    fn delete_non_existing_note() {
        let config = new_sample_repo();

        let args = fake_args(vec!["delete", "999"]);
        let res = parse_and_apply_command(args, &config);
        assert_eq!(res.unwrap_err().message, "Note with id 999 not found.");
    }

    fn fake_args(args: Vec<&str>) -> Vec<String> {
        let mut res = vec!["/intergation-test/note".to_string()];
        args.iter().for_each(|a| res.push(String::from(*a)));
        res
    }
}
