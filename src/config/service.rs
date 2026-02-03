use crate::config::Alert;
use crate::config::Check;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Service {
    pub uid: String,
    pub name: String,
    pub checks: Vec<Check>,
    pub alerts: Option<Vec<Alert>>,
}
