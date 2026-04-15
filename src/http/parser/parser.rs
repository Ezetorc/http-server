use std::collections::HashMap;

use crate::http::{
    content::{body::Body, headers::Headers},
    error::HttpError,
    request::request_line::RequestLine,
};

pub struct Parser;

impl Parser {
    pub fn parse_body(body: &[u8]) -> Result<Option<Body>, HttpError> {
        if body.len() == 0 {
            return Ok(None);
        }

        let http_body: Body = Body::Binary(Vec::from(body));

        Ok(Some(http_body))
    }

    pub fn parse_request_line(request_line: &[u8]) -> Result<RequestLine, HttpError> {
        let request_line_string: String = String::from_utf8(request_line.to_vec())?;
        let http_request_line: RequestLine = RequestLine::try_from(request_line_string)?;

        Ok(http_request_line)
    }

    pub fn parse_headers(headers: &[u8]) -> Result<Headers, HttpError> {
        let headers_string: String = String::from_utf8(headers.to_vec())?;
        let http_headers: Headers = Headers::from(headers_string);

        Ok(http_headers)
    }

    pub fn parse_query_parameters(queries: &str) -> HashMap<String, String> {
        let mut query_parameters: HashMap<String, String> = HashMap::new();
        let segments = queries.split("&");

        for segment in segments {
            if let Some((key, value)) = segment.split_once("=") {
                query_parameters.insert(key.to_string(), value.to_string());
            }
        }

        query_parameters
    }
}
