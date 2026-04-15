use std::{collections::HashMap, fmt};

use crate::http::{
    request::{method::Method, request_handler::RequestHandler},
    routing::route::Route,
};

#[derive(Debug)]
pub struct Router {
    path: String,
    routes: Vec<Route>,
}

impl Router {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path.trim_start_matches("/")),
            routes: Vec::new(),
        }
    }

    pub fn get_handler(
        &self,
        method: Method,
        segments: &[&str],
    ) -> Option<(RequestHandler, HashMap<String, String>)> {
        for route in &self.routes {
            if route.method() != method {
                continue;
            }

            let route_segments: Vec<&str> = route.path().split('/').collect();

            if route_segments.len() != segments.len() {
                continue;
            }

            let mut parameters: HashMap<String, String> = HashMap::new();
            let mut matched: bool = true;
            let iterator = route_segments.iter().zip(segments.iter());

            for (route_segment, request_segment) in iterator {
                if route_segment.starts_with(":") {
                    let key = route_segment.trim_start_matches(":");

                    parameters.insert(key.to_string(), request_segment.to_string());
                } else if route_segment != request_segment {
                    matched = false;

                    break;
                }
            }

            if matched {
                return Some((route.handler(), parameters));
            }
        }

        None
    }

    pub fn on_get(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Get, path, handler));
    }

    pub fn on_post(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Post, path, handler));
    }

    pub fn on_put(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Put, path, handler));
    }

    pub fn on_delete(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Delete, path, handler));
    }

    pub fn on_patch(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Patch, path, handler));
    }

    pub fn on_head(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Head, path, handler));
    }

    pub fn on_options(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Options, path, handler));
    }

    pub fn on_connect(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Connect, path, handler));
    }

    pub fn on_trace(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Trace, path, handler));
    }

    pub fn path(&self) -> String {
        self.path.trim_start_matches("/").to_string()
    }
}

impl fmt::Display for Router {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[Router '{}']", self.path)
    }
}
