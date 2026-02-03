pub mod alert;
pub mod check;
pub mod cli;
pub mod config;
pub mod debugger;
pub mod service;

use cli::load;
use config::Config;
use debugger::LogLevel;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse CLI arguments so we know what config file to parse, etc.
    let args = load::parse();

    // We need to load services and such from config.
    let mut cfg = Config::new();

    Config::load(&mut cfg, &args.cfg_path).expect("Unable to load config");

    // If the list argument is set, print contents of config and exit program successfully.
    if args.list {
        config::print(&cfg);

        return Ok(());
    }

    // We need to create log level enum and parse it from config.
    let log_level = match cfg.debug_lvl.to_lowercase().as_str() {
        "debug" => LogLevel::DEBUG,
        "info" => LogLevel::INFO,
        "warn" => LogLevel::WARN,
        "error" => LogLevel::ERROR,
        _ => LogLevel::INFO,
    };

    // We need to initialize our logger objecr first to pass along.
    let logger = debugger::Logger::new(log_level, cfg.log_dir.clone());

    // Loop through each service from config.
    for service in cfg.services.iter() {
        // We need to parse config checks and convert to proper object vector.
        let mut checks: Vec<check::Check> = Vec::new();

        for check in &service.checks {
            let name = check.name.clone();
            let cron = check.cron.clone();

            if check.check_type == config::CheckType::HTTP
                && let Some(check) = check.http.as_ref()
            {
                let check_type = check::CheckType::Http(check::HttpCheck {
                    url: check.url.clone(),
                    method: match check.method.as_str() {
                        "GET" => check::HttpMethod::GET,
                        "POST" => check::HttpMethod::POST,
                        "PUT" => check::HttpMethod::PUT,
                        "DELETE" => check::HttpMethod::DELETE,
                        "PATCH" => check::HttpMethod::PATCH,
                        _ => check::HttpMethod::GET,
                    },
                    headers: check.headers.clone(),
                    timeout: check.timeout.into(),
                });

                let new_check = check::Check::new(name, cron, check_type);

                checks.push(new_check);
            }
        }

        // Do the same thing as above, but for alerts.
        let mut alerts: Option<Vec<alert::Alert>> = None;

        if let Some(alerts_cfg) = &service.alerts {
            let mut new_alerts: Vec<alert::Alert> = Vec::new();

            for alert in alerts_cfg {
                // Check for Discord alerts.
                if alert.alert_type == config::AlertType::DISCORD
                    && let Some(alert) = alert.discord.as_ref()
                {
                    let alert_type = alert::AlertType::Discord(alert::DiscordAlert {
                        webhook_url: alert.webhook_url.clone(),
                        timeout: alert.timeout.into(),
                        content_basic: alert.content_basic.clone(),
                        content_raw: alert.content_raw.clone(),
                    });

                    let new_alert = alert::Alert::new(alert_type);

                    new_alerts.push(new_alert);
                }
            }

            alerts = Some(new_alerts);
        }

        // Create a new service object.
        let new_service =
            service::Service::new(service.uid.clone(), service.name.clone(), checks, alerts);
    }

    logger.log(debugger::INFO, "Exiting...");

    Ok(())
}
