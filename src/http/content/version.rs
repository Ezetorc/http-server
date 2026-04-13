use std::fmt;

use crate::http::error::HttpError;

#[derive(Debug, Clone, Copy)]
pub enum HttpVersion {
    Http11,
    Http2,
    Http3,
}

impl HttpVersion {
    pub fn as_str(&self) -> &str {
        match self {
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http2 => "HTTP/2",
            HttpVersion::Http3 => "HTTP/3",
        }
    }
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version: &str = self.as_str();

        write!(formatter, "{}", version)
    }
}

impl TryFrom<&str> for HttpVersion {
    type Error = HttpError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "http/1.1" => Ok(Self::Http11),
            "http/2" => Ok(Self::Http2),
            "http/3" => Ok(Self::Http3),

            _ => Err(HttpError::InvalidVersion(value.to_string())),
        }
    }
}
