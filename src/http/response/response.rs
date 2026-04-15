use crate::http::{
    content::{body::Body, headers::Headers, version::Version},
    response::{content_type::ContentType, status::Status},
};

#[derive(Debug)]
pub struct Response {
    version: Option<Version>,
    status: Status,
    headers: Headers,
    body: Option<Body>,
}

impl Response {
    pub fn new(status: Status) -> Self {
        Self {
            headers: Headers::new(),
            version: None,
            body: None,
            status,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let version = self
            .version
            .as_ref()
            .map(|value| value.as_str())
            .unwrap_or(Version::Http11.as_str());

        let mut response: Vec<u8> = Vec::new();

        response.extend_from_slice(
            format!(
                "{} {} {}\r\n",
                version,
                self.status.code(),
                self.status.reason()
            )
            .as_bytes(),
        );

        let body_bytes: Vec<u8> = match &self.body {
            Some(body) => body.as_bytes(),
            None => Vec::new(),
        };

        for (key, value) in self.headers.iter() {
            response.extend_from_slice(format!("{}: {}\r\n", key, value).as_bytes());
        }

        if !body_bytes.is_empty() {
            response.extend_from_slice(
                format!("{}: {}\r\n", Headers::CONTENT_LENGTH, body_bytes.len()).as_bytes(),
            );
        }

        response.extend_from_slice(b"\r\n");

        response.extend_from_slice(&body_bytes);

        response
    }

    pub fn set_version(&mut self, new_version: Version) {
        self.version = Some(new_version)
    }

    pub fn with_headers(mut self, new_headers: Headers) -> Self {
        self.headers = new_headers;
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.add(key, value);
        self
    }

    fn with_body_and_type(mut self, body: Body, content_type: ContentType) -> Self {
        self.body = Some(body);
        self.headers.set_content_type(content_type);
        self
    }

    fn with_textual(self, content: &str, content_type: ContentType) -> Self {
        self.with_body_and_type(Body::Text(content.to_string()), content_type)
    }

    pub fn with_html(self, content: &str) -> Self {
        self.with_textual(content, ContentType::Html)
    }

    pub fn with_text(self, content: &str) -> Self {
        self.with_textual(content, ContentType::Plain)
    }

    pub fn with_json(self, content: &str) -> Self {
        self.with_textual(content, ContentType::Json)
    }

    pub fn with_xml(self, content: &str) -> Self {
        self.with_textual(content, ContentType::Xml)
    }

    pub fn with_css(self, content: &str) -> Self {
        self.with_textual(content, ContentType::Css)
    }

    pub fn with_javascript(self, content: &str) -> Self {
        self.with_textual(content, ContentType::Javascript)
    }

    pub fn with_csv(self, content: &str) -> Self {
        self.with_textual(content, ContentType::Csv)
    }

    pub fn with_bytes(self, bytes: Vec<u8>, content_type: ContentType) -> Self {
        self.with_body_and_type(Body::Binary(bytes), content_type)
    }

    pub fn with_file(self, bytes: Vec<u8>, content_type: ContentType) -> Self {
        self.with_bytes(bytes, content_type)
    }
}
