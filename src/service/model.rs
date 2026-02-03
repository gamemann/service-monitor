use crate::service::{ServiceError, ServiceState, SharedState};

use crate::alert::Alert;
use crate::check::Check;

use crate::debugger::{LogLevel, Logger};

use tokio_cron_scheduler::JobScheduler;

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus {
    HEALTHY,
    CHECKING,
    UNHEALTHY,
}

impl ServiceStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceStatus::HEALTHY => "Healthy",
            ServiceStatus::CHECKING => "Checking",
            ServiceStatus::UNHEALTHY => "Unhealthy",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Service {
    pub uid: String,
    pub name: String,
    pub status: ServiceStatus,
    pub checks: Vec<Check>,
    pub alerts: Option<Vec<Alert>>,
}

impl Service {
    pub fn new(uid: String, name: String, checks: Vec<Check>, alerts: Option<Vec<Alert>>) -> Self {
        Service {
            uid,
            name,
            status: ServiceStatus::CHECKING,
            checks,
            alerts,
        }
    }

    pub fn get_state(&self) -> ServiceState {
        ServiceState {
            uid: self.uid.clone(),
            status: self.status.clone(),
        }
    }

    pub async fn setup_checks(
        &self,
        sch: &JobScheduler,
        state: &SharedState,
        logger: &Logger,
    ) -> Result<(), ServiceError> {
        for check in &self.checks {
            let logger_clone = logger.clone();

            // Clone state information to avoid mutating it.
            let uid = self.uid.clone();
            let name = self.name.clone();

            let uid4err = uid.clone();
            let name4err = name.clone();

            let check_clone = check.clone();
            let state_clone = state.clone();

            let job =
                tokio_cron_scheduler::Job::new_async(check.cron.as_str(), move |_uuid, _lock| {
                    let logger = logger_clone.clone();

                    let uid = uid.clone();
                    let name = name.clone();

                    let check = check_clone.clone();
                    let state = state_clone.clone();

                    Box::pin(async move {
                        if let Err(e) = check.exec(uid.as_str(), state).await {
                            logger.log(
                                LogLevel::ERROR,
                                format!("Unable to run check for {} (UID: {}): {}", name, uid, e)
                                    .as_str(),
                            )
                        }
                    })
                });

            // Check for error creating job before we actually schedule it.
            if let Err(e) = job {
                logger.log(
                    LogLevel::ERROR,
                    format!(
                        "Unable to create job for {} (UID: {}): {}",
                        name4err, uid4err, e
                    )
                    .as_str(),
                );

                continue;
            }

            if let Err(e) = sch.add(job.unwrap()).await {
                logger.log(
                    LogLevel::ERROR,
                    format!(
                        "Unable to add job for {} (UID: {}): {}",
                        name4err, uid4err, e
                    )
                    .as_str(),
                );

                continue;
            };
        }

        Ok(())
    }
}
