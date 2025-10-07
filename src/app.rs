// src/app.rs

// dependencies
use crate::handler::Handler;
use crate::method::{Method, convert_method};
use crate::request::AppRequest;
use crate::response::AppResponse;
use crate::router::Router;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;


// struct type to represent an Application, consists of a router
#[derive(Debug)]
pub struct App {
    router: Router,
}

// methods for the App type
impl App {
    pub fn new() -> Self {
        App {
            router: Router { routes: Vec::new() },
        }
    }

    pub fn get(&mut self, path: &str, handler: impl Handler + 'static) -> &mut Self {
        self.router.add_route(Method::Get, path, handler);
        self
    }

    pub fn post(&mut self, path: &str, handler: impl Handler + 'static) -> &mut Self {
        self.router.add_route(Method::Post, path, handler);
        self
    }

    pub fn put(&mut self, path: &str, handler: impl Handler + 'static) -> &mut Self {
        self.router.add_route(Method::Put, path, handler);
        self
    }

    pub fn patch(&mut self, path: &str, handler: impl Handler + 'static) -> &mut Self {
        self.router.add_route(Method::Patch, path, handler);
        self
    }

    pub fn delete(&mut self, path: &str, handler: impl Handler + 'static) -> &mut Self {
        self.router.add_route(Method::Delete, path, handler);
        self
    }

    pub async fn listen(self, port: u16) {
        println!("Server listening on port {}", port);

        let router = Arc::new(self.router);

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .unwrap();

        loop {
            let (socket, _remote_addr) = listener.accept().await.unwrap();
            let router = router.clone();

            tokio::task::spawn(async move {
                let io = TokioIo::new(socket);

                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        io,
                        service_fn(move |req| {
                            handle_request(req, router.clone()) // Pass router
                        }),
                    )
                    .await
                {
                    println!("Error: {:?}", err);
                }
            });
        }
    }
}

// implement the Default trait for the App type
impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

async fn handle_request(
    hyper_req: Request<hyper::body::Incoming>,
    router: Arc<Router>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let (parts, body) = hyper_req.into_parts();

    let method = convert_method(&parts.method);
    let path = parts.uri.path();

    let headers: HashMap<String, String> = parts
        .headers
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect();

    let body_bytes = body.collect().await?.to_bytes().to_vec();

    let app_req = AppRequest {
        method: method.clone(),
        headers,
        path: path.to_string(),
        body: body_bytes,
    };

    let response = if let Some(handler) = router.find_route(&method, path) {
        handler.handle(&app_req)
    } else {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        AppResponse::new(404, "Not Found").with_header("Content-Type", "text/plain")
    };

    let response_builder = response.headers.iter().fold(
        Response::builder().status(response.status),
        |builder, (key, value)| builder.header(key, value),
    );

    let body = response.body.unwrap_or_default();
    let body = Full::new(Bytes::from(body));
    Ok(response_builder.body(body).unwrap())
}
