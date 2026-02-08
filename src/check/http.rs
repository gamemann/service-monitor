use std::collections::HashMap;

use std::time::Duration;

use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpCheck {
    pub method: HttpMethod,

    pub is_insecure: bool,

    pub url: String,
    pub timeout: u64,
    pub headers: Option<HashMap<String, String>>,
}

impl HttpCheck {
    pub fn set_http_settings(
        &mut self,
        method: HttpMethod,
        url: String,
        timeout: u64,
        headers: Option<HashMap<String, String>>,
    ) {
        self.method = method;
        self.url = url;
        self.timeout = timeout;
        self.headers = headers;
    }

    pub fn method_as_str(&self) -> &str {
        self.method.as_str()
    }

    pub async fn exec(&self) -> Result<()> {
        let cl = reqwest::Client::builder()
            .danger_accept_invalid_certs(self.is_insecure)
            .danger_accept_invalid_hostnames(self.is_insecure)
            .build()?;

        let mut req = match self.method {
            HttpMethod::GET => cl.get(&self.url),
            HttpMethod::POST => cl.post(&self.url),
            HttpMethod::PUT => cl.put(&self.url),
            HttpMethod::DELETE => cl.delete(&self.url),
            HttpMethod::PATCH => cl.patch(&self.url),
        };

        req = req.timeout(Duration::from_secs(self.timeout));

        // We need to merge custom headers.
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

                Ok(())
            }
            Err(e) => {
                if e.is_status() {
                    return Err(anyhow!(
                        "HTTP Request failed due to invalid status code: {}",
                        e.status().unwrap()
                    ));
                } else if e.is_timeout() {
                    return Err(anyhow!("HTTP Request timed out ({} secs)", self.timeout));
                } else {
                    return Err(anyhow!("HTTP Request failed: {}", e));
                }
            }
        }
    }
}
