// src/lib/router.rs

// dependencies
use crate::method::{Method, convert_method};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;

// struct type to represent a route, which consists of a method, path, and handler
struct Route {
    method: Method,
    path: String,
    handler: Box<dyn Handler + Send + Sync>,
}

// implement the Debug trait for the Route type
impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("handler", &"<handler>") // Just print placeholder
            .finish()
    }
}

// struct type which represents a Router, a vector collection of Routes
#[derive(Debug)]
struct Router {
    routes: Vec<Route>,
}

// methods for the Route type
impl Router {
    fn add_route(&mut self, method: Method, path: &str, handler: impl Handler + 'static) {
        self.routes.push(Route {
            method,
            path: path.to_string(),
            handler: Box::new(handler),
        });
    }

    fn find_route(&self, method: &Method, path: &str) -> Option<&(dyn Handler + Send + Sync)> {
        self.routes
            .iter()
            .find(|route| route.method == *method && route.path == path)
            .map(|route| route.handler.as_ref())
    }
}

// struct type to represent a flux-web request
pub struct AppRequest {
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub path: String,
}

// struct type to represent a flux-web response
pub struct AppResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

// methods for the AppResponse type
impl AppResponse {
    pub fn new(status: u16, body: impl Into<String>) -> Self {
        AppResponse {
            status,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}

// a trait which enables creation of handlers
pub trait Handler: Send + Sync {
    fn handle(&self, req: &AppRequest) -> AppResponse;
}

// Automatically implement Handler for any closure that:
// - Takes a reference to AppRequest with any lifetime (for<'a>)
// - Returns an AppResponse
// - Is thread-safe (Send + Sync) for use across async tasks
// This allows users to pass closures directly to app.get() without
// manually implementing the Handler trait.
impl<F> Handler for F
where
    F: for<'a> Fn(&'a AppRequest) -> AppResponse + Send + Sync,
{
    fn handle(&self, req: &AppRequest) -> AppResponse {
        self(req) // Just call the closure
    }
}

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
) -> Result<Response<String>, hyper::Error> {
    let method = convert_method(hyper_req.method());
    let path = hyper_req.uri().path();

    let headers: HashMap<String, String> = hyper_req
        .headers()
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect();

    let app_req = AppRequest {
        method: method.clone(),
        headers,
        path: path.to_string(),
    };

    let response = if let Some(handler) = router.find_route(&method, path) {
        handler.handle(&app_req)
    } else {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        AppResponse {
            status: 404,
            headers,
            body: "Not Found".to_string(),
        }
    };

    let response_builder = response.headers.iter().fold(
        Response::builder().status(response.status),
        |builder, (key, value)| builder.header(key, value),
    );

    Ok(response_builder.body(response.body).unwrap())
}
