use serde::Deserialize;

use std::fmt::Display;

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct HttpCheckConfig {
    pub timeout: u32,

    pub method: String,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum CheckType {
    #[serde(rename = "http")]
    HTTP,
}

#[derive(Deserialize, Debug)]
pub struct Check {
    pub name: Option<String>,
    pub cron: String,

    #[serde(rename = "type")]
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
