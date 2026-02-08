use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'c', long = "cfg", default_value = "./settings.json")]
    pub cfg_path: String,

    #[arg(
        short = 'i',
        long = "input",
        default_value_t = false,
        help = "Uses input mode which allows user to input commands."
    )]
    pub input: bool,

    #[arg(
        short = 'l',
        long = "list",
        default_value_t = false,
        help = "List configuration settings"
    )]
    pub list: bool,
}
