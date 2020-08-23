
# Notes

A simple tool for taking notes. Work in progress.   

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

Install Rust nightly:

    $ rustup install nightly
    $ rustup default nightly


Build:

    $ cargo build --release

