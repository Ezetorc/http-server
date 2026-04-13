use std::fmt;

use crate::http::{
    request::{method::HttpMethod, request_handler::HttpRequestHandler},
    routing::route::Route,
};

#[derive(Debug)]
pub struct HttpRouter {
    base_path: String,
    routes: Vec<Route>,
}

impl HttpRouter {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: String::from(base_path.trim_start_matches("/")),
            routes: Vec::new(),
        }
    }

    pub fn get_matched_handler(
        &self,
        method: HttpMethod,
        path: &str,
    ) -> Option<HttpRequestHandler> {
        let matched_route = self
            .routes
            .iter()
            .find(|route| route.get_method() == method && route.get_path() == path);

        if let Some(route) = matched_route {
            return Some(route.get_handler());
        }

        None
    }

    pub fn on_get(&mut self, path: &str, handler: HttpRequestHandler) {
        self.routes.push(Route::new(HttpMethod::Get, path, handler));
    }

    pub fn get_base_path(&self) -> String {
        self.base_path.trim_start_matches("/").to_string()
    }
}

impl fmt::Display for HttpRouter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[HttpRouter '{}']", self.base_path)
    }
}
