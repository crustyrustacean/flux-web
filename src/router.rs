// src/lib/router.rs

// dependencies
use crate::handler::Handler;
use crate::method::Method;


// struct type to represent a route, which consists of a method, path, and handler
pub struct Route {
    pub method: Method,
    pub path: String,
    pub handler: Box<dyn Handler + Send + Sync>,
}

// implement the Debug trait for the Route type
impl std::fmt::Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("handler", &"<handler>")
            .finish()
    }
}

// struct type which represents a Router, a vector collection of Routes
#[derive(Debug)]
pub struct Router {
    pub routes: Vec<Route>,
}

// methods for the Route type
impl Router {
    pub fn add_route(&mut self, method: Method, path: &str, handler: impl Handler + 'static) {
        self.routes.push(Route {
            method,
            path: path.to_string(),
            handler: Box::new(handler),
        });
    }

    pub fn find_route(&self, method: &Method, path: &str) -> Option<&(dyn Handler + Send + Sync)> {
        self.routes
            .iter()
            .find(|route| route.method == *method && route.path == path)
            .map(|route| route.handler.as_ref())
    }
}