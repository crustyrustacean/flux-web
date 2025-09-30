// tests/integration_tests.rs

use flux_web_lib::{App, AppRequest, AppResponse};
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use std::time::Duration;
use tokio::time::sleep;

// Helper function to make HTTP requests
async fn make_request(
    url: &str,
) -> Result<(u16, String), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::builder(TokioExecutor::new()).build_http();

    let uri: hyper::Uri = url.parse()?;
    let req = hyper::Request::builder()
        .uri(uri)
        .body(Empty::<Bytes>::new())?;

    let res = client.request(req).await?;
    let status = res.status().as_u16();

    let body_bytes = res.into_body().collect().await?.to_bytes();
    let body = String::from_utf8(body_bytes.to_vec())?;

    Ok((status, body))
}

// Helper to start server in background
async fn start_test_server(port: u16, app: App) {
    tokio::spawn(async move {
        app.listen(port).await;
    });

    // Give server time to start
    sleep(Duration::from_millis(100)).await;
}

#[tokio::test]
async fn test_basic_get_route() {
    let mut app = App::new();

    app.get("/", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "Hello World!".to_string(),
    });

    start_test_server(8001, app).await;

    let (status, body) = make_request("http://127.0.0.1:8001/")
        .await
        .expect("Request failed");

    assert_eq!(status, 200);
    assert_eq!(body, "Hello World!");
}

#[tokio::test]
async fn test_multiple_routes() {
    let mut app = App::new();

    app.get("/home", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "Home Page".to_string(),
    })
    .get("/about", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "About Page".to_string(),
    });

    start_test_server(8002, app).await;

    let (status1, body1) = make_request("http://127.0.0.1:8002/home")
        .await
        .expect("Request failed");
    assert_eq!(status1, 200);
    assert_eq!(body1, "Home Page");

    let (status2, body2) = make_request("http://127.0.0.1:8002/about")
        .await
        .expect("Request failed");
    assert_eq!(status2, 200);
    assert_eq!(body2, "About Page");
}

#[tokio::test]
async fn test_not_found() {
    let mut app = App::new();

    app.get("/exists", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "Found".to_string(),
    });

    start_test_server(8003, app).await;

    let (status, body) = make_request("http://127.0.0.1:8003/does-not-exist")
        .await
        .expect("Request failed");

    assert_eq!(status, 404);
    assert_eq!(body, "Not Found");
}

#[tokio::test]
async fn test_different_status_codes() {
    let mut app = App::new();

    app.get("/created", |_req: &AppRequest| AppResponse {
        status: 201,
        body: "Resource created".to_string(),
    })
    .get("/error", |_req: &AppRequest| AppResponse {
        status: 500,
        body: "Internal Server Error".to_string(),
    });

    start_test_server(8004, app).await;

    let (status1, body1) = make_request("http://127.0.0.1:8004/created")
        .await
        .expect("Request failed");
    assert_eq!(status1, 201);
    assert_eq!(body1, "Resource created");

    let (status2, body2) = make_request("http://127.0.0.1:8004/error")
        .await
        .expect("Request failed");
    assert_eq!(status2, 500);
    assert_eq!(body2, "Internal Server Error");
}

#[tokio::test]
async fn test_request_path_available() {
    let mut app = App::new();

    app.get("/echo", |req: &AppRequest| AppResponse {
        status: 200,
        body: format!("You requested: {}", req.path),
    });

    start_test_server(8005, app).await;

    let (status, body) = make_request("http://127.0.0.1:8005/echo")
        .await
        .expect("Request failed");

    assert_eq!(status, 200);
    assert_eq!(body, "You requested: /echo");
}

#[tokio::test]
async fn test_all_http_methods() {
    let mut app = App::new();

    app.get("/resource", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "GET".to_string(),
    })
    .post("/resource", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "POST".to_string(),
    })
    .put("/resource", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "PUT".to_string(),
    })
    .patch("/resource", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "PATCH".to_string(),
    })
    .delete("/resource", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "DELETE".to_string(),
    });

    start_test_server(8006, app).await;

    let client = Client::builder(TokioExecutor::new()).build_http();

    // Test GET
    let get_req = hyper::Request::builder()
        .method("GET")
        .uri("http://127.0.0.1:8006/resource")
        .body(Empty::<Bytes>::new())
        .unwrap();
    let get_res = client.request(get_req).await.unwrap();
    let get_body = String::from_utf8(
        get_res
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec(),
    )
    .unwrap();
    assert_eq!(get_body, "GET");

    // Test POST
    let post_req = hyper::Request::builder()
        .method("POST")
        .uri("http://127.0.0.1:8006/resource")
        .body(Empty::<Bytes>::new())
        .unwrap();
    let post_res = client.request(post_req).await.unwrap();
    let post_body = String::from_utf8(
        post_res
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec(),
    )
    .unwrap();
    assert_eq!(post_body, "POST");

    // Test PUT
    let put_req = hyper::Request::builder()
        .method("PUT")
        .uri("http://127.0.0.1:8006/resource")
        .body(Empty::<Bytes>::new())
        .unwrap();
    let put_res = client.request(put_req).await.unwrap();
    let put_body = String::from_utf8(
        put_res
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec(),
    )
    .unwrap();
    assert_eq!(put_body, "PUT");

    // Test PATCH
    let patch_req = hyper::Request::builder()
        .method("PATCH")
        .uri("http://127.0.0.1:8006/resource")
        .body(Empty::<Bytes>::new())
        .unwrap();
    let patch_res = client.request(patch_req).await.unwrap();
    let patch_body = String::from_utf8(
        patch_res
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec(),
    )
    .unwrap();
    assert_eq!(patch_body, "PATCH");

    // Test DELETE
    let delete_req = hyper::Request::builder()
        .method("DELETE")
        .uri("http://127.0.0.1:8006/resource")
        .body(Empty::<Bytes>::new())
        .unwrap();
    let delete_res = client.request(delete_req).await.unwrap();
    let delete_body = String::from_utf8(
        delete_res
            .into_body()
            .collect()
            .await
            .unwrap()
            .to_bytes()
            .to_vec(),
    )
    .unwrap();
    assert_eq!(delete_body, "DELETE");
}

#[tokio::test]
async fn test_concurrent_requests() {
    let mut app = App::new();

    app.get("/concurrent", |_req: &AppRequest| AppResponse {
        status: 200,
        body: "Concurrent response".to_string(),
    });

    start_test_server(8007, app).await;

    // Spawn multiple concurrent requests
    let handles: Vec<_> = (0..10)
        .map(|_| tokio::spawn(async { make_request("http://127.0.0.1:8007/concurrent").await }))
        .collect();

    // Wait for all requests to complete
    for handle in handles {
        let result = handle.await.unwrap().unwrap();
        assert_eq!(result.0, 200);
        assert_eq!(result.1, "Concurrent response");
    }
}
