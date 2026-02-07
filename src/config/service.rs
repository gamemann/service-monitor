use crate::config::Alert;
use crate::config::Check;

use crate::config::{def_fails_cnt_to_alert, def_lats_max_track};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Service {
    pub name: String,

    pub check: Check,

    pub alert_pass: Option<Alert>,
    pub alert_fail: Option<Alert>,

    #[serde(default = "def_fails_cnt_to_alert")]
    pub fails_cnt_to_alert: Option<u32>,

    #[serde(default = "def_lats_max_track")]
    pub lats_max_track: Option<u32>,
}
