// src/lib/method.rs

// enum type to represent an HTTP method
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

// function which takes a hyper::Method as input and converts it to a flux-web Method
pub fn convert_method(m: &hyper::Method) -> Method {
    match *m {
        hyper::Method::GET => Method::Get,
        hyper::Method::POST => Method::Post,
        hyper::Method::PUT => Method::Put,
        hyper::Method::DELETE => Method::Delete,
        hyper::Method::PATCH => Method::Patch,
        _ => Method::Get,
    }
}
