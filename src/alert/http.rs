use std::fs;
use std::{collections::HashMap, time::Duration};

use anyhow::{Result, anyhow};

use crate::helper::HttpMethod;

#[derive(Debug, Clone)]
pub struct HttpAlert {
    pub method: HttpMethod,
    pub url: String,

    pub timeout: u64,

    pub body: Option<String>,
    pub body_is_file: bool,

    pub headers: Option<HashMap<String, String>>,
    pub is_insecure: bool,
}

impl HttpAlert {
    pub fn new(
        method: HttpMethod,
        url: String,
        timeout: u64,

        body: Option<String>,
        body_is_file: bool,

        headers: Option<HashMap<String, String>>,
        is_insecure: bool,
    ) -> Self {
        Self {
            method,
            url,
            timeout,
            body,
            body_is_file,
            headers,
            is_insecure,
        }
    }

    pub async fn exec(&self) -> Result<()> {
        // Build client.
        let cl = reqwest::Client::builder()
            .danger_accept_invalid_certs(self.is_insecure)
            .danger_accept_invalid_hostnames(self.is_insecure)
            .build()?;

        // Create request based off of method.
        let mut req = match self.method {
            HttpMethod::GET => cl.get(&self.url),
            HttpMethod::POST => cl.post(&self.url),
            HttpMethod::PUT => cl.put(&self.url),
            HttpMethod::DELETE => cl.delete(&self.url),
            HttpMethod::PATCH => cl.patch(&self.url),
        };

        // We need to set a timeout if we have one.
        if self.timeout > 0 {
            req = req.timeout(Duration::from_secs(self.timeout));
        }

        // If we have body payload, parse it.
        let body: Option<String> = match &self.body {
            Some(body) => {
                if self.body_is_file {
                    match fs::read_to_string(body.clone()) {
                        Ok(res) => Some(res),
                        Err(e) => {
                            return Err(anyhow!("Failed to read body from file: {}", e));
                        }
                    };
                }

                Some(body.clone())
            }
            None => None,
        };

        // If we have a valid body, we need to use it!
        if let Some(body) = body {
            req = req.body(body);
        }

        // If we have headers, append them now.
        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                req = req.header(key, value);
            }
        }

        let res = req.send().await;

        match res {
            Ok(res) => {
                let status_code = res.status().as_u16();

                if status_code != 200 {
                    return Err(anyhow!("Request failed with status code: {}", status_code));
                }
            }

            Err(e) => {
                if e.is_timeout() {
                    return Err(anyhow!("Request timed out: {}", e));
                } else if e.is_status() {
                    let status_code = e.status().unwrap().as_u16();

                    return Err(anyhow!("Request failed with status code: {}", status_code));
                } else {
                    return Err(anyhow!("Request failed: {}", e));
                }
            }
        }

        Ok(())
    }
}
