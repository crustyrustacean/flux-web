// src/bin/main.rs

// dependencies
use flux_web_lib::{App, AppRequest, AppResponse};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/", |_req: &AppRequest| {
        AppResponse::new(200, "Hello, world!").with_header("Content-Type", "text/plain")
    });

    app.listen(8000).await;
}
