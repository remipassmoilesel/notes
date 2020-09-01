use colored::*;

pub struct Banners;

impl Banners {
    pub fn small() -> String {
        "\nNotes 🚀\n".green().to_string()
    }

    pub fn big() -> String {
        "
███╗   ██╗ ██████╗ ████████╗███████╗███████╗
████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝
██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗
██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║
██║ ╚████║╚██████╔╝   ██║   ███████╗███████║
╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝

    Clean all the brains !
"
        .green()
        .to_string()
    }
}
