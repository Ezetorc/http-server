use crate::http::{request::request::Request, response::response::Response};

pub type RequestHandler = fn(request: Request) -> Response;
