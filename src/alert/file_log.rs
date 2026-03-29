use anyhow::Result;

#[derive(Clone)]
pub struct FileLogAlert {
    pub log_path: String,
    pub log_file_daily: bool,
}

impl FileLogAlert {
    pub fn new(log_path: String, log_file_daily: bool) -> Self {
        Self {
            log_path,
            log_file_daily,
        }
    }

    pub async fn exec(&self) -> Result<()> {
        Ok(())
    }
}
