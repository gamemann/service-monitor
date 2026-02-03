use crate::alert::error::AlertError;

use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordWebHookBody {
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct DiscordAlert {
    pub webhook_url: String,
    pub timeout: u32,

    pub content_basic: String,
    pub content_raw: Option<String>,
}

impl DiscordAlert {
    pub fn new(
        webhook_url: String,
        timeout: u32,
        content_basic: String,
        content_raw: Option<String>,
    ) -> Self {
        Self {
            webhook_url,
            timeout,
            content_basic,
            content_raw,
        }
    }

    pub async fn exec(&self) -> Result<(), AlertError> {
        let cl = reqwest::Client::new();

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
            Ok(_) => (),

            Err(e) => {
                if e.is_timeout() {
                    return Err(AlertError::new(format!("Request timed out: {}", e)));
                } else {
                    match e.status() {
                        Some(status) => {
                            return Err(AlertError::new(format!(
                                "Request failed with status code: {}",
                                status.as_u16()
                            )));
                        }
                        None => (),
                    }

                    return Err(AlertError::new(format!("Request failed: {}", e)));
                }
            }
        }

        Ok(())
    }
}
