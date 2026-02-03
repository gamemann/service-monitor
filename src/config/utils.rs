use crate::config::Config;

pub fn print(cfg: &Config) {
    println!("Listing settings...");

    println!("Debug level: {}", cfg.debug_lvl);

    let log_dir: String;

    match &cfg.log_dir {
        Some(dir) => log_dir = dir.clone(),
        None => log_dir = String::from("N/A"),
    }

    println!("Log directory: {}", log_dir);

    if cfg.services.len() > 0 {
        println!("Services:");
        for service in &cfg.services {
            println!("\t{} (UID => {})", service.name, service.uid);

            let checks = &service.checks;

            if checks.len() > 0 {
                println!("\t\tChecks:");

                for (idx, check) in checks.iter().enumerate() {
                    let check_name = match &check.name {
                        Some(name) => name.clone(),
                        None => String::from(format!("#{}", idx + 1)),
                    };

                    println!("\t\t\tCheck {}:", check_name);
                    println!("\t\t\t\tCron: {}", check.cron);
                    println!("\t\t\t\tType: {}", check.check_type);

                    // If we have web check settings, print them.
                    if let Some(http) = &check.http {
                        println!("\t\t\t\tHTTP Check:");
                        println!("\t\t\t\t\tMethod: {}", http.method);
                        println!("\t\t\t\t\tUrl: {}", http.url);

                        // If we have headers, map and print them as well.
                        if let Some(headers) = &http.headers {
                            println!("\t\t\t\t\tHeaders:");
                            for (key, val) in headers {
                                println!("\t\t\t\t\t\t{}: {}", key, val);
                            }
                        }
                    }
                }
            }

            if let Some(alerts) = &service.alerts {
                println!("\t\tAlerts:");

                for (idx, alert) in alerts.iter().enumerate() {
                    println!("\t\t\tAlert #{}: {}", idx + 1, alert);

                    println!("\t\t\t\tType: {}", alert.alert_type);

                    if let Some(discord) = &alert.discord {
                        println!("\t\t\t\tDiscord Settings:");

                        println!("\t\t\t\t\tWebhook URL: {}", discord.webhook_url);
                        println!("\t\t\t\t\tTimeout: {}", discord.timeout);
                        println!("\t\t\t\t\tContent Basic: {}", discord.content_basic);

                        println!(
                            "\t\t\t\t\tContent Raw: {}",
                            match &discord.content_raw {
                                Some(contents) => contents.clone(),
                                None => String::from("N/A"),
                            }
                        );
                    }
                }
            }
        }
    }
}
