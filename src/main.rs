mod alert;
mod check;
mod cli;
mod config;
mod context;
mod helper;
mod logger;
mod service;

use cli::{Args, UserInput};
use config::Config;

use logger::{LogLevel, Logger};

use anyhow::{Result, anyhow};
use tokio_cron_scheduler::JobScheduler;

use clap::Parser;

use crate::{context::ContextData, service::setup_services};

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
    let log_level = match cfg
        .debug_lvl
        .clone()
        .unwrap_or_else(|| "info".to_string())
        .as_str()
    {
        "debug" => LogLevel::DEBUG,
        "info" => LogLevel::INFO,
        "warn" => LogLevel::WARN,
        "error" => LogLevel::ERROR,
        _ => LogLevel::INFO,
    };

    // We need to initialize our logger object first to pass along.
    let logger = Logger::new(log_level, cfg.log_dir.clone(), args.input);

    // We need to create our cron scheduler now.
    let sched = JobScheduler::new().await?;

    // We need to create our context object now.
    let ctx = ContextData::new(args, cfg, logger, Vec::new(), sched);

    // Overwrite logger to use context's logger so we can log setup messages.
    let logger = ctx.logger.read().await;

    // Create our service objects now.
    setup_services(ctx.clone())
        .await
        .map_err(|e| anyhow!("Error setting up services: {}", e))?;

    // Start our scheduler and add signal shutdown.
    ctx.sched.write().await.shutdown_on_ctrl_c();

    ctx.sched.write().await.start().await?;

    // We need to create a new UserInput object.
    let mut input = UserInput::new(ctx.clone());

    match ctx.cli_opts.input {
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
                if !ctx.cli_opts.input {
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

    logger.log(LogLevel::INFO, "Exiting...", true);

    Ok(())
}
