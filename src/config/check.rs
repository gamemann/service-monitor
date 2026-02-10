use serde::Deserialize;

use std::fmt::Display;

use std::collections::HashMap;

use crate::helper::HTTP_OK_CODES;

/* Defaults */
pub fn def_cron() -> String {
    String::from("0 * * * * *")
}

// The default check type.
// The only check type available is HTTP right now.
fn def_check_type() -> CheckType {
    CheckType::HTTP
}

// The default HTTP URL.
// Should be localhost.
fn def_http_url() -> String {
    String::from("http://127.0.0.1")
}

// The default HTTP method.
// Most popular is GET.
fn def_http_method() -> String {
    String::from("GET")
}

// The default HTTP timeout.
// This is in seconds.
fn def_http_timeout() -> u64 {
    10
}

// The default HTTP alert body file flag.
fn def_body_is_file() -> bool {
    false
}

// The default HTTP insecure flag.
// If enabled, accepts server responses with invalid certs or hostnames.
fn def_http_is_insecure() -> bool {
    false
}

// We'll want to add the most popular success codes by default (200 - 206)
fn def_http_accept_codes() -> Vec<u16> {
    HTTP_OK_CODES.to_vec()
}

#[derive(Deserialize, Debug, Clone)]
pub struct HttpCheckConfig {
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

    #[serde(default = "def_http_accept_codes")]
    pub accept_codes: Vec<u16>,
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
