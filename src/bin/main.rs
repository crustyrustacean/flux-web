// src/bin/main.rs

// dependencies
use flux_web_lib::{App, AppRequest, AppResponse};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/health_check", |_req: &AppRequest| {
        AppResponse::status(200)
    });

    app.listen(8080).await;
}
