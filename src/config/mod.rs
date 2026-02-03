mod alert;
mod check;
mod service;
mod utils;

pub use alert::{Alert, AlertType, DiscordAlert};
pub use check::{Check, CheckType, HttpCheckConfig};
pub use service::Service;
pub use utils::print;

use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub debug_lvl: String,
    pub log_dir: Option<String>,

    pub services: Vec<Service>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            debug_lvl: "info".into(),
            log_dir: String::from("logs/").into(),
            services: Vec::new(),
        }
    }

    pub fn load(&mut self, file_path: &str) -> Result<Self, serde_json::Error> {
        let mut file = File::open(file_path).expect("Unable to open config file");

        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Unable to read contents of file");

        let new_cfg: Config = serde_json::from_str(&contents).expect("Unable to parse config file");

        let new_self = std::mem::replace(self, new_cfg);

        Ok(new_self)
    }
}
