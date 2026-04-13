use crate::http::{request::request::HttpRequest, response::response::HttpResponse};

pub type HttpRequestHandler = fn(request: HttpRequest) -> HttpResponse;
