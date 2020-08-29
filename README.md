
# Notes

<a href="https://gitlab.com/remipassmoilesel/notes/pipelines" style="display: flex; align-items: center;">
  <img src="https://gitlab.com/remipassmoilesel/notes/badges/master/pipeline.svg" alt="Pipeline status">
  <img src="https://gitlab.com/remipassmoilesel/notes/badges/master/coverage.svg" alt="coverage report"/>
</a>


A simple tool for taking notes. `notes` helps you to keep your notes in a clean directory structure, with Git as versioning.     

Work in progress.     

Prerequisites:
- Git
- $EDITOR variable set with the path of your favorite editor

Main repository is hosted on Gitlab: [https://gitlab.com/remipassmoilesel/notes.git](https://gitlab.com/remipassmoilesel/notes.git)


## Usage

    $ notes help

    Notes 🚀 🚀 🚀
    
    ███╗   ██╗ ██████╗ ████████╗███████╗███████╗
    ████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝
    ██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗
    ██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║
    ██║ ╚████║╚██████╔╝   ██║   ███████╗███████║
    ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝
    
        Clean all the brains !
    
    Usage:
    
      notes new <title>         Create a new note.
      notes n <title>             -> All commands have a short alias
      notes search <needle>     Search for a note. You can use regex !
      notes s <needle>
      notes edit <id>           Edit specified note
      notes e <id>
      notes delete <id>         Delete specified note
      notes d <id>
      notes list                List all notes
      notes l
      notes push                Push notes repository (Git based)
      notes p
      notes pull                Pull notes repository (Git based)
      notes ll
      notes help                Show this help
      notes h
    
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
    

## Build

Install Rust nightly and tools:

    $ rustup install nightly
    $ rustup default nightly
    $ cargo install cargo-tarpaulin


Build:

    $ cargo build --release


Unit testing:

    $ cargo test --lib
    

Integration tests need setup:

    $ ./_test.sh
