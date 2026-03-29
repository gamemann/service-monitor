use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct FileLogAlert {
    #[serde(default = "FileLogAlert::def_log_path")]
    pub log_path: String,

    #[serde(default = "FileLogAlert::def_log_file_daily")]
    pub log_file_daily: bool,
}

impl FileLogAlert {
    fn def_log_path() -> String {
        "logs/".to_string()
    }

    fn def_log_file_daily() -> bool {
        true
    }
}
