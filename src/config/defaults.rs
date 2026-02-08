use crate::config::{AlertType, CheckType};

pub fn def_debug_lvl() -> Option<String> {
    Some(String::from("info"))
}

pub fn def_log_dir() -> Option<String> {
    None
}

pub fn def_fails_cnt_to_alert() -> Option<u32> {
    Some(3)
}

pub fn def_lats_max_track() -> Option<u32> {
    Some(10)
}

pub fn def_cron() -> String {
    String::from("0 * * * * *")
}

pub fn def_check_type() -> CheckType {
    CheckType::HTTP
}

pub fn def_check_http_method() -> String {
    String::from("GET")
}

pub fn def_check_http_timeout() -> u32 {
    10
}

pub fn def_check_http_url() -> String {
    String::from("http://127.0.0.1")
}

pub fn def_alert_type() -> AlertType {
    AlertType::DISCORD
}

pub fn def_alert_discord_timeout() -> u32 {
    10
}

pub fn def_alert_discord_content_basic() -> String {
    String::from("Found {s.name} offline!")
}

pub fn def_check_http_is_insecure() -> bool {
    false
}

pub fn def_alert_discord_is_insecure() -> bool {
    false
}
