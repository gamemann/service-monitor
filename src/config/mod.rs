mod alert;
mod check;
mod defaults;
mod service;
mod utils;

pub use alert::{Alert, AlertType, DiscordAlert};
pub use check::{Check, CheckType, HttpCheckConfig};
pub use defaults::{
    def_alert_discord_content_basic, def_alert_discord_timeout, def_alert_type,
    def_check_http_method, def_check_http_timeout, def_check_http_url, def_check_type, def_cron,
    def_debug_lvl, def_fails_cnt_to_alert, def_lats_max_track, def_log_dir,
};
pub use service::Service;

use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use anyhow::Result;

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
