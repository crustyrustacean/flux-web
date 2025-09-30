# Changelog

All notable changes to Flux Web will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Request headers support
- Request body parsing (JSON, form data)
- Query parameter parsing
- Path parameters (e.g., `/users/:id`)
- Response helpers (`.json()`, `.redirect()`)
- Middleware support
- Static file serving
- Template rendering (Tera)
- Rate limiting

## [0.1.0] - 2025-09-29

### Added
- Initial Express-like web framework implementation
- `App` struct with builder-style API
- HTTP method support: GET, POST, PUT, PATCH, DELETE
- `Method` enum for type-safe HTTP methods
- `AppRequest` struct for incoming requests
- `AppResponse` struct for outgoing responses
- `Handler` trait with automatic closure implementation
- `Router` struct for route storage and matching
- Exact path matching for routes
- Automatic 404 responses for unmatched routes
- Custom status code support
- Async server built on Tokio and Hyper
- Concurrent request handling via `tokio::spawn`
- Chainable route definition methods
- `.listen(port)` method for starting server
- Comprehensive integration test suite covering:
  - Basic GET routes
  - Multiple routes
  - 404 handling
  - Custom status codes
  - Request path access
  - All HTTP methods
  - Concurrent requests

### Technical Details
- Built on Tokio 1.47 for async runtime
- Uses Hyper 1.7 for HTTP implementation
- Zero framework dependencies (only infrastructure)
- Type-safe routing with compile-time method verification
- Thread-safe handler storage with `Send + Sync` bounds
- Higher-ranked trait bounds for flexible closure lifetimes
- Arc-based router sharing across async tasks

### Project Structure
- `src/lib.rs` - Library root and re-exports
- `src/method.rs` - HTTP method enum and conversion
- `src/router.rs` - Core routing logic and types
- `src/bin/main.rs` - Example server
- `tests/integration_tests.rs` - Integration test suite

### Documentation
- README.md with quick start and examples
- CONTRIBUTING.md with development guidelines
- LICENSE.txt (MIT)
- Comprehensive inline code documentation

### Known Limitations
- No request body parsing yet
- No query parameter support
- No path parameter support (e.g., `/users/:id`)
- No request headers exposed to handlers
- No response headers customization
- Simple exact path matching only
- No middleware support
- Single-threaded within each connection
- Basic error handling (panics on some errors)

## [0.0.0] - 2025-09-27

### Added
- Initial project exploration
- TCP server implementation (blocking)
- HTTP request parsing (basic)
- HTTP response generation (basic)
- Windows FFI for graceful shutdown
- Foundation for async migration

*Note: This version was part of the learning phase and is not published.*