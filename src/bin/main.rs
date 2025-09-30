// src/bin/main.rs

// dependencies
use flux_web_lib::{App, AppRequest, AppResponse};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.get("/", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "Hello!".to_string(),
    });

    app.listen(8000).await;
}
