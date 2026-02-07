use crate::check::http::HttpCheck;

use anyhow::Result;

#[derive(Debug, Clone)]
pub enum CheckType {
    Http(HttpCheck),
}

#[derive(Debug, Clone)]
pub struct Check {
    pub cron: String,

    pub check_type: CheckType,

    pub fails_tot: u32,
    pub fails_cur: u32,
}

impl Check {
    pub fn new(cron: String, check_type: CheckType) -> Self {
        Check {
            cron,

            check_type,

            fails_tot: 0,
            fails_cur: 0,
        }
    }

    pub async fn exec(&self) -> Result<()> {
        let check_type = self.check_type.clone();

        match check_type {
            CheckType::Http(http_check) => return http_check.exec().await,
        };
    }
}
