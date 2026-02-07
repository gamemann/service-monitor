use crate::service::ServiceStatus;

use crate::alert::Alert;
use crate::check::Check;

use crate::debugger::{LogLevel, Logger};

use tokio_cron_scheduler::{Job, JobScheduler};

use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::time::Instant;

use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct Service {
    pub status: ServiceStatus,

    pub name: String,

    pub lats_max_track: u32,
    pub lats: Vec<u32>,

    pub fails_cnt_to_alert: u32,

    pub check: Check,

    pub alert_pass: Option<Alert>,
    pub alert_fail: Option<Alert>,
}

impl Service {
    pub fn new(
        name: String,
        check: Check,
        alert_pass: Option<Alert>,
        alert_fail: Option<Alert>,
        fails_cnt_to_alert: Option<u32>,
        lats_max_track: Option<u32>,
    ) -> Self {
        Service {
            status: ServiceStatus::INIT,

            name,

            lats_max_track: lats_max_track.unwrap_or(10),
            lats: Vec::new(),

            check,

            alert_pass,
            alert_fail,

            fails_cnt_to_alert: fails_cnt_to_alert.unwrap_or(3),
        }
    }

    pub fn lat_min(&self) -> Option<u32> {
        self.lats.iter().min().copied()
    }

    pub fn lat_max(&self) -> Option<u32> {
        self.lats.iter().max().copied()
    }

    pub fn lat_avg(&self) -> Option<u32> {
        match self.lats.len() {
            0 => None,
            len => Some(self.lats.iter().sum::<u32>() / len as u32),
        }
    }

    pub fn lat_last(&self) -> Option<u32> {
        match self.lats.len() {
            0 => None,
            _ => Some(self.lats.last().copied().unwrap_or(0) as u32),
        }
    }

    pub async fn init_check(&mut self, sch: &mut JobScheduler, logger: &Logger) -> Result<()> {
        // Clone basic vars.
        let logger: Logger = logger.clone();
        let name = self.name.clone();

        let fails_cnt_to_alert = self.fails_cnt_to_alert;
        let lats_max_track = self.lats_max_track;
        let cron: String = self.check.cron.clone();

        // We need to clone *again* so that we can use these later.
        let name2 = name.clone();

        // Create Arcs
        let status = Arc::new(Mutex::new(self.status.clone()));
        let lats = Arc::new(Mutex::new(self.lats.clone()));

        let check = Arc::new(Mutex::new(self.check.clone()));

        let alert_pass = Arc::new(self.alert_pass.clone());
        let alert_fail = Arc::new(self.alert_fail.clone());

        let job = Job::new_async(cron.as_str(), move |_uuid, _lock| {
            // We need to clone some necessary vars (strings and Arc pointers) here due to Tokio and async.
            let logger = logger.clone();

            let name = name.clone();

            let status = status.clone();
            let lats = lats.clone();

            let check = check.clone();

            let alert_pass = alert_pass.clone();
            let alert_fail = alert_fail.clone();

            Box::pin(async move {
                let mut check = check.lock().await;

                // Set state to checking.
                *status.lock().await = ServiceStatus::CHECKING;

                // Before we run the check, store precise time so we can calulate latency of call.
                let now = Instant::now();

                match check.exec().await {
                    Err(e) => {
                        logger.log(
                            LogLevel::ERROR,
                            format!("Unable to run check for {}: {}", name, e).as_str(),
                        );

                        // We need to check the fails count threshold and alert if needed.
                        if fails_cnt_to_alert > 0
                            && check.fails_tot == fails_cnt_to_alert
                            && let Some(alert) = alert_fail.as_ref()
                        {
                            match alert.exec().await {
                                Ok(_) => (),
                                Err(e) => logger.log(
                                    LogLevel::ERROR,
                                    format!("Unable to run fail alert for {}: {}", name, e)
                                        .as_str(),
                                ),
                            }
                        }

                        // Set state to unhealthy and increment fail counters.
                        *status.lock().await = ServiceStatus::UNHEALTHY;

                        check.fails_cur += 1;
                        check.fails_tot += 1;
                    }
                    Ok(_) => {
                        let mut lats = lats.lock().await;

                        // Calculate latency before anything for precision.
                        let lat = now.elapsed().as_millis() as u32;

                        lats.push(lat);

                        // If we exceed max track, we need to remove oldest entry.
                        if lats_max_track > 0 && lats.len() > lats_max_track as usize {
                            lats.remove(0);
                        }

                        drop(lats);

                        // Set state to healthy.
                        *status.lock().await = ServiceStatus::HEALTHY;

                        // We need to trigger pass alert if enabled and if our current fail count exeeds the fail alert threshold (or 0 if none).
                        if let Some(alert) = alert_pass.as_ref()
                            && ((fails_cnt_to_alert < 1 && check.fails_cur > 0)
                                || (fails_cnt_to_alert > 0
                                    && check.fails_cur >= fails_cnt_to_alert))
                        {
                            match alert.exec().await {
                                Ok(_) => (),
                                Err(e) => logger.log(
                                    LogLevel::ERROR,
                                    format!("Unable to run healthyalert for {}: {}", name, e)
                                        .as_str(),
                                ),
                            }
                        }

                        // Reset fail counter.
                        check.fails_cur = 0;
                    }
                }
            })
        });

        // Check for error creating job before we actually schedule it.
        if let Err(e) = job {
            return Err(anyhow!(
                "Unable to create job for {}: {}",
                name2,
                e.to_string()
            ));
        }

        if let Err(e) = sch.add(job.unwrap()).await {
            return Err(anyhow!("Unable to add job for {}: {}", name2, e));
        };

        Ok(())
    }
}
