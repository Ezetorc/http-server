use crate::{
    http::request::request::Request, server::middleware::middleware_result::MiddlewareResult,
};

pub type Middleware = Box<
    dyn Fn(&mut Request, &mut dyn FnMut() -> MiddlewareResult) -> MiddlewareResult + Send + Sync,
>;
