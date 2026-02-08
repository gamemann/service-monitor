use crate::service::Service;

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    INIT,
    HEALTHY,
    CHECKING,
    UNHEALTHY,
}

impl ServiceStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceStatus::INIT => "Init",
            ServiceStatus::HEALTHY => "Healthy",
            ServiceStatus::CHECKING => "Checking",
            ServiceStatus::UNHEALTHY => "Unhealthy",
        }
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
