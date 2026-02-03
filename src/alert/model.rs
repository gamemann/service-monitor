use std::fmt::{Display, Formatter};

use crate::alert::error::AlertError;

use crate::alert::discord::DiscordAlert;

#[derive(Debug, Clone)]
pub enum AlertType {
    Discord(DiscordAlert),
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

    pub async fn exec(&self) -> Result<(), AlertError> {
        match &self.alert_type {
            AlertType::Discord(discord_alert) => discord_alert.exec().await,
        }
    }
}
