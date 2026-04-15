use crate::http::{request::request::Request, response::handler_result::HandlerResult};

pub type RequestHandler = fn(request: Request) -> HandlerResult;
