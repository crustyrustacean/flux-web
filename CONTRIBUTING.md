# Contributing to Flux Web

First off, thanks for your interest in Flux Web! 

## Project Philosophy

Flux Web is primarily a **learning project** aimed at understanding web frameworks, Rust networking, and async programming from the ground up. The goal is to create an Express.js-like experience in Rust with minimal complexity.

While contributions are welcome, please understand that:
- This is not intended for production use
- Design decisions prioritize learning and simplicity over performance or completeness
- The project may evolve in experimental directions
- There's no guarantee of API stability

## Ways to Contribute

### 1. Use It and Share Feedback

The most valuable contribution is **trying Flux Web** and sharing your experience:
- Build something with it
- Report what works and what doesn't
- Share what features you actually need vs. what's theoretical

### 2. Bug Reports

Found a bug? Please open an issue with:
- **Clear description** of the problem
- **Steps to reproduce** the issue
- **Expected behavior** vs. actual behavior
- **Environment** (OS, Rust version)

Example:
```
Title: Server panics on invalid UTF-8 in request path

Description: When sending a request with invalid UTF-8 characters in the path,
the server panics instead of returning 400 Bad Request.

Steps to reproduce:
1. Start server with basic route
2. Send request with invalid UTF-8: curl "http://localhost:8000/%FF%FF"
3. Server panics

Expected: Return 400 Bad Request
Actual: Server crashes
```

### 3. Feature Requests

Before requesting a feature, consider:
- Is this aligned with the Express.js-like simplicity goal?
- Is this something you'd actually use, or just "nice to have"?
- Can it be added without significant complexity?

Open an issue with:
- **Use case** - Why do you need this?
- **Proposed API** - What would it look like?
- **Alternatives** - How do other frameworks handle this?

### 4. Code Contributions

#### Before Starting

1. **Open an issue first** to discuss the change
2. Make sure it aligns with project goals
3. Keep it simple - complexity is the enemy

#### Development Setup

```bash
# Clone the repo
git clone https://github.com/crustyrustacean/flux-web.git
cd flux-web

# Run tests
cargo test

# Run the example
cargo run
```

#### Code Style

- Follow standard Rust conventions (`cargo fmt`)
- Run Clippy and fix warnings (`cargo clippy`)
- Keep functions small and focused
- Comment complex logic, not obvious code
- Prioritize readability over cleverness

#### Testing Requirements

All new features must include tests:
- **Unit tests** for internal logic
- **Integration tests** for public APIs
- Tests should use descriptive names
- Each test should verify one thing

**Header Testing Guidelines:**
- Always set `Content-Type` headers in test responses
- Test both request header access and response header setting
- Use the new `AppResponse::new()` constructor in all tests
- Chain `.with_header()` calls for multiple headers

Example:
```rust
#[tokio::test]
async fn test_query_params_are_parsed() {
    let mut app = App::new();

    app.get("/search", |req: &AppRequest| {
        let query = req.query_param("q").unwrap_or("none");
        AppResponse::new(200, format!("Searching for: {}", query))
            .with_header("Content-Type", "text/plain")
    });

    start_test_server(8010, app).await;

    let (status, body) = make_request("http://127.0.0.1:8010/search?q=rust")
        .await
        .expect("Request failed");

    assert_eq!(status, 200);
    assert_eq!(body, "Searching for: rust");
}
```

**Header Testing Example:**
```rust
#[tokio::test]
async fn test_request_headers_are_accessible() {
    let mut app = App::new();

    app.get("/headers", |req: &AppRequest| {
        let user_agent = req.headers.get("user-agent")
            .unwrap_or(&"Unknown".to_string());

        AppResponse::new(200, format!("User-Agent: {}", user_agent))
            .with_header("Content-Type", "text/plain")
            .with_header("X-Custom-Header", "Flux-Web")
    });

    start_test_server(8011, app).await;

    // Test would include making request with specific headers
    // and verifying both response body and response headers
}
```

#### Pull Request Process

1. **Fork** the repository
2. **Create a branch** with a descriptive name (`feature/query-params`)
3. **Make your changes** with clear, atomic commits
4. **Add tests** that cover your changes
5. **Run the full test suite** (`cargo test`)
6. **Update documentation** if needed
7. **Submit a PR** with:
   - Clear description of what and why
   - Link to related issue
   - Any breaking changes noted

#### Commit Messages

Use clear, descriptive commit messages:

```
Good:
- Add query parameter parsing to AppRequest
- Fix panic when request path contains invalid UTF-8
- Implement middleware support for request/response pipeline
- feat: add request/response header support with AppResponse::new()

Bad:
- Fixed stuff
- Update code
- Changes
```

**Note on API Changes:**
Since this is a learning project, breaking changes are expected. Always:
- Use the new `AppResponse::new()` constructor in examples
- Include headers with `.with_header()` in test responses
- Access request headers via `req.headers.get()`

## Development Priorities

**Recently Completed:**
- ✅ **Request/Response Headers** - Full header support with `AppRequest.headers` and `AppResponse.with_header()`

**Current focus areas (in order):**

1. **Request parsing** - Body, query params, path params
2. **Response helpers** - JSON, redirects, status shortcuts
3. **Middleware** - Request/response pipeline
4. **Static files** - Serving static assets
5. **Templates** - Tera integration
6. **Error handling** - Graceful failures

## What We're NOT Looking For

To keep the project focused:

- ❌ Complex type-level abstractions
- ❌ Performance optimizations at the cost of readability
- ❌ Framework bloat (features most people won't use)
- ❌ Dependencies beyond the core set (Tokio, Hyper, Serde, Tera)
- ❌ Production-readiness features (distributed tracing, metrics, etc.)

## Questions?

Open an issue with the `question` label. No question is too basic!

## Alternative: Fork It

If you want to take Flux Web in a different direction:
- **Fork it!** That's what open source is for
- Experiment freely
- Share what you learn

The MIT license means you can do whatever you want with the code.

## Code of Conduct

Be kind. Be respectful. Assume good intent. We're all learning here.

## Recognition

Contributors will be acknowledged in the README. Every contribution, no matter how small, is appreciated!

---

Thank you for contributing to Flux Web! 🌊