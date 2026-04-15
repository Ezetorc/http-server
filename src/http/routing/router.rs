use std::{collections::HashMap, fmt};

use crate::http::{
    request::{method::Method, request_handler::RequestHandler},
    routing::{route::Route, route_builder::RouteBuilder},
};

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

    pub fn get_route(
        &self,
        method: Method,
        segments: &[&str],
    ) -> Option<(&Route, HashMap<String, String>)> {
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
                return Some((route, parameters));
            }
        }

        None
    }

    pub fn get_mutable_route(&mut self, index: usize) -> &mut Route {
        &mut self.routes[index]
    }

    pub fn path(&self) -> String {
        self.path.trim_start_matches("/").to_string()
    }

    pub fn on_method<'a>(
        &'a mut self,
        path: &str,
        handler: RequestHandler,
        method: Method,
    ) -> RouteBuilder<'a> {
        let route: Route = Route::new(method, path, handler);
        self.routes.push(route);
        let index: usize = self.routes.len() - 1;

        RouteBuilder::new(self, index)
    }

    pub fn on_get<'a>(&'a mut self, path: &str, handler: RequestHandler) -> RouteBuilder<'a> {
        self.on_method(path, handler, Method::Get)
    }

    pub fn on_post<'a>(&'a mut self, path: &str, handler: RequestHandler) -> RouteBuilder<'a> {
        self.on_method(path, handler, Method::Post)
    }

    pub fn on_put<'a>(&'a mut self, path: &str, handler: RequestHandler) -> RouteBuilder<'a> {
        self.on_method(path, handler, Method::Put)
    }

    pub fn on_delete<'a>(&'a mut self, path: &str, handler: RequestHandler) -> RouteBuilder<'a> {
        self.on_method(path, handler, Method::Delete)
    }

    pub fn on_patch<'a>(&'a mut self, path: &str, handler: RequestHandler) -> RouteBuilder<'a> {
        self.on_method(path, handler, Method::Patch)
    }

    pub fn on_head<'a>(&'a mut self, path: &str, handler: RequestHandler) -> RouteBuilder<'a> {
        self.on_method(path, handler, Method::Head)
    }

    pub fn on_options<'a>(&'a mut self, path: &str, handler: RequestHandler) -> RouteBuilder<'a> {
        self.on_method(path, handler, Method::Options)
    }

    pub fn on_connect<'a>(&'a mut self, path: &str, handler: RequestHandler) -> RouteBuilder<'a> {
        self.on_method(path, handler, Method::Connect)
    }
    pub fn on_trace<'a>(&'a mut self, path: &str, handler: RequestHandler) -> RouteBuilder<'a> {
        self.on_method(path, handler, Method::Trace)
    }
}

impl fmt::Display for Router {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[Router '{}']", self.path)
    }
}
