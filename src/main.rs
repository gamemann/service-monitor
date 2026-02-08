pub mod alert;
pub mod check;
pub mod cli;
pub mod config;
pub mod debugger;
pub mod service;

use cli::{Args, UserInput};
use config::Config;

use alert::{Alert, AlertType, DiscordAlert};
use check::{Check, CheckType};
use debugger::{LogLevel, Logger};
use service::Service;

use std::sync::{Arc, Mutex};

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
    let log_level = match cfg.debug_lvl.clone().unwrap().as_str() {
        "debug" => LogLevel::DEBUG,
        "info" => LogLevel::INFO,
        "warn" => LogLevel::WARN,
        "error" => LogLevel::ERROR,
        _ => LogLevel::INFO,
    };

    // We need to initialize our logger objecr first to pass along.
    let logger = Logger::new(log_level, cfg.log_dir.clone(), args.input);

    // We need to create our cron scheduler now.
    let mut sched = JobScheduler::new().await?;

    // Create our service objects now.
    let services = Arc::new(Mutex::new(Vec::new()));

    // Loop through each service from config.
    for cfg_service in cfg.services.iter() {
        let cfg_check = cfg_service.check.clone();

        // We need to parse the check type from the config before creating the check object.
        let check_type = match cfg_check.check_type {
            config::CheckType::HTTP => {
                let http: config::HttpCheckConfig = cfg_check.clone().http.unwrap();

                CheckType::Http(check::HttpCheck {
                    url: http.url.clone(),
                    method: match http.method.as_str() {
                        "GET" => check::HttpMethod::GET,
                        "POST" => check::HttpMethod::POST,
                        "PUT" => check::HttpMethod::PUT,
                        "DELETE" => check::HttpMethod::DELETE,
                        "PATCH" => check::HttpMethod::PATCH,
                        _ => check::HttpMethod::GET,
                    },
                    headers: http.headers.clone(),
                    timeout: http.timeout.into(),
                    is_insecure: http.is_insecure,
                })
            }
        };

        // Create a new check object.
        let check = Check::new(cfg_check.cron, check_type);

        // Do the same thing as above, but for alerts.
        let mut alert_pass: Option<Alert> = None;

        if let Some(alert_pass_cfg) = cfg_service.alert_pass.clone() {
            alert_pass = Some(Alert {
                alert_type: match alert_pass_cfg.alert_type {
                    config::AlertType::DISCORD => {
                        let discord = alert_pass_cfg.clone().discord.unwrap();

                        AlertType::Discord(DiscordAlert::new(
                            discord.webhook_url.clone(),
                            discord.timeout.into(),
                            discord.is_insecure,
                            discord.content_basic.clone(),
                            discord.content_raw.clone(),
                        ))
                    }
                },
            });
        }

        // Now do same thing for alert fail.
        let mut alert_fail: Option<Alert> = None;

        if let Some(alert_fail_cfg) = cfg_service.alert_fail.clone() {
            alert_fail = Some(Alert {
                alert_type: match alert_fail_cfg.alert_type {
                    config::AlertType::DISCORD => {
                        let discord = alert_fail_cfg.clone().discord.unwrap();

                        AlertType::Discord(DiscordAlert::new(
                            discord.webhook_url.clone(),
                            discord.timeout.into(),
                            discord.is_insecure,
                            discord.content_basic.clone(),
                            discord.content_raw.clone(),
                        ))
                    }
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

        new_service.init_check(&mut sched, &logger).await?;

        let mut services = services.lock().unwrap();
        services.push(new_service);
    }

    sched.shutdown_on_ctrl_c();

    sched.start().await?;

    // We need to create a new UserInput object.
    let mut input = UserInput::new(cfg, services);

    match args.input {
        true => logger.log(
            LogLevel::INFO,
            "Services started. Using input mode. Please input 'quit', 'exit', or 'q' to exit...",
            true,
        ),
        false => logger.log(
            LogLevel::INFO,
            "Services started. Please use CTRL + C to exit...",
            false,
        ),
    }

    let mut cont = true;

    while cont == true {
        tokio::select! {
            _ = async {
                // If we're not in input mode, just sleep.
                if !args.input {
                    std::future::pending::<()>().await;
                }

                match input.parse().await {
                    Ok(keep_cont) => {
                        cont = keep_cont;
                    },
                    Err(e) => {
                        println!("Error: {}", e);

                        cont = false;
                    }
                }
            } => {}
            _ = tokio::signal::ctrl_c() => {
                cont = false;
            }
        }
    }

    println!();

    logger.log(debugger::INFO, "Exiting...", true);

    Ok(())
}
