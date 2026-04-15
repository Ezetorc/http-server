use crate::{http::routing::router::Router, server::middleware::middleware::Middleware};

pub struct RouteBuilder<'a> {
    router: &'a mut Router,
    route_index: usize,
}

impl<'a> RouteBuilder<'a> {
    pub fn new(router: &'a mut Router, route_index: usize) -> Self {
        RouteBuilder {
            router,
            route_index,
        }
    }

    pub fn with(self, middleware: Middleware) -> Self {
        let route = &mut self.router.get_mutable_route(self.route_index);

        route.add_middleware(Box::new(middleware));

        self
    }
}
