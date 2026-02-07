use crate::config::Config;

impl Config {
    pub fn print(&self) {
        println!("Listing settings...");

        let debug_lvl = match self.debug_lvl.clone() {
            Some(lvl) => lvl,
            None => String::from("N/A"),
        };

        println!("Debug level: {}", debug_lvl);

        let log_dir = match self.log_dir.clone() {
            Some(dir) => dir,
            None => String::from("N/A"),
        };

        println!("Log directory: {}", log_dir);

        if self.services.len() > 0 {
            println!("Services:");
            for service in self.services.iter() {
                println!("\t{}", service.name);

                let fails_cnt_to_alert = match service.fails_cnt_to_alert {
                    Some(cnt) => cnt,
                    None => 0,
                };

                println!("\t\tFails Count To Alert => {}", fails_cnt_to_alert);

                let lats_max_track = match service.lats_max_track {
                    Some(cnt) => cnt,
                    None => 0,
                };

                println!("\t\tLatency Max Track => {}", lats_max_track);

                // We need to list check settings!
                let check = service.check.clone();

                println!("\t\tCheck Settings");

                println!("\t\t\tCron: {}", check.cron);
                println!("\t\t\tType: {}", check.check_type);

                // If we have web check settings, print them.
                if let Some(http) = &check.http {
                    println!("\t\t\tHTTP Settings:");
                    println!("\t\t\t\tMethod: {}", http.method);
                    println!("\t\t\t\tUrl: {}", http.url);

                    // If we have headers, map and print them as well.
                    if let Some(headers) = &http.headers {
                        println!("\t\t\t\tHeaders:");
                        for (key, val) in headers {
                            println!("\t\t\t\t\t{}: {}", key, val);
                        }
                    }
                }

                if let Some(alert) = &service.alert_pass {
                    let alert = alert.clone();

                    println!("\t\tAlert (Success):");

                    println!("\t\t\tType: {}", alert.alert_type);

                    if let Some(discord) = alert.discord {
                        println!("\t\t\tDiscord Settings:");

                        println!("\t\t\t\tWebhook URL: {}", discord.webhook_url);
                        println!("\t\t\t\tTimeout: {}", discord.timeout);
                        println!("\t\t\t\tContent Basic: {}", discord.content_basic);

                        println!(
                            "\t\t\t\tContent Raw: {}",
                            match discord.content_raw {
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
