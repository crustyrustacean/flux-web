// tests/flux_web/helpers.rs

// dependencies
use flux_web_lib::App;
use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

// Helper function to make HTTP requests
pub async fn make_request(
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

// Helper function to make HTTP requests with custom headers
pub async fn make_request_with_headers(
    url: &str,
    headers: HashMap<&str, &str>,
) -> Result<(u16, String, HashMap<String, String>), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::builder(TokioExecutor::new()).build_http();

    let uri: hyper::Uri = url.parse()?;
    let mut req_builder = hyper::Request::builder().uri(uri);

    // Add custom headers
    for (key, value) in headers {
        req_builder = req_builder.header(key, value);
    }

    let req = req_builder.body(Empty::<Bytes>::new())?;

    let res = client.request(req).await?;
    let status = res.status().as_u16();

    // Extract response headers
    let response_headers: HashMap<String, String> = res
        .headers()
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect();

    let body_bytes = res.into_body().collect().await?.to_bytes();
    let body = String::from_utf8(body_bytes.to_vec())?;

    Ok((status, body, response_headers))
}

// Helper function to make HTTP requests with specific method and headers
pub async fn make_request_with_method_and_headers(
    url: &str,
    method: &str,
    headers: HashMap<&str, &str>,
) -> Result<(u16, String, HashMap<String, String>), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::builder(TokioExecutor::new()).build_http();

    let uri: hyper::Uri = url.parse()?;
    let mut req_builder = hyper::Request::builder().method(method).uri(uri);

    // Add custom headers
    for (key, value) in headers {
        req_builder = req_builder.header(key, value);
    }

    let req = req_builder.body(Empty::<Bytes>::new())?;

    let res = client.request(req).await?;
    let status = res.status().as_u16();

    // Extract response headers
    let response_headers: HashMap<String, String> = res
        .headers()
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect();

    let body_bytes = res.into_body().collect().await?.to_bytes();
    let body = String::from_utf8(body_bytes.to_vec())?;

    Ok((status, body, response_headers))
}

// Helper to start server in background
pub async fn start_test_server(port: u16, app: App) {
    tokio::spawn(async move {
        app.listen(port).await;
    });

    // Give server time to start
    sleep(Duration::from_millis(100)).await;
}
