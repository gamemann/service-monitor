use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short='c', long="cfg", default_value="./settings.json")]
    pub cfg_path: String,

    #[arg(short='l', long="list", default_value_t = true, help = "List configuration settings")]
    pub list: bool,
}

pub fn parse() -> Args {
    return Args::parse();
}