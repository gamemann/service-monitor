use serde::Deserialize;

use std::fmt::Display;

use std::collections::HashMap;

use crate::config::{
    def_check_http_method, def_check_http_timeout, def_check_http_url, def_check_type, def_cron,
};

#[derive(Deserialize, Debug, Clone)]
pub struct HttpCheckConfig {
    #[serde(default = "def_check_http_timeout")]
    pub timeout: u32,

    #[serde(default = "def_check_http_method")]
    pub method: String,

    #[serde(default = "def_check_http_url")]
    pub url: String,

    pub headers: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum CheckType {
    #[serde(rename = "http")]
    HTTP,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Check {
    #[serde(default = "def_cron")]
    pub cron: String,

    #[serde(rename = "type", default = "def_check_type")]
    pub check_type: CheckType,

    pub http: Option<HttpCheckConfig>,
}

impl Display for CheckType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_f = match self {
            CheckType::HTTP => "HTTP",
        };

        write!(f, "{:?}", str_f)
    }
}
