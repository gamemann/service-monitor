mod log;

pub use log::{LogLevel, Logger};

pub const DEBUG: LogLevel = LogLevel::DEBUG;
pub const INFO: LogLevel = LogLevel::INFO;
pub const WARN: LogLevel = LogLevel::WARN;
pub const ERROR: LogLevel = LogLevel::ERROR;
