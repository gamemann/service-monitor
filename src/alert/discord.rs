use std::time::Duration;

use serde::{Deserialize, Serialize};

use anyhow::{Result, anyhow};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordWebHookBody {
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct DiscordAlert {
    pub webhook_url: String,
    pub timeout: u32,

    pub is_insecure: bool,

    pub content_basic: String,
    pub content_raw: Option<String>,
}

impl DiscordAlert {
    pub fn new(
        webhook_url: String,
        timeout: u32,
        is_insecure: bool,
        content_basic: String,
        content_raw: Option<String>,
    ) -> Self {
        Self {
            webhook_url,
            timeout,
            is_insecure,
            content_basic,
            content_raw,
        }
    }

    pub async fn exec(&self) -> Result<()> {
        let cl = reqwest::Client::builder()
            .danger_accept_invalid_certs(self.is_insecure)
            .danger_accept_invalid_hostnames(self.is_insecure)
            .build()?;

        // We need to decide what payload to send based off of raw contents.
        let body = match &self.content_raw {
            Some(contents) => contents.clone(),
            None => {
                let body = DiscordWebHookBody {
                    content: self.content_basic.clone(),
                };
                serde_json::to_string(&body).unwrap()
            }
        };

        let req = cl
            .post(self.webhook_url.clone())
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(self.timeout.into()))
            .body(body);

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
