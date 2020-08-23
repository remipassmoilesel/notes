use colored::*;

pub struct Banners;

impl Banners {
    pub fn big() -> String {
        BANNER.green().to_string()
    }

    pub fn small() -> String {
        SMALL_BANNER.green().to_string()
    }
}

const BANNER: &'static str = "
███╗   ██╗ ██████╗ ████████╗███████╗███████╗
████╗  ██║██╔═══██╗╚══██╔══╝██╔════╝██╔════╝
██╔██╗ ██║██║   ██║   ██║   █████╗  ███████╗
██║╚██╗██║██║   ██║   ██║   ██╔══╝  ╚════██║
██║ ╚████║╚██████╔╝   ██║   ███████╗███████║
╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚══════╝╚══════╝

    Clean all the brains !
";

const SMALL_BANNER: &'static str = "\nNotes 🚀\n";
