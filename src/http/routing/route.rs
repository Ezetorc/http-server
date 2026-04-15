use crate::{
    http::request::{method::Method, request::Request, request_handler::RequestHandler},
    server::middleware::{middleware::Middleware, middleware_result::MiddlewareResult},
};

pub struct Route {
    method: Method,
    path: String,
    handler: RequestHandler,
    middlewares: Vec<Box<Middleware>>,
}

impl Route {
    pub fn new(method: Method, path: &str, handler: RequestHandler) -> Self {
        Self {
            method,
            handler,
            path: String::from(path.trim_start_matches("/")),
            middlewares: Vec::new(),
        }
    }

    fn run_middleware(
        &self,
        request: *mut Request,
        middlewares: &[Box<Middleware>],
    ) -> MiddlewareResult {
        if middlewares.is_empty() {
            return Ok(());
        }

        let (current, rest) = middlewares.split_first().unwrap();

        let mut next = {
            let request = request;

            move || self.run_middleware(request, rest)
        };

        let req = unsafe { &mut *request };

        current(req, &mut next)
    }

    pub fn run_middlewares(&self, request: &mut Request) -> MiddlewareResult {
        let request_pointer = request as *mut Request;

        self.run_middleware(request_pointer, &self.middlewares)?;
        Ok(())
    }

    pub fn add_middleware(&mut self, new_middleware: Box<Middleware>) {
        self.middlewares.push(new_middleware);
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
