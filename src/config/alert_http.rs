use serde::Deserialize;

use std::collections::HashMap;

use crate::helper::HTTP_OK_CODES;

#[derive(Deserialize, Clone)]
pub struct HttpAlert {
    #[serde(default = "HttpAlert::def_http_method")]
    pub method: String,

    #[serde(default = "HttpAlert::def_http_url")]
    pub url: String,

    #[serde(default = "HttpAlert::def_http_timeout")]
    pub timeout: u64,

    #[serde(default = "HttpAlert::def_http_is_insecure")]
    pub is_insecure: bool,

    pub body: Option<String>,

    #[serde(default = "HttpAlert::def_body_is_file")]
    pub body_is_file: bool,

    #[serde(default = "HttpAlert::def_http_accept_codes")]
    pub accept_codes: Vec<u16>,

    pub headers: Option<HashMap<String, String>>,
}

impl HttpAlert {
    fn def_http_method() -> String {
        "GET".to_string()
    }

    fn def_http_url() -> String {
        "http://127.0.0.1".to_string()
    }

    fn def_http_timeout() -> u64 {
        10
    }

    fn def_http_is_insecure() -> bool {
        false
    }

    fn def_body_is_file() -> bool {
        false
    }

    fn def_http_accept_codes() -> Vec<u16> {
        vec![HTTP_OK_CODES.to_vec()].concat()
    }
}
