// src/request.rs

// dependencies
use crate::method::Method;
use std::collections::HashMap;

// struct type to represent a flux-web request
pub struct AppRequest {
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub path: String,
    pub body: Vec<u8>,
}