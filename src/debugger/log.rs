use chrono::Local;
use std::fmt::Display;

use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self {
            LogLevel::DEBUG => "DEBUG",
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
            LogLevel::ERROR => "ERROR",
        };
        write!(f, "{}", level_str)
    }
}

#[derive(Debug, Clone)]
pub struct Logger {
    pub level: LogLevel,
    pub log_file: Option<String>,
}

impl Logger {
    pub fn new(level: LogLevel, log_file: Option<String>) -> Self {
        Logger { level, log_file }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        if self.should_log(&level) {
            // We need to create date string for log entry.
            let date_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log_msg = format!("[{}] {}", level, message);
            let log_msg_date = format!("[{}] {}", date_str, log_msg);

            match &self.log_file {
                Some(file_path) => {
                    // Append log message to the specified file.

                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(file_path)
                        .expect("Unable to open log file");
                    writeln!(file, "{}", log_msg_date).expect("Unable to write to log file");
                }
                None => {
                    // Print log message to console.
                    println!("{}", log_msg);
                }
            }
        }
    }

    fn should_log(&self, level: &LogLevel) -> bool {
        match (&self.level, level) {
            (LogLevel::DEBUG, _) => true,
            (LogLevel::INFO, LogLevel::INFO)
            | (LogLevel::INFO, LogLevel::WARN)
            | (LogLevel::INFO, LogLevel::ERROR) => true,
            (LogLevel::WARN, LogLevel::WARN) | (LogLevel::WARN, LogLevel::ERROR) => true,
            (LogLevel::ERROR, LogLevel::ERROR) => true,
            _ => false,
        }
    }
}
