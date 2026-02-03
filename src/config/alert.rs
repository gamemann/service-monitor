use serde::Deserialize;

use std::fmt::Display;

#[derive(Deserialize, Debug)]
pub struct DiscordAlert {
    pub webhook_url: String,
    pub timeout: u32,

    pub content_basic: String,
    pub content_raw: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum AlertType {
    #[serde(rename = "discord")]
    DISCORD,
}

impl Display for AlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug)]
pub struct Alert {
    #[serde(rename = "type")]
    pub alert_type: AlertType,

    pub discord: Option<DiscordAlert>,
}

impl Display for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
