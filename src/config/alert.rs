use serde::Deserialize;

use std::{collections::HashMap, fmt};

/* Defaults */
// The default HTTP alert URL.
// Should be localhost.
fn def_http_url() -> String {
    "http://127.0.0.1".to_string()
}

// The default HTTP alert timeout.
// This is in seconds.
fn def_http_timeout() -> u64 {
    10
}

// The default HTTP alert insecure flag.
fn def_http_is_insecure() -> bool {
    false
}

// The default HTTP alert method.
// Most popular is GET.
fn def_http_method() -> String {
    "GET".to_string()
}

// The default HTTP alert body file flag.
fn def_body_is_file() -> bool {
    false
}

#[derive(Deserialize, Debug, Clone)]
pub struct HttpAlert {
    #[serde(default = "def_http_method")]
    pub method: String,

    #[serde(default = "def_http_url")]
    pub url: String,

    #[serde(default = "def_http_timeout")]
    pub timeout: u64,

    #[serde(default = "def_http_is_insecure")]
    pub is_insecure: bool,

    pub body: Option<String>,

    #[serde(default = "def_body_is_file")]
    pub body_is_file: bool,

    pub headers: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum AlertType {
    #[serde(rename = "http")]
    HTTP,
}

impl fmt::Display for AlertType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Alert {
    #[serde(rename = "type")]
    pub alert_type: AlertType,

    pub http: Option<HttpAlert>,
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
