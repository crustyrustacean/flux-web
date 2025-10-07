// src/response.rs

// dependencies
use std::collections::HashMap;

// struct type to represent a flux-web response
pub struct AppResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

// methods for the AppResponse type
impl AppResponse {
    pub fn new(status: u16, body: impl Into<String>) -> Self {
        AppResponse {
            status,
            headers: HashMap::new(),
            body: Some(body.into().into_bytes()),
        }
    }

    pub fn status(code: u16) -> Self {
        AppResponse {
            status: code,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn ok(body: impl Into<String>) -> Self {
        Self::new(200, body)
    }

    pub fn created(body: impl Into<String>) -> Self {
        Self::new(201, body)
    }

    pub fn no_content() -> Self {
        AppResponse {
            status: 204,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn bad_request(body: impl Into<String>) -> Self {
        Self::new(400, body)
    }

    pub fn not_found(body: impl Into<String>) -> Self {
        Self::new(404, body)
    }

    pub fn internal_error(body: impl Into<String>) -> Self {
        Self::new(500, body)
    }

    pub fn with_bytes(status: u16, bytes: Vec<u8>) -> Self {
        AppResponse {
            status,
            headers: HashMap::new(),
            body: Some(bytes),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}
