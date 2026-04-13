use crate::http::request::{method::Method, request_handler::RequestHandler};

#[derive(Debug)]
pub struct Route {
    method: Method,
    path: String,
    handler: RequestHandler,
}

impl Route {
    pub fn new(method: Method, path: &str, handler: RequestHandler) -> Self {
        Self {
            method,
            handler,
            path: String::from(path.trim_start_matches("/")),
        }
    }

    pub fn handler(&self) -> RequestHandler {
        self.handler
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}
