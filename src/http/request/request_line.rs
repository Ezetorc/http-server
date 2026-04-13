use crate::http::{content::version::HttpVersion, error::HttpError, request::method::HttpMethod};

pub struct HttpRequestLine {
    pub http_version: HttpVersion,
    pub method: HttpMethod,
    pub path: String,
}

impl TryFrom<String> for HttpRequestLine {
    type Error = HttpError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let method: &str = parts.next().unwrap_or_default();
        let path: &str = parts.next().unwrap_or_default();
        let version: &str = parts.next().unwrap_or_default();

        Ok(Self {
            method: HttpMethod::try_from(method)?,
            path: path.to_string(),
            http_version: HttpVersion::try_from(version)?,
        })
    }
}
