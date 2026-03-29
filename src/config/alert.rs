use std::fmt;

use serde::Deserialize;

use crate::config::{FileLogAlert, HttpAlert};

#[derive(Deserialize, PartialEq, Clone)]
pub enum AlertType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "file_log")]
    FileLog,
}

impl fmt::Display for AlertType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertType::Http => write!(f, "HTTP"),
            AlertType::FileLog => write!(f, "File Log"),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Alert {
    #[serde(rename = "type")]
    pub alert_type: AlertType,

    pub http: Option<HttpAlert>,
    pub file_log: Option<FileLogAlert>,
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self.alert_type {
            AlertType::Http => write!(f, "HTTP Alert"),
            AlertType::FileLog => write!(f, "File Log Alert"),
        }
    }
}
