# Flux Web 🌊

An Express-like web framework for Rust, built from the ground up with simplicity and familiarity in mind.

## Why Flux Web?

If you're coming from Node.js and Express, Flux Web will feel immediately familiar. No complex type systems, no framework magic—just a clean, simple API for building web applications in Rust.

```rust
use flux_web_lib::{App, AppRequest, AppResponse};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/", |_req: &AppRequest| {
        AppResponse::new(200, "Hello, World!")
            .with_header("Content-Type", "text/plain")
    });

    app.listen(8000).await;
}
```

## Features

✅ **Express-like API** - Familiar chainable methods
✅ **Async/await** - Built on Tokio and Hyper for performance
✅ **All HTTP methods** - GET, POST, PUT, PATCH, DELETE
✅ **Request & Response Headers** - Full header support
✅ **Simple routing** - Exact path matching
✅ **Minimal dependencies** - Just Tokio and Hyper for infrastructure
✅ **Type-safe** - Leverage Rust's type system without complexity  

## Installation

Add Flux Web to your `Cargo.toml`:

```toml
[dependencies]
flux-web = { git = "https://github.com/crustyrustacean/flux-web" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use flux_web_lib::{App, AppRequest, AppResponse};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    // Define routes
    app.get("/", |_req: &AppRequest| {
        AppResponse::new(200, "Home Page")
            .with_header("Content-Type", "text/html")
    })
    .get("/about", |_req: &AppRequest| {
        AppResponse::new(200, "About Page")
            .with_header("Content-Type", "text/plain")
    })
    .post("/users", |_req: &AppRequest| {
        AppResponse::new(201, "User created")
            .with_header("Content-Type", "application/json")
    });

    // Start server
    println!("Server running on http://localhost:8000");
    app.listen(8000).await;
}
```

## Usage

### Defining Routes

Flux Web supports all standard HTTP methods:

```rust
app.get("/users", get_users)
   .post("/users", create_user)
   .put("/users/:id", update_user)
   .patch("/users/:id", partial_update)
   .delete("/users/:id", delete_user);
```

### Request Handlers

Handlers are simple functions that take a request and return a response:

```rust
fn hello_handler(req: &AppRequest) -> AppResponse {
    AppResponse::new(200, format!("Hello from {}!", req.path))
        .with_header("Content-Type", "text/plain")
        .with_header("X-Custom-Header", "Flux-Web")
}

app.get("/hello", hello_handler);
```

Or use closures:

```rust
app.get("/inline", |req: &AppRequest| {
    AppResponse::new(200, "Inline handler")
        .with_header("Content-Type", "text/plain")
});
```

### Status Codes

Set any HTTP status code:

```rust
app.get("/created", |_req: &AppRequest| {
    AppResponse::new(201, "Resource created")
        .with_header("Content-Type", "application/json")
        .with_header("Location", "/resource/123")
})
.get("/error", |_req: &AppRequest| {
    AppResponse::new(500, "Internal Server Error")
        .with_header("Content-Type", "text/plain")
});
```

### Working with Headers

**Request Headers:**
Access incoming request headers through the `AppRequest`:

```rust
app.get("/headers", |req: &AppRequest| {
    let user_agent = req.headers.get("user-agent")
        .unwrap_or(&"Unknown".to_string());

    AppResponse::new(200, format!("Your user agent: {}", user_agent))
        .with_header("Content-Type", "text/plain")
});
```

**Response Headers:**
Set response headers using the builder pattern:

```rust
app.get("/api/data", |_req: &AppRequest| {
    AppResponse::new(200, r#"{"message": "Hello, API!"}"#)
        .with_header("Content-Type", "application/json")
        .with_header("Access-Control-Allow-Origin", "*")
        .with_header("Cache-Control", "no-cache")
});
```

**Multiple Headers:**
Chain multiple `.with_header()` calls:

```rust
app.post("/upload", |_req: &AppRequest| {
    AppResponse::new(201, "File uploaded successfully")
        .with_header("Content-Type", "text/plain")
        .with_header("Location", "/files/123")
        .with_header("X-Upload-Status", "completed")
});
```

### 404 Not Found

Unmatched routes automatically return 404 with proper headers:

```rust
// Request to /unknown automatically returns:
// Status: 404
// Headers: Content-Type: text/plain
// Body: "Not Found"
```

## Current Status

**Working:**
- ✅ All HTTP methods (GET, POST, PUT, PATCH, DELETE)
- ✅ Request headers access
- ✅ Response headers support
- ✅ Exact path matching
- ✅ Custom status codes
- ✅ Concurrent request handling
- ✅ Request path access
- ✅ Comprehensive test coverage

**Planned:**
- 🚧 Path parameters (`/users/:id`)
- 🚧 Query string parsing (`?key=value`)
- 🚧 Request body parsing (JSON, form data)
- 🚧 Middleware support
- 🚧 Response helpers (`.json()`, `.redirect()`)
- 🚧 Static file serving
- 🚧 Template rendering (Tera)
- 🚧 Rate limiting

## Architecture

Flux Web is built on:
- **Tokio** - Async runtime
- **Hyper** - HTTP implementation
- **Standard Library** - Everything else

The framework focuses on providing an Express-like developer experience while leveraging Rust's performance and safety.

## Testing

Run the test suite:

```bash
cargo test
```

Integration tests cover:
- Route matching
- HTTP methods
- Request/response headers
- Status codes
- 404 handling
- Concurrent requests

## Contributing

This is a learning project and personal experiment. Feel free to fork and experiment, but note that it's not intended for production use.

## Comparison to Other Frameworks

| Framework | Philosophy |
|-----------|-----------|
| **Axum** | Type-heavy, extractor-focused, total DIY |
| **Actix-Web** | Full-featured, high-performance |
| **Warp** | Filter combinators |
| **Flux Web** | Express-like simplicity |

Flux Web prioritizes familiarity for Express.js developers over Rust-specific abstractions.

## Goals

- ✅ Learn Rust networking and async from the ground up
- ✅ Understand what frameworks like Hyper and Axum abstract away
- ✅ Build something that feels like Express in Rust
- 🚧 Create a framework that's approachable for Node.js developers

## Non-Goals

- Production readiness (use Axum or Actix-Web for that)
- Maximum performance optimization
- Complex type-level abstractions
- Full HTTP specification compliance

## License

MIT License - see LICENSE.txt

## Author

Jeffery D. Mitchell

---

Built with ❤️ and a lot of learning about Rust, Tokio, and web frameworks.