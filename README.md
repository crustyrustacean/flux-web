# Flux Web 🌊

An Express-like web framework for Rust, built from the ground up with simplicity and familiarity in mind.

## Why Flux Web?

If you're coming from Node.js and Express, Flux Web will feel immediately familiar. No complex type systems, no framework magic—just a clean, simple API for building web applications in Rust.

```rust
use flux_web_lib::{App, AppRequest, AppResponse};

#[tokio::main]
async fn main() {
    let mut app = App::new();
    
    app.get("/", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "Hello, World!".to_string(),
    });
    
    app.listen(8000).await;
}
```

## Features

✅ **Express-like API** - Familiar chainable methods  
✅ **Async/await** - Built on Tokio and Hyper for performance  
✅ **All HTTP methods** - GET, POST, PUT, PATCH, DELETE  
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
    app.get("/", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "Home Page".to_string(),
    })
    .get("/about", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "About Page".to_string(),
    })
    .post("/users", |_req: &AppRequest| AppResponse {
        status: 201,
        body: "User created".to_string(),
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
    AppResponse {
        status: 200,
        body: format!("Hello from {}!", req.path),
    }
}

app.get("/hello", hello_handler);
```

Or use closures:

```rust
app.get("/inline", |req: &AppRequest| AppResponse {
    status: 200,
    body: "Inline handler".to_string(),
});
```

### Status Codes

Set any HTTP status code:

```rust
app.get("/created", |_req: &AppRequest| AppResponse {
    status: 201,
    body: "Resource created".to_string(),
})
.get("/error", |_req: &AppRequest| AppResponse {
    status: 500,
    body: "Internal Server Error".to_string(),
});
```

### 404 Not Found

Unmatched routes automatically return 404:

```rust
// Request to /unknown automatically returns:
// Status: 404
// Body: "Not Found"
```

## Current Status

**Working:**
- ✅ All HTTP methods (GET, POST, PUT, PATCH, DELETE)
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