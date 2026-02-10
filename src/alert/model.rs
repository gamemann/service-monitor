use std::fmt::{Display, Formatter};

use crate::alert::http::HttpAlert;

use anyhow::Result;

#[derive(Debug, Clone)]
pub enum AlertType {
    Http(HttpAlert),
}

impl Display for AlertType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub alert_type: AlertType,
}

impl Alert {
    pub fn new(alert_type: AlertType) -> Self {
        Self { alert_type }
    }

    pub async fn exec(&self) -> Result<()> {
        match &self.alert_type {
            AlertType::Http(http_alert) => http_alert.exec().await,
        }
    }
}
