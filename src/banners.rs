use colored::*;

pub struct Banners;

impl Banners {
    pub fn small() -> String {
        SMALL_BANNER.green().to_string()
    }
    pub fn big() -> String {
        BANNER.green().to_string()
    }
}

const SMALL_BANNER: &str = "\nNotes 🚀\n\n";

const BANNER: &str = "
███╗   ██╗ ██████╗ ████████╗███████╗███████╗
████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝
██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗
██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║
██║ ╚████║╚██████╔╝   ██║   ███████╗███████║
╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝

    Clean all the brains !
";
