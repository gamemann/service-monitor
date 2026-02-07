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
