use crate::service::Service;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    INIT,
    HEALTHY,
    CHECKING,
    UNHEALTHY,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ServiceStatus::INIT => "INIT",
                ServiceStatus::HEALTHY => "HEALTHY",
                ServiceStatus::CHECKING => "CHECKING",
                ServiceStatus::UNHEALTHY => "UNHEALTHY",
            }
        )
    }
}

impl Service {
    pub async fn get_status(&self) -> ServiceStatus {
        self.status.lock().await.clone()
    }

    pub async fn set_status(&mut self, status: ServiceStatus) {
        *self.status.lock().await = status;
    }
}
