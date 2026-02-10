use crate::check::Check;
use crate::cli::input::UserInput;

impl UserInput {
    pub async fn list_services(&self) {
        let services = self.services.lock().unwrap();

        if services.len() < 1 {
            println!("No services found...");

            return;
        }

        println!("Listing services...");

        for service in services.iter() {
            println!("\t{}", service.name);

            println!("\t\tStatus => {}", service.get_status().await.to_string());

            println!(
                "\t\tLatency Min => {}ms",
                service.lat_min().await.unwrap_or(0)
            );
            println!(
                "\t\tLatency Max => {}ms",
                service.lat_max().await.unwrap_or(0)
            );
            println!(
                "\t\tLatency Avg => {}ms",
                service.lat_avg().await.unwrap_or(0)
            );
            println!(
                "\t\tLatency Last => {}ms",
                service.lat_last().await.unwrap_or(0)
            );

            let check: Check = service.check.lock().await.clone();

            println!("\t\tCheck Type => {}", check.check_type.to_string());
            println!(
                "\t\tFails Current => {}/{}",
                check.fails_cur, service.fails_cnt_to_alert
            );

            println!("\t\tFails Total => {}", check.fails_tot);
        }
    }
}
