use crate::check::{error::CheckError, http::HttpCheck};
use crate::service::{ServiceState, ServiceStatus, SharedState};

#[derive(Debug, Clone)]
pub enum CheckType {
    Http(HttpCheck),
}

#[derive(Debug, Clone)]
pub struct Check {
    pub name: Option<String>,
    pub cron: String,

    pub check_type: CheckType,
}

impl Check {
    pub fn new(name: Option<String>, cron: String, check_type: CheckType) -> Self {
        Check {
            name,
            cron,
            check_type,
        }
    }

    pub async fn exec(&self, uid: &str, state: SharedState) -> Result<(), CheckError> {
        // We need to set the state to checking.
        ServiceState::set_state(&state, uid, ServiceStatus::CHECKING).await;

        let res = match &self.check_type {
            CheckType::Http(http_check) => http_check.exec(&uid).await,
        };

        let new_status = match res {
            Ok(true) => ServiceStatus::HEALTHY,
            Ok(false) | Err(_) => ServiceStatus::UNHEALTHY,
        };

        ServiceState::set_state(&state, uid, new_status).await;
        Ok(())
    }
}
