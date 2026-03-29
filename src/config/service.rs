use crate::config::Alert;
use crate::config::Check;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Service {
    pub name: String,

    pub check: Check,

    pub alert_pass: Option<Alert>,
    pub alert_fail: Option<Alert>,

    #[serde(default = "Service::def_fails_cnt_to_alert")]
    pub fails_cnt_to_alert: Option<u32>,

    #[serde(default = "Service::def_lats_max_track")]
    pub lats_max_track: Option<u32>,
}

impl Service {
    fn def_fails_cnt_to_alert() -> Option<u32> {
        Some(3)
    }

    fn def_lats_max_track() -> Option<u32> {
        Some(10)
    }
}
