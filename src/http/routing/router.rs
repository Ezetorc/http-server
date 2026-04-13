use std::fmt;

use crate::http::{
    request::{method::Method, request_handler::RequestHandler},
    routing::route::Route,
};

#[derive(Debug)]
pub struct Router {
    base_path: String,
    routes: Vec<Route>,
}

impl Router {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: String::from(base_path.trim_start_matches("/")),
            routes: Vec::new(),
        }
    }

    pub fn get_handler(&self, method: Method, path: &str) -> Option<RequestHandler> {
        let matched_route = self
            .routes
            .iter()
            .find(|route| route.method() == method && route.path() == path);

        if let Some(route) = matched_route {
            return Some(route.handler());
        }

        None
    }

    pub fn on_get(&mut self, path: &str, handler: RequestHandler) {
        self.routes.push(Route::new(Method::Get, path, handler));
    }

    pub fn base_path(&self) -> String {
        self.base_path.trim_start_matches("/").to_string()
    }
}

impl fmt::Display for Router {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[HttpRouter '{}']", self.base_path)
    }
}
