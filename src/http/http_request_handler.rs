use crate::http::{http_request::HttpRequest, http_response::HttpResponse};

pub type HttpRequestHandler = fn(request: HttpRequest) -> HttpResponse;
