use std::fmt::Display;

use std::error::Error;

#[derive(Debug)]
pub struct CheckError {
    message: String,
}

impl Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl CheckError {
    pub fn new(message: String) -> Self {
        CheckError { message }
    }
}

impl Error for CheckError {}