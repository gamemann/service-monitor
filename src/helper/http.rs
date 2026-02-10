use std::fmt;

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

pub const HTTP_OK_CODES: [u16; 7] = [200, 201, 202, 203, 204, 205, 206];

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
        }
    }
}

impl HttpMethod {
    pub fn from_str(method: &str) -> HttpMethod {
        match method.to_lowercase().as_str() {
            "get" => HttpMethod::GET,
            "post" => HttpMethod::POST,
            "put" => HttpMethod::PUT,
            "delete" => HttpMethod::DELETE,
            "patch" => HttpMethod::PATCH,
            _ => HttpMethod::GET,
        }
    }
}
