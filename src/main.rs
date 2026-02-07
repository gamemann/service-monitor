pub mod alert;
pub mod check;
pub mod cli;
pub mod config;
pub mod debugger;
pub mod service;

use cli::Args;
use config::Config;

use alert::{Alert, AlertType, DiscordAlert};
use check::{Check, CheckType};
use debugger::{LogLevel, Logger};
use service::Service;

use std::time::Duration;
use tokio::time;

use anyhow::Result;
use tokio_cron_scheduler::JobScheduler;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments so we know what config file to parse, etc.
    let args = Args::parse();

    // We need to load services and such from config.
    let mut cfg = Config::new();

    cfg.load(args.cfg_path.as_str())?;

    // If the list argument is set, print contents of config and exit program successfully.
    if args.list {
        cfg.print();

        return Ok(());
    }

    // We need to create log level enum and parse it from config.
    let log_level = match cfg.debug_lvl.unwrap().as_str() {
        "debug" => LogLevel::DEBUG,
        "info" => LogLevel::INFO,
        "warn" => LogLevel::WARN,
        "error" => LogLevel::ERROR,
        _ => LogLevel::INFO,
    };

    // We need to initialize our logger objecr first to pass along.
    let logger = Logger::new(log_level, cfg.log_dir.clone());

    // We need to create our cron scheduler now.
    let mut sch = JobScheduler::new().await?;

    // Loop through each service from config.
    for cfg_service in cfg.services.iter() {
        let cfg_check = cfg_service.check.clone();

        // We need to parse the check type from the config before creating the check object.
        let check_type = match cfg_check.check_type {
            config::CheckType::HTTP => CheckType::Http(check::HttpCheck {
                url: cfg_check.clone().http.unwrap().url.clone(),
                method: match cfg_check.clone().http.unwrap().method.as_str() {
                    "GET" => check::HttpMethod::GET,
                    "POST" => check::HttpMethod::POST,
                    "PUT" => check::HttpMethod::PUT,
                    "DELETE" => check::HttpMethod::DELETE,
                    "PATCH" => check::HttpMethod::PATCH,
                    _ => check::HttpMethod::GET,
                },
                headers: cfg_check.clone().http.unwrap().headers.clone(),
                timeout: cfg_check.clone().http.unwrap().timeout.into(),
            }),
        };

        // Create a new check object.
        let check = Check::new(cfg_check.cron, check_type);

        // Do the same thing as above, but for alerts.
        let mut alert_pass: Option<Alert> = None;

        if let Some(alert_pass_cfg) = cfg_service.alert_pass.clone() {
            alert_pass = Some(Alert {
                alert_type: match alert_pass_cfg.alert_type {
                    config::AlertType::DISCORD => AlertType::Discord(DiscordAlert::new(
                        alert_pass_cfg.clone().discord.unwrap().webhook_url.clone(),
                        alert_pass_cfg.clone().discord.unwrap().timeout.into(),
                        alert_pass_cfg
                            .clone()
                            .discord
                            .unwrap()
                            .content_basic
                            .clone(),
                        alert_pass_cfg.clone().discord.unwrap().content_raw.clone(),
                    )),
                },
            });
        }

        // Now do same thing for alert fail.
        let mut alert_fail: Option<Alert> = None;

        if let Some(alert_fail_cfg) = cfg_service.alert_fail.clone() {
            alert_fail = Some(Alert {
                alert_type: match alert_fail_cfg.alert_type {
                    config::AlertType::DISCORD => AlertType::Discord(DiscordAlert::new(
                        alert_fail_cfg.clone().discord.unwrap().webhook_url.clone(),
                        alert_fail_cfg.clone().discord.unwrap().timeout.into(),
                        alert_fail_cfg
                            .clone()
                            .discord
                            .unwrap()
                            .content_basic
                            .clone(),
                        alert_fail_cfg.clone().discord.unwrap().content_raw.clone(),
                    )),
                },
            });
        }

        // Create a new service object.
        let mut new_service = Service::new(
            cfg_service.name.clone(),
            check,
            alert_pass,
            alert_fail,
            cfg_service.fails_cnt_to_alert,
            cfg_service.lats_max_track,
        );

        new_service.init_check(&mut sch, &logger).await?;
    }

    sch.shutdown_on_ctrl_c();

    sch.start().await?;

    time::sleep(Duration::from_secs(1)).await;

    logger.log(debugger::INFO, "Exiting...");

    Ok(())
}
