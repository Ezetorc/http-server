use std::fmt;

use crate::http::{
    content::{body::Body, headers::Headers, version::Version},
    request::{method::Method, request_line::RequestLine},
};

#[derive(Debug)]
pub struct Request {
    version: Version,
    headers: Headers,
    method: Method,
    body: Option<Body>,
    path: String,
}

impl Request {
    pub fn from(request_line: RequestLine, headers: Headers, body: Option<Body>) -> Self {
        Self {
            body,
            headers,
            path: request_line.path,
            method: request_line.method,
            version: request_line.version,
        }
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn split_path(&self) -> (&str, &str) {
        let path = self.path.trim_start_matches('/');

        match path.split_once('/') {
            Some((base, rest)) => (base, rest),
            None => (path, ""),
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[Request]: {} {} {}",
            self.method, self.path, self.version
        )
    }
}
