use crate::http::request::{method::HttpMethod, request_handler::HttpRequestHandler};

#[derive(Debug)]
pub struct Route {
    method: HttpMethod,
    path: String,
    handler: HttpRequestHandler,
}

impl Route {
    pub fn new(method: HttpMethod, path: &str, handler: HttpRequestHandler) -> Self {
        Self {
            method,
            handler,
            path: String::from(path.trim_start_matches("/")),
        }
    }

    pub fn get_handler(&self) -> HttpRequestHandler {
        self.handler
    }

    pub fn get_method(&self) -> HttpMethod {
        self.method
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }
}
