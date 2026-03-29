use crate::alert::{FileLogAlert, http::HttpAlert};

use anyhow::Result;

#[derive(Clone)]
pub enum AlertType {
    Http(HttpAlert),
    FileLog(FileLogAlert),
}

#[derive(Clone)]
pub struct Alert {
    pub alert_type: AlertType,
}

impl Alert {
    pub async fn exec(&self) -> Result<()> {
        match &self.alert_type {
            AlertType::Http(http_alert) => http_alert.exec().await,
            AlertType::FileLog(file_log_alert) => file_log_alert.exec().await,
        }
    }
}
