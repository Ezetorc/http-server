use crate::http::{content::version::Version, error::HttpError, request::method::Method};

pub struct RequestLine {
    pub version: Version,
    pub method: Method,
    pub path: String,
}

impl TryFrom<String> for RequestLine {
    type Error = HttpError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        let method: &str = parts.next().unwrap_or_default();
        let path: &str = parts.next().unwrap_or_default();
        let version: &str = parts.next().unwrap_or_default();

        Ok(Self {
            method: Method::try_from(method)?,
            path: path.to_string(),
            version: Version::try_from(version)?,
        })
    }
}
