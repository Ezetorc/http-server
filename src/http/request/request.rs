use std::{collections::HashMap, fmt};

use crate::http::{
    content::{body::Body, headers::Headers, version::Version},
    request::{method::Method, request_line::RequestLine},
    response::{response::Response, status::Status},
};

#[derive(Debug)]
pub struct Request {
    body: Option<Body>,
    version: Version,
    headers: Headers,
    method: Method,
    path: String,
    path_parameters: Option<HashMap<String, String>>,
    query_parameters: Option<HashMap<String, String>>,
}

impl Request {
    pub fn from(request_line: RequestLine, headers: Headers, body: Option<Body>) -> Self {
        Self {
            body,
            headers,
            path: request_line.path,
            method: request_line.method,
            version: request_line.version,
            path_parameters: None,
            query_parameters: None,
        }
    }

    pub fn get_query(&self, query: &str) -> Option<String> {
        self.query_parameters
            .as_ref()
            .and_then(|map| map.get(query))
            .cloned()
    }

    pub fn get_parameter(&self, parameter: &str) -> Option<String> {
        self.path_parameters
            .as_ref()
            .and_then(|map| map.get(parameter))
            .cloned()
    }

    pub fn get_parameter_or_error(&self, parameter: &str) -> Result<String, Response> {
        self.path_parameters
            .as_ref()
            .and_then(|map| map.get(parameter))
            .cloned()
            .ok_or(Response::new(Status::BadRequest))
    }

    pub fn get_query_or_error(&self, query: &str) -> Result<String, Response> {
        self.query_parameters
            .as_ref()
            .and_then(|map| map.get(query))
            .cloned()
            .ok_or(Response::new(Status::BadRequest))
    }

    pub fn set_path_parameters(&mut self, new_path_parameters: HashMap<String, String>) {
        if new_path_parameters.len() != 0 {
            self.path_parameters = Some(new_path_parameters)
        }
    }

    pub fn set_query_parameters(&mut self, new_query_parameters: HashMap<String, String>) {
        if new_query_parameters.len() != 0 {
            self.query_parameters = Some(new_query_parameters)
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.set(key, value);
    }

    pub fn get_header(&self, header_name: &str) -> Option<String> {
        self.headers.get(header_name).cloned()
    }

    pub fn get_header_or_error(&self, header_name: &str) -> Result<String, Response> {
        self.headers
            .get(header_name)
            .cloned()
            .ok_or(Response::new(Status::BadRequest))
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn split_path(&self) -> (&str, &str) {
        let path: &str = self.path.trim_start_matches('/').trim_end_matches("/");

        match path.split_once('?') {
            Some((path, queries)) => (path, queries),
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
