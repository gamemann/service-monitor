use std::fmt;

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl From<HttpMethod> for String {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::PUT => "PUT".to_string(),
            HttpMethod::DELETE => "DELETE".to_string(),
            HttpMethod::PATCH => "PATCH".to_string(),
        }
    }
}

impl From<String> for HttpMethod {
    fn from(method: String) -> Self {
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
