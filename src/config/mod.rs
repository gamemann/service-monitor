mod alert;
mod alert_file_log;
mod alert_http;
mod check;
mod service;
mod utils;

pub use alert::{Alert, AlertType};
pub use alert_file_log::FileLogAlert;
pub use alert_http::HttpAlert;
pub use check::{Check, CheckType, HttpCheckConfig};
pub use service::Service;

use serde::Deserialize;
use std::fs;

use anyhow::Result;

fn def_debug_lvl() -> Option<String> {
    Some(String::from("info"))
}

fn def_log_dir() -> Option<String> {
    None
}

#[derive(Deserialize, Clone)]
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
        // Read contents from config file and store.
        // Use fs::read_to_string() for simplicity.
        let contents = fs::read_to_string(file_path)?;

        // We must deserialize contents into Config struct.
        let new_cfg: Config = serde_json::from_str(&contents)?;

        // Since we can't reassign self directly, use mem::replace() to update config structure.
        match std::mem::replace(self, new_cfg) {
            _ => Ok(()),
        }
    }
}
