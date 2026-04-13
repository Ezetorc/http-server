use crate::http::{
    content::{body::HttpBody, headers::HttpHeaders, version::HttpVersion},
    response::status::HttpStatus,
};

#[derive(Debug)]
pub struct HttpResponse {
    version: Option<HttpVersion>,
    status: HttpStatus,
    headers: HttpHeaders,
    body: Option<HttpBody>,
}

impl HttpResponse {
    pub fn new(status: HttpStatus) -> Self {
        Self {
            headers: HttpHeaders::new(),
            version: None,
            body: None,
            status,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let version = self
            .version
            .as_ref()
            .map(|v| v.as_str())
            .unwrap_or("HTTP/1.1");

        let mut response = Vec::new();

        response.extend_from_slice(
            format!(
                "{} {} {}\r\n",
                version,
                self.status.code(),
                self.status.reason()
            )
            .as_bytes(),
        );

        let body_bytes = match &self.body {
            Some(body) => body.as_bytes(),
            None => Vec::new(),
        };

        for (key, value) in self.headers.iterate() {
            response.extend_from_slice(format!("{}: {}\r\n", key, value).as_bytes());
        }

        if !body_bytes.is_empty() {
            response
                .extend_from_slice(format!("Content-Length: {}\r\n", body_bytes.len()).as_bytes());
        }

        response.extend_from_slice(b"\r\n");

        response.extend_from_slice(&body_bytes);

        response
    }

    pub fn set_version(&mut self, new_version: HttpVersion) {
        self.version = Some(new_version)
    }

    pub fn with_body(mut self, new_body: HttpBody) -> Self {
        self.body = Some(new_body);
        self
    }

    pub fn with_headers(mut self, new_headers: HttpHeaders) -> Self {
        self.headers = new_headers;
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.add(key, value);
        self
    }
}
