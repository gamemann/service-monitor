use std::error::Error;

use std::fmt::Display;

#[derive(Debug)]
pub struct AlertError {
    message: String,
}

impl AlertError {
    pub fn new(message: String) -> Self {
        AlertError { message }
    }

    pub fn get(&self) -> &str {
        &self.message
    }
}

impl Error for AlertError {}

impl Display for AlertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
