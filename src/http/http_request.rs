use std::fmt;

use crate::http::{
    http_body::HttpBody, http_headers::HttpHeaders, http_method::HttpMethod,
    http_request_line::HttpRequestLine, http_version::HttpVersion,
};

#[derive(Debug)]
pub struct HttpRequest {
    http_version: HttpVersion,
    headers: HttpHeaders,
    method: HttpMethod,
    body: Option<HttpBody>,
    path: String,
}

impl HttpRequest {
    pub fn from(
        request_line: HttpRequestLine,
        headers: HttpHeaders,
        body: Option<HttpBody>,
    ) -> Self {
        Self {
            body,
            headers,
            path: request_line.path,
            method: request_line.method,
            http_version: request_line.http_version,
        }
    }
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[HttpRequest]: {} {} {}",
            self.method, self.path, self.http_version
        )
    }
}
