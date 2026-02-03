use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ServiceError {
    pub message: String,
}

impl ServiceError {
    pub fn new(msg: String) -> Self {
        ServiceError { message: msg }
    }
}

impl Error for ServiceError {}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
