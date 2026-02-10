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
    pub status: Arc<Mutex<ServiceStatus>>,

    pub name: String,

    pub lats_max_track: u32,
    pub lats: Arc<Mutex<Vec<u32>>>,

    pub fails_cnt_to_alert: u32,

    pub check: Arc<Mutex<Check>>,

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
            status: Arc::new(Mutex::new(ServiceStatus::INIT)),

            name,

            lats_max_track: lats_max_track.unwrap_or(10),
            lats: Arc::new(Mutex::new(Vec::new())),

            check: Arc::new(Mutex::new(check)),

            alert_pass,
            alert_fail,

            fails_cnt_to_alert: fails_cnt_to_alert.unwrap_or(3),
        }
    }

    pub async fn lat_min(&self) -> Option<u32> {
        self.lats.lock().await.iter().min().copied()
    }

    pub async fn lat_max(&self) -> Option<u32> {
        self.lats.lock().await.iter().max().copied()
    }

    pub async fn lat_avg(&self) -> Option<u32> {
        let lats = self.lats.lock().await;

        match lats.len() {
            0 => None,
            len => Some(lats.iter().sum::<u32>() / len as u32),
        }
    }

    pub async fn lat_last(&self) -> Option<u32> {
        let lats = self.lats.lock().await;

        match lats.len() {
            0 => None,
            _ => Some(lats.last().copied().unwrap_or(0) as u32),
        }
    }

    pub async fn init_check(&mut self, sch: &mut JobScheduler, logger: &Logger) -> Result<()> {
        // Clone basic vars.
        let logger: Logger = logger.clone();
        let name = self.name.clone();

        let fails_cnt_to_alert = self.fails_cnt_to_alert;
        let lats_max_track = self.lats_max_track;

        // Create Arcs
        let status = self.status.clone();
        let lats = self.lats.clone();

        let check = self.check.clone();

        let cron: String = check.lock().await.cron.clone();

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

                // If we're checking, just return.
                let old_status = status.lock().await.clone();

                // Set state to checking.
                *status.lock().await = ServiceStatus::CHECKING;

                // Before we run the check, store precise time so we can calulate latency of call.
                let now = Instant::now();

                match check.exec().await {
                    Err(e) => {
                        let first_time = check.fails_cur == 0;

                        if first_time {
                            logger.log(
                                LogLevel::ERROR,
                                format!("Unable to run check for {}: {}", name, e).as_str(),
                                false,
                            );
                        }

                        // We need to check the fails count threshold and alert if needed.
                        if let Some(alert) = alert_fail.as_ref()
                            && fails_cnt_to_alert > 0
                            && check.fails_cur == fails_cnt_to_alert
                        {
                            match alert.exec().await {
                                Ok(_) => (),
                                Err(e) => logger.log(
                                    LogLevel::ERROR,
                                    format!("Unable to run fail alert for {}: {}", name, e)
                                        .as_str(),
                                    false,
                                ),
                            }
                        }

                        // Set state to unhealthy and increment fail counters.
                        *status.lock().await = ServiceStatus::UNHEALTHY;

                        check.fails_cur += 1;
                        check.fails_tot += 1;
                    }
                    Ok(_) => {
                        // Calculate latency now and push to vector.
                        let mut lats = lats.lock().await;

                        // Calculate latency before anything for precision.
                        let lat = now.elapsed().as_millis() as u32;

                        lats.push(lat);

                        // If we exceed max latency track, we need to remove oldest entry.
                        if lats_max_track > 0 && lats.len() > lats_max_track as usize {
                            lats.remove(0);
                        }

                        // We no longer need to access lats lock.
                        drop(lats);

                        // Quickly set state to healthy.
                        *status.lock().await = ServiceStatus::HEALTHY;

                        // If we weren't healthy before, we need to trigger alerts and check things.
                        if old_status == ServiceStatus::HEALTHY {
                            return;
                        }

                        logger.log(
                            LogLevel::INFO,
                            format!("{} is now healthy!", name).as_str(),
                            false,
                        );

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
                                    false,
                                ),
                            }
                        }

                        // Reset fail counter.
                        check.fails_cur = 0;
                    }
                }
            })
        });

        // We need to clone name again.
        let name = self.name.clone();

        // Check for error creating job before we actually schedule it.
        if let Err(e) = job {
            return Err(anyhow!(
                "Unable to create job for {}: {}",
                name,
                e.to_string()
            ));
        }

        if let Err(e) = sch.add(job.unwrap()).await {
            return Err(anyhow!("Unable to add job for {}: {}", name, e));
        };

        Ok(())
    }
}
