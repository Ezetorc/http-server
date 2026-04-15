use crate::http::response::response::Response;

pub type MiddlewareResult = Result<(), Response>;
