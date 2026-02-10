mod alert;
mod check;
mod service;
mod utils;

pub use alert::{Alert, AlertType, HttpAlert};
pub use check::{Check, CheckType, HttpCheckConfig};
pub use service::Service;

use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use anyhow::Result;

fn def_debug_lvl() -> Option<String> {
    Some(String::from("info"))
}

fn def_log_dir() -> Option<String> {
    None
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "def_debug_lvl")]
    pub debug_lvl: Option<String>,

    #[serde(default = "def_log_dir")]
    pub log_dir: Option<String>,

    pub services: Vec<Service>,
}

impl Config {
    pub fn new() -> Self {
        // These don't indicate the default values.
        // That is done with serde.
        Config {
            debug_lvl: None,
            log_dir: None,
            services: Vec::new(),
        }
    }

    pub fn load(&mut self, file_path: &str) -> Result<()> {
        let mut file: File = File::open(file_path).expect("Unable to open config file");

        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Unable to read contents of file");

        let new_cfg: Config = serde_json::from_str(&contents).expect("Unable to parse config file");

        match std::mem::replace(self, new_cfg) {
            _ => Ok(()),
        }
    }
}
