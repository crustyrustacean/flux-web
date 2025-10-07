// src/handler.rs

// dependencies
use crate::request::AppRequest;
use crate::response::AppResponse;

// a trait which enables creation of handlers
pub trait Handler: Send + Sync {
    fn handle(&self, req: &AppRequest) -> AppResponse;
}

// Automatically implement Handler for any closure that:
// - Takes a reference to AppRequest with any lifetime (for<'a>)
// - Returns an AppResponse
// - Is thread-safe (Send + Sync) for use across async tasks
// This allows users to pass closures directly to app.get() without
// manually implementing the Handler trait.
impl<F> Handler for F
where
    F: for<'a> Fn(&'a AppRequest) -> AppResponse + Send + Sync,
{
    fn handle(&self, req: &AppRequest) -> AppResponse {
        self(req)
    }
}