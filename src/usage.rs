use crate::PKG_VERSION;

pub fn usage() -> String {
    format!(
        "
Usage:

  notes new <path>          Create a new note.
  notes search <needle>     Search for a note. You can use regex !
  notes edit <id>           Edit specified note
  notes delete <id>         Delete specified note
  notes list                List all notes
  notes push                Push notes repository
  notes pull                Pull notes repository
  notes help                Show this help

Options:
  -h --help     Show this screen.
  --version     Show version.

Examples:

    $ notes new my-awesome-idea
    $ notes list
    $ notes edit 123
    $ notes delete 123

With shortcuts:

    $ notes n my-awesome-idea
    $ notes e 123
    $ notes d 123

See https://gitlab.com/remipassmoilesel/notes

Version: {pkg_version}

",
        pkg_version = PKG_VERSION
    )
}
