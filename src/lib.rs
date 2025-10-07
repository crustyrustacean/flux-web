// src/lib/lib.rs

// module declarations
mod method;
mod handler;

// public module declarations
pub mod app;
pub mod request;
pub mod response;
pub mod router;

// re-exports
pub use app::*;
pub use request::*;
pub use response::*;
pub use router::*;
