// tests/integration_tests.rs

// dependencies
use crate::helpers::{make_request, make_request_with_headers, make_request_with_method_and_headers, start_test_server};
use flux_web_lib::{App, AppRequest, AppResponse};
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use std::collections::HashMap;

// module declarations
mod helpers;

#[tokio::test]
async fn test_basic_get_route() {
    let mut app = App::new();

    app.get("/", |_req: &AppRequest| {
        AppResponse::new(200, "Hello World!").with_header("Content-Type", "text/plain")
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

    app.get("/home", |_req: &AppRequest| {
        AppResponse::new(200, "Home Page").with_header("Content-Type", "text/plain")
    })
    .get("/about", |_req: &AppRequest| {
        AppResponse::new(200, "About Page").with_header("Content-Type", "text/plain")
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

    app.get("/exists", |_req: &AppRequest| {
        AppResponse::new(200, "Hello, world!").with_header("Content-Type", "text/plain")
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

    app.get("/created", |_req: &AppRequest| {
        AppResponse::new(201, "Resource created").with_header("Content-Type", "text/plain")
    })
    .get("/error", |_req: &AppRequest| {
        AppResponse::new(500, "Internal Server Error").with_header("Content-Type", "text/plain")
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

    app.get("/echo", |req: &AppRequest| {
        AppResponse::new(200, format!("You requested: {}", req.path))
            .with_header("Content-Type", "text/plain")
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

    app.get("/resource", |_req: &AppRequest| {
        AppResponse::new(200, "GET").with_header("Content-Type", "text/plain")
    })
    .post("/resource", |_req: &AppRequest| {
        AppResponse::new(200, "POST").with_header("Content-Type", "text/plain")
    })
    .put("/resource", |_req: &AppRequest| {
        AppResponse::new(200, "PUT").with_header("Content-Type", "text/plain")
    })
    .patch("/resource", |_req: &AppRequest| {
        AppResponse::new(200, "PATCH").with_header("Content-Type", "text/plain")
    })
    .delete("/resource", |_req: &AppRequest| {
        AppResponse::new(200, "DELETE").with_header("Content-Type", "text/plain")
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

    app.get("/concurrent", |_req: &AppRequest| {
        AppResponse::new(200, "Concurrent response".to_string())
            .with_header("Content-Type", "text/plain")
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

// ===== HEADER FUNCTIONALITY TESTS =====

#[tokio::test]
async fn test_request_headers_are_accessible() {
    let mut app = App::new();

    app.get("/headers", |req: &AppRequest| {
        let user_agent = req.headers.get("user-agent")
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        let custom_header = req.headers.get("x-custom-header")
            .map(|s| s.as_str())
            .unwrap_or("Not found");

        AppResponse::new(200, format!("User-Agent: {}, Custom: {}", user_agent, custom_header))
            .with_header("Content-Type", "text/plain")
    });

    start_test_server(8008, app).await;

    let mut headers = HashMap::new();
    headers.insert("user-agent", "Flux-Web-Test/1.0");
    headers.insert("x-custom-header", "test-value");

    let (status, body, _response_headers) = make_request_with_headers("http://127.0.0.1:8008/headers", headers)
        .await
        .expect("Request failed");

    assert_eq!(status, 200);
    assert_eq!(body, "User-Agent: Flux-Web-Test/1.0, Custom: test-value");
}

#[tokio::test]
async fn test_response_headers_are_set() {
    let mut app = App::new();

    app.get("/api/data", |_req: &AppRequest| {
        AppResponse::new(200, r#"{"message": "Hello, API!"}"#)
            .with_header("Content-Type", "application/json")
            .with_header("Access-Control-Allow-Origin", "*")
            .with_header("Cache-Control", "no-cache")
            .with_header("X-API-Version", "1.0")
    });

    start_test_server(8009, app).await;

    let (status, body, response_headers) = make_request_with_headers("http://127.0.0.1:8009/api/data", HashMap::new())
        .await
        .expect("Request failed");

    assert_eq!(status, 200);
    assert_eq!(body, r#"{"message": "Hello, API!"}"#);

    // Check response headers
    assert_eq!(response_headers.get("content-type"), Some(&"application/json".to_string()));
    assert_eq!(response_headers.get("access-control-allow-origin"), Some(&"*".to_string()));
    assert_eq!(response_headers.get("cache-control"), Some(&"no-cache".to_string()));
    assert_eq!(response_headers.get("x-api-version"), Some(&"1.0".to_string()));
}

#[tokio::test]
async fn test_multiple_headers_chaining() {
    let mut app = App::new();

    app.post("/upload", |_req: &AppRequest| {
        AppResponse::new(201, "File uploaded successfully")
            .with_header("Content-Type", "text/plain")
            .with_header("Location", "/files/123")
            .with_header("X-Upload-Status", "completed")
            .with_header("X-File-Size", "1024")
            .with_header("X-Processing-Time", "50ms")
    });

    start_test_server(8010, app).await;

    let (status, body, response_headers) = make_request_with_method_and_headers(
        "http://127.0.0.1:8010/upload",
        "POST",
        HashMap::new()
    )
        .await
        .expect("Request failed");

    assert_eq!(status, 201);
    assert_eq!(body, "File uploaded successfully");

    // Verify all chained headers are present
    assert_eq!(response_headers.get("content-type"), Some(&"text/plain".to_string()));
    assert_eq!(response_headers.get("location"), Some(&"/files/123".to_string()));
    assert_eq!(response_headers.get("x-upload-status"), Some(&"completed".to_string()));
    assert_eq!(response_headers.get("x-file-size"), Some(&"1024".to_string()));
    assert_eq!(response_headers.get("x-processing-time"), Some(&"50ms".to_string()));
}

#[tokio::test]
async fn test_request_headers_case_insensitive_access() {
    let mut app = App::new();

    app.get("/case-test", |req: &AppRequest| {
        // Headers should be lowercase when stored
        let content_type = req.headers.get("content-type")
            .map(|s| s.as_str())
            .unwrap_or("text/plain");
        let authorization = req.headers.get("authorization")
            .map(|s| s.as_str())
            .unwrap_or("none");

        AppResponse::new(200, format!("Content-Type: {}, Auth: {}", content_type, authorization))
            .with_header("Content-Type", "text/plain")
    });

    start_test_server(8011, app).await;

    let mut headers = HashMap::new();
    headers.insert("Content-Type", "application/json");  // Mixed case
    headers.insert("Authorization", "Bearer token123");   // Mixed case

    let (status, body, _response_headers) = make_request_with_headers("http://127.0.0.1:8011/case-test", headers)
        .await
        .expect("Request failed");

    assert_eq!(status, 200);
    assert_eq!(body, "Content-Type: application/json, Auth: Bearer token123");
}

#[tokio::test]
async fn test_missing_request_headers_handled() {
    let mut app = App::new();

    app.get("/optional-headers", |req: &AppRequest| {
        let optional_header = req.headers.get("x-optional-header")
            .cloned()
            .unwrap_or_else(|| "default-value".to_string());

        AppResponse::new(200, format!("Optional header: {}", optional_header))
            .with_header("Content-Type", "text/plain")
    });

    start_test_server(8012, app).await;

    // Test without the optional header
    let (status, body, _response_headers) = make_request_with_headers("http://127.0.0.1:8012/optional-headers", HashMap::new())
        .await
        .expect("Request failed");

    assert_eq!(status, 200);
    assert_eq!(body, "Optional header: default-value");

    // Test with the optional header
    let mut headers = HashMap::new();
    headers.insert("x-optional-header", "provided-value");

    let (status, body, _response_headers) = make_request_with_headers("http://127.0.0.1:8012/optional-headers", headers)
        .await
        .expect("Request failed");

    assert_eq!(status, 200);
    assert_eq!(body, "Optional header: provided-value");
}

#[tokio::test]
async fn test_404_response_has_default_headers() {
    let mut app = App::new();

    app.get("/exists", |_req: &AppRequest| {
        AppResponse::new(200, "Found").with_header("Content-Type", "text/plain")
    });

    start_test_server(8013, app).await;

    let (status, body, response_headers) = make_request_with_headers("http://127.0.0.1:8013/does-not-exist", HashMap::new())
        .await
        .expect("Request failed");

    assert_eq!(status, 404);
    assert_eq!(body, "Not Found");

    // 404 responses should have default Content-Type header
    assert_eq!(response_headers.get("content-type"), Some(&"text/plain".to_string()));
}

#[tokio::test]
async fn test_header_override_within_response() {
    let mut app = App::new();

    app.get("/override", |_req: &AppRequest| {
        AppResponse::new(200, "Response with overridden header")
            .with_header("Content-Type", "text/plain")
            .with_header("X-Version", "1.0")
            .with_header("Content-Type", "application/json")  // Override previous Content-Type
            .with_header("X-Version", "2.0")  // Override previous X-Version
    });

    start_test_server(8014, app).await;

    let (status, body, response_headers) = make_request_with_headers("http://127.0.0.1:8014/override", HashMap::new())
        .await
        .expect("Request failed");

    assert_eq!(status, 200);
    assert_eq!(body, "Response with overridden header");

    // Last header value should win
    assert_eq!(response_headers.get("content-type"), Some(&"application/json".to_string()));
    assert_eq!(response_headers.get("x-version"), Some(&"2.0".to_string()));
}
