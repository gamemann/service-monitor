use anyhow::{Result, anyhow};

use crate::{
    alert::{Alert, AlertType, FileLogAlert, HttpAlert},
    check::{self, Check, CheckType},
    config,
    context::Context,
    helper::HttpMethod,
    logger::LogLevel,
    service::Service,
};

/// Builds an alert object from the config alert settings (pass or fail).
///
/// # Arguments
/// * `alert_cfg` - The config alert settings.
/// * `service_name` - The name of the service.
///
/// # Returns
/// `Ok(Alert)` on success, or `Err(anyhow::Error)` on failure.
fn build_alert(alert_cfg: config::Alert, service_name: &str) -> Result<Alert> {
    let alert_type = match alert_cfg.alert_type {
        config::AlertType::Http => {
            let opts: config::HttpAlert = alert_cfg.http.ok_or_else(|| {
                anyhow!(
                    "Alert type is HTTP but no HTTP settings provided for service: {}",
                    service_name
                )
            })?;

            AlertType::Http(HttpAlert::new(
                HttpMethod::from_str(opts.method.as_str()),
                opts.url,
                opts.timeout.into(),
                opts.body,
                opts.body_is_file,
                opts.headers,
                opts.is_insecure,
                opts.accept_codes,
            ))
        }
        config::AlertType::FileLog => {
            let opts = alert_cfg.file_log.ok_or_else(|| {
                anyhow!(
                    "Alert type is FileLog but no FileLog settings provided for service: {}",
                    service_name
                )
            })?;

            AlertType::FileLog(FileLogAlert::new(opts.log_path, opts.log_file_daily))
        }
    };

    Ok(Alert { alert_type })
}

/// Sets up services from config file.
///
/// # Arguments
/// * `ctx` - The program's context.
///
/// # Returns
/// `Ok(())` on success, or `Err(anyhow::Error)` on failure.
pub async fn setup_services(ctx: Context) -> Result<()> {
    // Get config.
    let cfg = ctx.cfg.read().await;
    let logger = ctx.logger.read().await;

    let mut services = ctx.services.write().await;
    let mut sched = ctx.sched.write().await;

    logger.log(LogLevel::INFO, "Setting up services...", false);

    // Loop through each service from config.
    for cfg_service in cfg.services.iter() {
        let service = cfg_service.clone();
        let cfg_check = cfg_service.check.clone();

        // We need to parse the check type from the config before creating the check object.
        let check_type = match cfg_check.check_type {
            config::CheckType::HTTP => {
                let http: config::HttpCheckConfig = cfg_check.clone().http.ok_or_else(|| {
                    anyhow!("Check type is HTTP but no HTTP settings provided in config for service: {}", cfg_service.name)
                })?;

                CheckType::Http(check::HttpCheck {
                    method: HttpMethod::from_str(http.method.as_str()),
                    url: http.url.clone(),
                    timeout: http.timeout.into(),

                    body: http.body.clone(),
                    body_is_file: http.body_is_file,

                    headers: http.headers.clone(),
                    is_insecure: http.is_insecure,

                    accept_codes: http.accept_codes,
                })
            }
        };

        // Create check object to pass to service.
        let check = Check::new(cfg_check.cron, check_type);

        // Create alert pass object and convert config over to object.
        let alert_pass = service
            .alert_pass
            .map(|cfg| build_alert(cfg, &service.name))
            .transpose()?;

        // Now do same thing for alert fail.
        let alert_fail = service
            .alert_fail
            .map(|cfg| build_alert(cfg, &service.name))
            .transpose()?;

        // Create a new service object and pass everything we need to self.
        let mut new_service = Service::new(
            service.name.clone(),
            check,
            alert_pass,
            alert_fail,
            service.fails_cnt_to_alert,
            service.lats_max_track,
        );

        // We need to initialize our checks which'll add jobs to the scheduler.
        new_service.init_check(&mut sched, &logger).await?;

        // Add service to list.
        services.push(new_service);
    }

    Ok(())
}
