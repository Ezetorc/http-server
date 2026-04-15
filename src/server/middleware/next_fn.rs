use crate::server::middleware::middleware_result::MiddlewareResult;

pub type NextFn<'a> = &'a mut dyn FnMut() -> MiddlewareResult;
