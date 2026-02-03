use crate::service::model::ServiceStatus;

use std::sync::Arc;
use tokio::sync::RwLock;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ServiceState {
    pub uid: String,
    pub status: ServiceStatus,
}

pub type SharedState = Arc<RwLock<HashMap<String, ServiceState>>>;

impl ServiceState {
    pub async fn set_state(state: &SharedState, uid: &str, status: ServiceStatus) {
        let mut state = state.write().await;

        if let Some(service) = state.get_mut(uid) {
            service.status = status.clone();
        }
    }
}
